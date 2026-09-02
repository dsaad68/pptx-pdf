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
}
