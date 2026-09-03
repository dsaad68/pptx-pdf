//! The LibreOffice tree compiled into the binary.
//!
//! The tarball is keyed by its SHA-256 (computed in `build.rs`), so a new tree never collides
//! with a cached older one, a rebuild with the same tree reuses the cache, and a slim binary
//! that downloads the same tree shares it.

use std::path::PathBuf;

use crate::error::Error;
use crate::unpack::{tree_dir_name, unpack_zstd_tar};
use crate::{Result, cache};

static TREE: &[u8] = include_bytes!(env!("PPTX_PDF_LO_TREE"));
const TREE_SHA256: &str = env!("PPTX_PDF_LO_TREE_SHA256");

/// Unpack the embedded tree if not already cached, and return its root.
///
/// # Errors
///
/// [`Error::NoEmbeddedTree`] if the binary was built without `PPTX_PDF_LO_TREE`;
/// [`Error::NoCacheDir`] if no cache directory can be determined; [`Error::Io`] on unpack
/// failure.
pub(crate) fn ensure_unpacked() -> Result<PathBuf> {
    if TREE.is_empty() {
        return Err(Error::NoEmbeddedTree);
    }
    let root = cache::cache_dir()?.join(tree_dir_name(TREE_SHA256));
    if !root.is_dir() {
        tracing::info!(root = %root.display(), bytes = TREE.len(), "unpacking embedded LibreOffice tree");
    }
    unpack_zstd_tar(TREE, &root)
}
