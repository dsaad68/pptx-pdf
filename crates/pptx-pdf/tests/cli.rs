//! End-to-end checks of the `pptx-pdf` binary.
//!
//! The conversion test only runs when `PPTX_PDF_LO_DIR` points at a LibreOffice tree and
//! `PPTX_PDF_TEST_DECK` at a `.pptx`; otherwise it reports itself skipped. The others need
//! nothing installed.

use std::path::PathBuf;
use std::process::Command;

const ENV_LO_DIR: &str = "PPTX_PDF_LO_DIR";
const ENV_DECK: &str = "PPTX_PDF_TEST_DECK";

fn bin() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pptx-pdf"));
    // Never let a test reach the network: a slim build would otherwise try the real release.
    cmd.env_remove(ENV_LO_DIR)
        .env_remove("RUST_LOG")
        .env(
            "PPTX_PDF_LO_TREE_URL",
            "http://127.0.0.1:9/unreachable.tar.zst",
        )
        .env(
            "PPTX_PDF_CACHE_DIR",
            std::env::temp_dir().join("pptx-pdf-cli-test-cache"),
        );
    cmd
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pptx-pdf-cli-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        std::fs::create_dir_all(&dir).is_ok(),
        "creating {}",
        dir.display()
    );
    dir
}

#[test]
fn missing_input_fails_before_touching_libreoffice() {
    let out = bin().arg("/definitely/not/here.pptx").output().unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("is not a file"), "stderr: {stderr}");
}

#[test]
fn reports_missing_runtime_clearly() {
    let dir = scratch("noruntime");
    let deck = dir.join("empty.pptx");
    std::fs::write(&deck, b"not really a deck").unwrap();
    let out = bin().arg(&deck).output().unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    // Depending on features: nothing configured, an empty embedded tree, or a download that
    // cannot reach its (deliberately dead) URL.
    assert!(
        stderr.contains("no LibreOffice runtime")
            || stderr.contains("embedded LibreOffice tree")
            || stderr.contains("fetching http://127.0.0.1:9/"),
        "stderr: {stderr}"
    );
}

#[test]
fn rejects_output_in_missing_directory() {
    let dir = scratch("badout");
    let deck = dir.join("deck.pptx");
    std::fs::write(&deck, b"x").unwrap();
    let out = bin()
        .arg(&deck)
        .arg("-o")
        .arg(dir.join("nope").join("out.pdf"))
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("does not exist"));
}

#[test]
fn converts_a_real_deck_when_a_runtime_is_configured() {
    let (Some(lo_dir), Some(deck)) = (std::env::var_os(ENV_LO_DIR), std::env::var_os(ENV_DECK))
    else {
        eprintln!("skipped: set {ENV_LO_DIR} and {ENV_DECK} to run");
        return;
    };
    let dir = scratch("convert");
    let output = dir.join("out.pdf");
    let out = bin()
        .env(ENV_LO_DIR, lo_dir)
        .arg(deck)
        .arg("-o")
        .arg(&output)
        .arg("-v")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        output.display().to_string()
    );
    let bytes = std::fs::read(&output).unwrap();
    assert!(
        bytes.starts_with(b"%PDF-"),
        "not a PDF: {:?}",
        bytes.get(..8)
    );
    assert!(bytes.len() > 1000);
}

#[test]
fn renders_png_files_when_a_runtime_is_configured() {
    let (Some(lo_dir), Some(deck)) = (std::env::var_os(ENV_LO_DIR), std::env::var_os(ENV_DECK))
    else {
        eprintln!("skipped: set {ENV_LO_DIR} and {ENV_DECK} to run");
        return;
    };
    let dir = scratch("png").join("out");
    let out = bin()
        .env(ENV_LO_DIR, lo_dir)
        .arg(deck)
        .arg("--png")
        .arg(&dir)
        .arg("--dpi")
        .arg("72")
        .arg("--pages")
        .arg("1-2")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let listed: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(
        listed,
        [
            dir.join("slide-01.png").display().to_string(),
            dir.join("slide-02.png").display().to_string()
        ]
    );
    let bytes = std::fs::read(dir.join("slide-01.png")).unwrap();
    assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    assert!(!dir.join("slide-03.png").exists());
}
