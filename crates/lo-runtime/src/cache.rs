//! The per-user cache directory: unpacked trees, fontconfig caches.

use std::path::PathBuf;

use crate::Result;
use crate::error::Error;

/// Override for the cache location.
pub const ENV_CACHE_DIR: &str = "PPTX_PDF_CACHE_DIR";

/// `$PPTX_PDF_CACHE_DIR`, else the platform's per-user cache directory plus `pptx-pdf`.
///
/// # Errors
///
/// [`Error::NoCacheDir`] if neither the override nor `HOME` is set.
pub fn cache_dir() -> Result<PathBuf> {
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
