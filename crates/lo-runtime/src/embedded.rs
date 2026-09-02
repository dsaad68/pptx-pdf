//! The LibreOffice tree compiled into the binary, and its unpack-to-cache logic.
//!
//! The tarball is keyed by its SHA-256 (computed in `build.rs`), so a new tree never
//! collides with a cached older one, and a rebuild with the same tree reuses the cache.
//! Unpacking goes into a sibling temp directory that is renamed into place only once
//! complete, so an interrupted first run cannot leave a half tree that looks valid.

use std::path::PathBuf;

use crate::Result;
use crate::error::Error;

static TREE: &[u8] = include_bytes!(env!("PPTX_PDF_LO_TREE"));
const TREE_SHA256: &str = env!("PPTX_PDF_LO_TREE_SHA256");

/// Override for the cache location.
const ENV_CACHE_DIR: &str = "PPTX_PDF_CACHE_DIR";

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
    let root = cache_dir()?.join(format!(
        "lo-{}",
        TREE_SHA256.get(..16).unwrap_or(TREE_SHA256)
    ));
    if root.is_dir() {
        return Ok(root);
    }
    tracing::info!(root = %root.display(), bytes = TREE.len(), "unpacking embedded LibreOffice tree");
    let staging = root.with_extension(format!("tmp-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&staging);
    std::fs::create_dir_all(&staging).map_err(Error::io("creating cache directory", &staging))?;
    let decoder = zstd::Decoder::new(TREE).map_err(Error::io("opening embedded tree", &staging))?;
    tar::Archive::new(decoder)
        .unpack(&staging)
        .map_err(Error::io("unpacking embedded tree", &staging))?;
    match std::fs::rename(&staging, &root) {
        Ok(()) => {}
        // Another process finished first; its copy is byte-identical.
        Err(_) if root.is_dir() => {
            let _ = std::fs::remove_dir_all(&staging);
        }
        Err(e) => return Err(Error::io("finalising cache directory", &root)(e)),
    }
    Ok(root)
}

fn cache_dir() -> Result<PathBuf> {
    if let Some(dir) = std::env::var_os(ENV_CACHE_DIR) {
        return Ok(PathBuf::from(dir));
    }
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or(Error::NoCacheDir)?;
    let base = if cfg!(target_os = "macos") {
        home.join("Library").join("Caches")
    } else {
        std::env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".cache"))
    };
    Ok(base.join("pptx-pdf"))
}
