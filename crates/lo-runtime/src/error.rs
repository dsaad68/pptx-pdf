//! Error type for the `lo-runtime` crate.

use std::path::PathBuf;

/// Everything that can go wrong locating or preparing the runtime.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// No explicit directory, no environment variable, and no embedded tree.
    #[error(
        "no LibreOffice runtime: pass --lo-dir, set {env}, or build with the `embedded` feature",
        env = crate::ENV_LO_DIR
    )]
    NoRuntime,

    /// The directory exists but holds no LibreOfficeKit library in any known layout.
    #[error(
        "{0} is not a LibreOffice installation (no program/ or Contents/Frameworks with libmergedlo)"
    )]
    NotALibreOfficeTree(PathBuf),

    /// Filesystem failure with the path that caused it.
    #[error("{context} {path}: {source}")]
    Io {
        /// What was being attempted.
        context: &'static str,
        /// Path involved.
        path: PathBuf,
        /// Underlying error.
        #[source]
        source: std::io::Error,
    },

    /// Built with the `embedded` feature but without a tree to embed.
    #[error(
        "this binary was built without an embedded LibreOffice tree (PPTX_PDF_LO_TREE was unset)"
    )]
    NoEmbeddedTree,

    /// HTTP transport failure or non-success status while fetching the tree.
    #[cfg(feature = "download")]
    #[error("fetching {url}: {source}")]
    Http {
        /// What was being fetched.
        url: String,
        /// Underlying error.
        #[source]
        source: ureq::Error,
    },

    /// The downloaded tree did not hash to the expected value; nothing was kept.
    #[error("{file} failed verification: expected sha256 {expected}, got {actual}")]
    ChecksumMismatch {
        /// Tarball name.
        file: String,
        /// Pinned or published hash.
        expected: String,
        /// Hash of what was received.
        actual: String,
    },

    /// No pinned hash and `SHA256SUMS` has no entry for the tarball.
    #[error("no SHA-256 for {file} in {url}; cannot verify the download")]
    ChecksumUnavailable {
        /// Tarball name.
        file: String,
        /// The `SHA256SUMS` that was consulted.
        url: String,
    },

    /// Neither an embedded tree nor a download source was compiled in or configured for the
    /// platform this binary runs on.
    #[error("no LibreOffice tree is available for this platform ({0})")]
    UnsupportedPlatform(&'static str),

    /// No usable cache directory could be determined for unpacking the embedded tree.
    #[error("cannot determine a cache directory; set PPTX_PDF_CACHE_DIR")]
    NoCacheDir,
}

impl Error {
    pub(crate) fn io(
        context: &'static str,
        path: impl Into<PathBuf>,
    ) -> impl FnOnce(std::io::Error) -> Self {
        move |source| Self::Io {
            context,
            path: path.into(),
            source,
        }
    }
}
