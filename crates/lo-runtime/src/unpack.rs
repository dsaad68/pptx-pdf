//! Unpacking a `tar.zst` tree into the cache, shared by the embedded and download sources.
//!
//! The archive is unpacked into a sibling staging directory that is renamed into place only
//! once complete, so an interrupted run cannot leave a half tree that looks valid, and two
//! processes racing to unpack the same tree both end up with one good copy.

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::Result;
use crate::error::Error;

/// Cache directory name for a tree with this SHA-256 (hex): `lo-<first 16 hex digits>`.
#[must_use]
pub(crate) fn tree_dir_name(sha256_hex: &str) -> String {
    format!("lo-{}", sha256_hex.get(..16).unwrap_or(sha256_hex))
}

/// Unpack a zstd-compressed tar stream into `root`, unless `root` already exists.
pub(crate) fn unpack_zstd_tar(archive: impl Read, root: &Path) -> Result<PathBuf> {
    if root.is_dir() {
        return Ok(root.to_path_buf());
    }
    let staging = root.with_extension(format!("tmp-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&staging);
    std::fs::create_dir_all(&staging).map_err(Error::io("creating cache directory", &staging))?;
    let decoder =
        zstd::Decoder::new(archive).map_err(Error::io("opening tree archive", &staging))?;
    tar::Archive::new(decoder)
        .unpack(&staging)
        .map_err(Error::io("unpacking tree archive", &staging))?;
    match std::fs::rename(&staging, root) {
        Ok(()) => {}
        // Another process finished first; its copy is byte-identical.
        Err(_) if root.is_dir() => {
            let _ = std::fs::remove_dir_all(&staging);
        }
        Err(e) => return Err(Error::io("finalising cache directory", root)(e)),
    }
    Ok(root.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_name_uses_hash_prefix() {
        assert_eq!(
            tree_dir_name("0123456789abcdef0123456789abcdef"),
            "lo-0123456789abcdef"
        );
        assert_eq!(tree_dir_name("abc"), "lo-abc");
    }
}
