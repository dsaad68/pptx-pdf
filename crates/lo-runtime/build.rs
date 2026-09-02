//! With the `embedded` feature, hashes the tree tarball named by `PPTX_PDF_LO_TREE` so the
//! crate can key its unpack cache on content rather than on a version string.
//!
//! When the variable is unset the crate still compiles — `cargo clippy --all-features` must
//! work on a machine without a tree — but embeds an empty placeholder that fails at runtime
//! with a clear message.

use std::env;
use std::fs;
use std::io::{self, Read as _};
use std::path::PathBuf;

use sha2::{Digest as _, Sha256};

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-env-changed=PPTX_PDF_LO_TREE");
    if env::var_os("CARGO_FEATURE_EMBEDDED").is_none() {
        return Ok(());
    }
    let out_dir =
        PathBuf::from(env::var_os("OUT_DIR").ok_or_else(|| io::Error::other("OUT_DIR unset"))?);
    let (tree, sha) = match env::var_os("PPTX_PDF_LO_TREE") {
        Some(tree) => {
            let tree = PathBuf::from(tree);
            println!("cargo:rerun-if-changed={}", tree.display());
            let sha = sha256_hex(&tree)?;
            (tree, sha)
        }
        None => {
            println!("cargo:warning=PPTX_PDF_LO_TREE unset; embedding an empty placeholder tree");
            let placeholder = out_dir.join("empty-tree.bin");
            fs::write(&placeholder, b"")?;
            (placeholder, String::new())
        }
    };
    println!("cargo:rustc-env=PPTX_PDF_LO_TREE={}", tree.display());
    println!("cargo:rustc-env=PPTX_PDF_LO_TREE_SHA256={sha}");
    Ok(())
}

fn sha256_hex(path: &PathBuf) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(buf.get(..n).unwrap_or_default());
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}
