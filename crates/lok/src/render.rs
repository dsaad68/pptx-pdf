//! Slide rendering through LOK's tile API.
//!
//! LibreOffice can paint any rectangle of the current part into a caller-supplied pixel
//! buffer at any scale (`paintTile`); Collabora Online draws its browser view this way. A
//! whole slide at a chosen DPI is one tile of the slide's full size, so PNG output comes from
//! the same renderer as the PDF, with no intermediate format. The members involved are behind
//! `LOK_USE_UNSTABLE_API` in the header, and every one is checked against the populated
//! struct size before use.

use std::cell::Cell;
use std::ffi::{CString, c_int, c_long};
use std::io::{BufWriter, Write};
use std::mem::offset_of;
use std::path::{Path, PathBuf};

use crate::document::Document;
use crate::error::Error;
use crate::pdf::PageRange;
use crate::{Result, ffi};

/// Largest accepted `dpi`.
pub(crate) const MAX_DPI: u32 = 1200;
/// Largest raster side in pixels; keeps one slide under about a gigabyte of buffer.
pub(crate) const MAX_SIDE: u64 = 16_384;
/// Twips per inch.
const TWIPS_PER_INCH: i64 = 1440;
/// Settings applied by `initializeForRendering`: the tile painter draws the editing view, so
/// automatic spell checking would add red underlines to the pixels.
const RENDER_ARGUMENTS: &str = r#"{".uno:SpellOnline":{"type":"boolean","value":"false"}}"#;

/// How to render slides with [`Document::render_slides`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct RenderOptions {
    dpi: u32,
    pages: Option<PageRange>,
    hidden: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            dpi: 150,
            pages: None,
            hidden: false,
        }
    }
}

impl RenderOptions {
    /// 150 dpi, every visible slide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolution in pixels per inch of slide (1 to 1200).
    #[must_use]
    pub fn dpi(mut self, dpi: u32) -> Self {
        self.dpi = dpi;
        self
    }

    /// Restrict output to these 1-based slide numbers.
    #[must_use]
    pub fn pages(mut self, pages: Option<PageRange>) -> Self {
        self.pages = pages;
        self
    }

    /// Also render slides marked hidden in the deck (the PDF export skips them).
    #[must_use]
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}

/// One rendered slide: 8-bit RGB, rows top-down, no padding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raster {
    width: u32,
    height: u32,
    rgb: Vec<u8>,
}

impl Raster {
    /// Width in pixels.
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Height in pixels.
    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// `width * height * 3` bytes of R, G, B.
    #[must_use]
    pub fn rgb(&self) -> &[u8] {
        &self.rgb
    }

    /// Encode as PNG.
    ///
    /// # Errors
    ///
    /// [`Error::Png`] if the encoder or the writer fails; `path` only labels the error.
    pub fn write_png<W: Write>(&self, writer: W, path: &Path) -> Result<()> {
        let mut encoder = png::Encoder::new(writer, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let png_err = |source| Error::Png {
            path: path.to_path_buf(),
            source,
        };
        encoder
            .write_header()
            .map_err(png_err)?
            .write_image_data(&self.rgb)
            .map_err(png_err)
    }

    /// Write a PNG file at `path`.
    ///
    /// # Errors
    ///
    /// [`Error::Io`] if the file cannot be created; [`Error::Png`] as for [`Self::write_png`].
    pub fn save_png(&self, path: &Path) -> Result<()> {
        let file = std::fs::File::create(path).map_err(|source| Error::Io {
            context: "creating",
            path: path.to_path_buf(),
            source,
        })?;
        self.write_png(BufWriter::new(file), path)
    }

    /// Convert a LOK tile buffer (4 bytes per pixel) to packed RGB.
    fn from_tile(width: u32, height: u32, tile: &[u8], bgra: bool) -> Self {
        let mut rgb = Vec::with_capacity(tile.len() / 4 * 3);
        for &[a, b, c, _] in tile.as_chunks::<4>().0 {
            if bgra {
                rgb.extend_from_slice(&[c, b, a]);
            } else {
                rgb.extend_from_slice(&[a, b, c]);
            }
        }
        Self { width, height, rgb }
    }
}

/// State the tile API needs on a [`Document`].
#[derive(Debug, Default)]
pub(crate) struct RenderState {
    initialised: Cell<bool>,
}

impl Document<'_> {
    /// Number of slides (parts) in the document.
    ///
    /// # Errors
    ///
    /// [`Error::AbiTooOld`] if the loaded LibreOffice lacks the tile API.
    pub fn parts(&self) -> Result<u32> {
        let f = self.member(
            offset_of!(ffi::LibreOfficeKitDocumentClass, get_parts),
            self.class().get_parts,
            "getParts",
        )?;
        // SAFETY: `ptr` is a live document on the creating thread.
        Ok(u32::try_from(unsafe { f(self.raw()) }).unwrap_or(0))
    }

    /// Whether 0-based `part` is a visible slide. Slides marked hidden in the deck report
    /// `false`; a LibreOffice that cannot say reports `true`.
    ///
    /// # Errors
    ///
    /// [`Error::AbiTooOld`] if the loaded LibreOffice lacks the tile API.
    pub fn part_visible(&self, part: u32) -> Result<bool> {
        let f = self.member(
            offset_of!(ffi::LibreOfficeKitDocumentClass, get_part_info),
            self.class().get_part_info,
            "getPartInfo",
        )?;
        let n = c_int::try_from(part).map_err(|_| Error::BadPart(part))?;
        // SAFETY: `ptr` is live; the returned string is ours to free, done by `take_c_string`.
        let raw = unsafe { f(self.raw(), n) };
        let info = self.office().take_c_string(raw).unwrap_or_default();
        Ok(json_int_field(&info, "visible").is_none_or(|v| v != 0))
    }

    /// Render 0-based `part` at `dpi`.
    ///
    /// # Errors
    ///
    /// [`Error::AbiTooOld`] if the loaded LibreOffice lacks the tile API; [`Error::BadDpi`],
    /// [`Error::BadPart`] or [`Error::RasterTooLarge`] for arguments LibreOffice cannot serve.
    pub fn render_part(&self, part: u32, dpi: u32) -> Result<Raster> {
        if dpi == 0 || dpi > MAX_DPI {
            return Err(Error::BadDpi(dpi));
        }
        let class = self.class();
        let set_part = self.member(
            offset_of!(ffi::LibreOfficeKitDocumentClass, set_part),
            class.set_part,
            "setPart",
        )?;
        let get_size = self.member(
            offset_of!(ffi::LibreOfficeKitDocumentClass, get_document_size),
            class.get_document_size,
            "getDocumentSize",
        )?;
        let paint = self.member(
            offset_of!(ffi::LibreOfficeKitDocumentClass, paint_tile),
            class.paint_tile,
            "paintTile",
        )?;
        let tile_mode = self.member(
            offset_of!(ffi::LibreOfficeKitDocumentClass, get_tile_mode),
            class.get_tile_mode,
            "getTileMode",
        )?;
        self.ensure_rendering()?;

        let n = c_int::try_from(part).map_err(|_| Error::BadPart(part))?;
        if part >= self.parts()? {
            return Err(Error::BadPart(part));
        }
        let (mut w, mut h): (c_long, c_long) = (0, 0);
        // SAFETY: `ptr` is live; the out-pointers are valid for the call.
        unsafe {
            set_part(self.raw(), n);
            get_size(self.raw(), &raw mut w, &raw mut h);
        }
        let (w, h) = (widen(w), widen(h));
        if w <= 0 || h <= 0 {
            return Err(Error::BadPart(part));
        }
        let px = |twips: i64| {
            u64::try_from((twips * i64::from(dpi) + TWIPS_PER_INCH / 2) / TWIPS_PER_INCH)
                .unwrap_or(0)
                .max(1)
        };
        let (width, height) = (px(w), px(h));
        if width > MAX_SIDE || height > MAX_SIDE {
            return Err(Error::RasterTooLarge { width, height });
        }
        let (cw, ch) = (c_int::try_from(width), c_int::try_from(height));
        let (tw, th) = (c_int::try_from(w), c_int::try_from(h));
        let (Ok(cw), Ok(ch), Ok(tw), Ok(th)) = (cw, ch, tw, th) else {
            return Err(Error::RasterTooLarge { width, height });
        };
        let mut tile = vec![
            0u8;
            usize::try_from(width * height * 4)
                .map_err(|_| Error::RasterTooLarge { width, height })?
        ];
        tracing::debug!(part, width, height, dpi, "paintTile");
        // SAFETY: `ptr` is live; `tile` holds exactly `cw * ch` 4-byte pixels, which is the
        // buffer size `paintTile` documents.
        let mode = unsafe {
            paint(self.raw(), tile.as_mut_ptr(), cw, ch, 0, 0, tw, th);
            tile_mode(self.raw())
        };
        let (Ok(width), Ok(height)) = (u32::try_from(width), u32::try_from(height)) else {
            return Err(Error::RasterTooLarge { width, height });
        };
        Ok(Raster::from_tile(
            width,
            height,
            &tile,
            mode != ffi::LOK_TILEMODE_RGBA,
        ))
    }

    /// Render slides into `dir` as `slide-01.png`, `slide-02.png`, ... (numbered by position
    /// in the deck, zero-padded to the deck's slide count) and return the files written.
    /// Hidden slides are skipped unless [`RenderOptions::hidden`]; a page range filters by
    /// slide number.
    ///
    /// # Errors
    ///
    /// As for [`Self::render_part`], plus [`Error::Io`]/[`Error::Png`] for the files and
    /// [`Error::NoParts`] for a document without slides.
    pub fn render_slides(&self, dir: &Path, options: &RenderOptions) -> Result<Vec<PathBuf>> {
        let count = self.parts()?;
        if count == 0 {
            return Err(Error::NoParts);
        }
        let digits = count.to_string().len().max(2);
        let mut written = Vec::new();
        for part in 0..count {
            let number = part + 1;
            if options.pages.as_ref().is_some_and(|r| !r.contains(number)) {
                continue;
            }
            if !options.hidden && !self.part_visible(part)? {
                tracing::debug!(slide = number, "skipping hidden slide");
                continue;
            }
            let raster = self.render_part(part, options.dpi)?;
            let path = dir.join(format!("slide-{number:0digits$}.png"));
            raster.save_png(&path)?;
            written.push(path);
        }
        Ok(written)
    }

    /// Call `initializeForRendering` once per document.
    fn ensure_rendering(&self) -> Result<()> {
        if self.render_state().initialised.get() {
            return Ok(());
        }
        let f = self.member(
            offset_of!(ffi::LibreOfficeKitDocumentClass, initialize_for_rendering),
            self.class().initialize_for_rendering,
            "initializeForRendering",
        )?;
        let args = CString::new(RENDER_ARGUMENTS)
            .map_err(|_| Error::InteriorNul(RENDER_ARGUMENTS.to_owned()))?;
        // SAFETY: `ptr` is live; `args` outlives the call.
        unsafe { f(self.raw(), args.as_ptr()) };
        self.render_state().initialised.set(true);
        Ok(())
    }

    fn member<T>(&self, offset: usize, f: Option<T>, name: &'static str) -> Result<T> {
        if !ffi::has_member(offset, size_of::<usize>(), self.class().n_size) {
            return Err(Error::AbiTooOld(name));
        }
        f.ok_or(Error::AbiTooOld(name))
    }
}

/// `c_long` is 32 bits on some targets and 64 on the ones we build for.
#[expect(
    clippy::useless_conversion,
    reason = "identity on 64-bit targets, widening elsewhere"
)]
fn widen(v: c_long) -> i64 {
    i64::from(v)
}

/// The integer value of `"key": <digits>` in a flat JSON object, without a JSON parser: the
/// strings LOK returns here are its own fixed format. Older LibreOffice builds quote the
/// numbers (`"visible": "0"`), newer ones do not; both are accepted.
fn json_int_field(json: &str, key: &str) -> Option<i64> {
    let needle = format!("\"{key}\"");
    let rest = json.get(json.find(&needle)? + needle.len()..)?;
    let rest = rest.trim_start().strip_prefix(':')?.trim_start();
    let (rest, quoted) = match rest.strip_prefix('"') {
        Some(unquoted) => (unquoted, true),
        None => (rest, false),
    };
    let end = rest
        .char_indices()
        .find(|&(i, c)| !(c.is_ascii_digit() || (i == 0 && c == '-')))
        .map_or(rest.len(), |(i, _)| i);
    if quoted && !rest.get(end..)?.starts_with('"') {
        return None;
    }
    rest.get(..end)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_integer_fields_from_part_info() {
        let info = r#"{ "masterPageCount": 11, "mode": 0, "visible": 1, "selected": 0, "name": "Slide 1" }"#;
        assert_eq!(json_int_field(info, "visible"), Some(1));
        assert_eq!(json_int_field(info, "mode"), Some(0));
        assert_eq!(json_int_field(info, "masterPageCount"), Some(11));
        assert_eq!(json_int_field(info, "name"), None);
        assert_eq!(json_int_field(info, "absent"), None);
        assert_eq!(json_int_field(r#"{"x":-3}"#, "x"), Some(-3));
        // LibreOffice 25.2 (Debian trixie) quotes the numbers; 25.8 does not.
        let old = r#"{ "visible": "0", "selected": "1", "masterPageCount": "11", "mode": "0" }"#;
        assert_eq!(json_int_field(old, "visible"), Some(0));
        assert_eq!(json_int_field(old, "masterPageCount"), Some(11));
        assert_eq!(json_int_field(r#"{"visible": "x"}"#, "visible"), None);
    }

    #[test]
    fn tile_buffers_convert_in_both_pixel_orders() {
        let tile = [1, 2, 3, 255, 4, 5, 6, 255];
        assert_eq!(
            Raster::from_tile(2, 1, &tile, true).rgb(),
            &[3, 2, 1, 6, 5, 4]
        );
        assert_eq!(
            Raster::from_tile(2, 1, &tile, false).rgb(),
            &[1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn png_round_trips_through_the_encoder() {
        let raster = Raster::from_tile(2, 2, &[0u8; 16], true);
        let mut out = Vec::new();
        raster.write_png(&mut out, Path::new("mem.png")).unwrap();
        assert!(out.starts_with(b"\x89PNG\r\n\x1a\n"));
        let reader = png::Decoder::new(std::io::Cursor::new(&out))
            .read_info()
            .unwrap();
        assert_eq!((reader.info().width, reader.info().height), (2, 2));
    }

    #[test]
    fn options_default_to_150_dpi_visible_slides() {
        let o = RenderOptions::new();
        assert_eq!(o.dpi, 150);
        assert!(!o.hidden);
        assert!(o.pages.is_none());
    }
}
