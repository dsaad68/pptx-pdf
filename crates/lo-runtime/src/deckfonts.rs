//! The fonts a presentation needs, read from its OOXML.
//!
//! Every text run in a `.pptx` names its typefaces (`<a:latin typeface="Lato"/>`, plus `a:ea`,
//! `a:cs`, `a:sym`, `a:buFont` for bullets); the theme names the major/minor fonts that
//! `+mj-lt`/`+mn-lt` refer to. Collecting the `typeface` attributes across every XML part under
//! `ppt/` yields the complete set without parsing the schema, and takes a few milliseconds.
//! The text itself tells which scripts need fallback fonts.
//!
//! The result drives the fontconfig filter in [`crate::fontconfig`]: LibreOffice inspects
//! every font it can see at startup (about 1 ms each), so on a machine with hundreds of fonts,
//! seeing only the ones a deck needs cuts its boot from ~1 s to ~0.2 s.

use std::collections::BTreeSet;
use std::io::Read;
use std::path::Path;

use crate::Result;
use crate::error::Error;

/// What a deck needs from the font system: the families it names and, for the scripts its
/// text uses, fontconfig language tags so that fallback fonts covering those scripts stay
/// visible even though no family names them.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct DeckFonts {
    /// Family names referenced by any `typeface` (or ODF font) attribute.
    pub families: BTreeSet<String>,
    /// fontconfig `lang` tags for non-Latin scripts found in the text, e.g. `ja`, `ar`.
    pub langs: BTreeSet<&'static str>,
}

/// Fonts a presentation needs, read from its XML parts.
///
/// Theme placeholders (`+mj-lt` and friends) are skipped; the theme part itself contributes
/// the real names. A document that is not a zip of XML (a binary `.ppt`) yields an empty
/// result; `.odp` is handled like `.pptx`.
///
/// # Errors
///
/// [`Error::Io`] if the file cannot be opened. A file that is not a zip archive is not an
/// error; it simply references nothing.
pub fn scan(deck: &Path) -> Result<DeckFonts> {
    let file = std::fs::File::open(deck).map_err(Error::io("opening deck", deck))?;
    let mut archive = match zip::ZipArchive::new(file) {
        Ok(a) => a,
        Err(_) => return Ok(DeckFonts::default()),
    };
    let mut found = DeckFonts::default();
    let mut buf = String::new();
    for i in 0..archive.len() {
        let Ok(mut entry) = archive.by_index(i) else {
            continue;
        };
        let name = entry.name().to_owned();
        let relevant = name.ends_with(".xml")
            && (name.starts_with("ppt/") || name == "content.xml" || name == "styles.xml");
        if !relevant {
            continue;
        }
        buf.clear();
        if entry.read_to_string(&mut buf).is_err() {
            continue;
        }
        collect(&buf, &mut found.families);
        scripts(&buf, &mut found.langs);
    }
    Ok(found)
}

/// Family names only; see [`scan`].
///
/// # Errors
///
/// As for [`scan`].
pub fn families(deck: &Path) -> Result<BTreeSet<String>> {
    scan(deck).map(|f| f.families)
}

/// Pull every `typeface="…"` / `fo:font-family="…"` / `style:font-name="…"` value out of `xml`.
fn collect(xml: &str, out: &mut BTreeSet<String>) {
    for key in [
        "typeface=\"",
        "fo:font-family=\"",
        "style:font-name=\"",
        "svg:font-family=\"",
    ] {
        let mut rest = xml;
        while let Some(i) = rest.find(key) {
            rest = rest.get(i + key.len()..).unwrap_or_default();
            let Some(end) = rest.find('"') else { break };
            let value = rest.get(..end).unwrap_or_default();
            rest = rest.get(end..).unwrap_or_default();
            let value = unescape(value);
            let value = value.trim().trim_matches('\'');
            if value.is_empty() || value.starts_with('+') {
                continue;
            }
            out.insert(value.to_owned());
        }
    }
}

/// Language tags for the non-Latin scripts whose characters appear in `xml`. Latin, Greek
/// and Cyrillic are covered by the substitute fonts and need no extra pool.
fn scripts(xml: &str, out: &mut BTreeSet<&'static str>) {
    for c in xml.chars() {
        let tag = match c as u32 {
            0x0590..=0x05FF => "he",
            0x0600..=0x06FF | 0x0750..=0x077F => "ar",
            0x0900..=0x097F => "hi",
            0x0980..=0x09FF => "bn",
            0x0B80..=0x0BFF => "ta",
            0x0E00..=0x0E7F => "th",
            0x1100..=0x11FF | 0xAC00..=0xD7AF => "ko",
            0x3040..=0x30FF => "ja",
            0x3400..=0x4DBF | 0x4E00..=0x9FFF => "zh-cn",
            _ => continue,
        };
        out.insert(tag);
    }
    // CJK ideographs are shared; a deck with kana is Japanese, otherwise Chinese fonts do.
    if out.contains("ja") {
        out.remove("zh-cn");
    }
}

fn unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_typefaces_and_skips_theme_placeholders() {
        let xml = r#"<a:latin typeface="Lato"/><a:ea typeface="+mn-ea"/><a:buFont typeface="Wingdings"/>
                     <a:latin typeface="Calibri Light"/><a:cs typeface=""/><a:sym typeface="Symbol"/>"#;
        let mut out = BTreeSet::new();
        collect(xml, &mut out);
        assert_eq!(
            out.into_iter().collect::<Vec<_>>(),
            ["Calibri Light", "Lato", "Symbol", "Wingdings"]
        );
    }

    #[test]
    fn handles_entities_and_odf_attributes() {
        let mut out = BTreeSet::new();
        collect(
            r#"<x fo:font-family="&apos;Noto Sans&apos;" style:font-name="Liberation Serif"/>"#,
            &mut out,
        );
        assert!(out.contains("Noto Sans"));
        assert!(out.contains("Liberation Serif"));
    }

    #[test]
    fn detects_scripts_in_text() {
        let mut langs = BTreeSet::new();
        scripts("<a:t>Ελληνικά Кириллица</a:t>", &mut langs);
        assert!(
            langs.is_empty(),
            "Greek and Cyrillic need no extra pool: {langs:?}"
        );
        scripts("<a:t>日本語テキスト</a:t>", &mut langs);
        assert_eq!(langs.iter().copied().collect::<Vec<_>>(), ["ja"]);
        let mut langs = BTreeSet::new();
        scripts("<a:t>中文 مرحبا</a:t>", &mut langs);
        assert_eq!(langs.iter().copied().collect::<Vec<_>>(), ["ar", "zh-cn"]);
    }

    #[test]
    fn non_zip_input_references_nothing() {
        let path = std::env::temp_dir().join(format!("deckfonts-{}.pptx", std::process::id()));
        std::fs::write(&path, b"not a zip").unwrap();
        assert_eq!(scan(&path).unwrap(), DeckFonts::default());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn reads_the_corpus_deck_when_present() {
        let deck = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tools/lo-tree/corpus/text_fonts.pptx");
        if !deck.is_file() {
            return;
        }
        let fonts = scan(&deck).unwrap();
        for f in [
            "Calibri",
            "Arial",
            "Times New Roman",
            "Cambria",
            "Courier New",
            "Nonexistent Brand Font",
        ] {
            assert!(
                fonts.families.contains(f),
                "{f} missing from {:?}",
                fonts.families
            );
        }
        assert!(
            fonts.langs.contains("ja"),
            "the corpus deck has a Japanese line: {:?}",
            fonts.langs
        );
    }
}
