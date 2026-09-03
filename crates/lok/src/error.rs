//! Error type for the `lok` crate.

use std::path::PathBuf;

/// Everything that can go wrong when driving LibreOfficeKit.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Neither `libsofficeapp` nor `libmergedlo` was found in the given directory.
    #[error("no LibreOfficeKit library found in {0}")]
    LibraryNotFound(PathBuf),

    /// The dynamic loader refused the library (missing dependency, wrong architecture, ...).
    #[error("failed to load {path}: {source}")]
    LibraryLoad {
        /// Library that failed to load.
        path: PathBuf,
        /// Loader error.
        #[source]
        source: libloading::Error,
    },

    /// The library loaded but does not export `libreofficekit_hook_2`.
    #[error("{0} does not export libreofficekit_hook_2; LibreOffice too old?")]
    MissingHook(PathBuf),

    /// The hook ran but returned null. LibreOffice prints the reason on stderr.
    #[error("LibreOfficeKit initialisation failed (see stderr for LibreOffice's own message)")]
    Init,

    /// A second [`crate::Office`] was requested in this process.
    #[error("LibreOfficeKit was already initialised in this process; only one Office is allowed")]
    AlreadyInitialised,

    /// The loaded LibreOffice predates a LOK member this crate needs.
    #[error("LibreOfficeKit ABI too old: missing {0}")]
    AbiTooOld(&'static str),

    /// Document load failed. Carries LibreOffice's own error text when it gave one.
    #[error("failed to load document: {0}")]
    DocumentLoad(String),

    /// `saveAs` failed. Carries LibreOffice's own error text when it gave one.
    #[error("failed to save document: {0}")]
    SaveAs(String),

    /// A path or option contained an interior NUL byte and cannot cross the C boundary.
    #[error("string contains an interior NUL byte: {0:?}")]
    InteriorNul(String),

    /// A page range string was not of the form LibreOffice accepts.
    #[error("invalid page range {0:?}: expected e.g. \"1-3,5\"")]
    InvalidPageRange(String),

    /// A rendering call was made on a document that has no parts to render.
    #[error("document has no slides to render")]
    NoParts,

    /// A part index past the end, or a slide size LibreOffice reports as empty.
    #[error("slide {0} does not exist or has no size")]
    BadPart(u32),

    /// The requested resolution is outside the supported range.
    #[error("dpi {0} is out of range (1..={max})", max = crate::render::MAX_DPI)]
    BadDpi(u32),

    /// The raster for a slide would exceed the size cap.
    #[error("slide raster {width}x{height} exceeds the {max} pixel per side limit", max = crate::render::MAX_SIDE)]
    RasterTooLarge {
        /// Requested width in pixels.
        width: u64,
        /// Requested height in pixels.
        height: u64,
    },

    /// A file could not be written.
    #[error("{context} {path}: {source}")]
    Io {
        /// What was being done.
        context: &'static str,
        /// The file involved.
        path: PathBuf,
        /// Underlying error.
        #[source]
        source: std::io::Error,
    },

    /// PNG encoding failed.
    #[error("encoding {path}: {source}")]
    Png {
        /// The file being written.
        path: PathBuf,
        /// Underlying error.
        #[source]
        source: png::EncodingError,
    },
}
