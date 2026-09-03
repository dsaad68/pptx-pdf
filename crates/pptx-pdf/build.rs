//! Re-run when the pinned tree hash changes, so `option_env!` in `tree.rs` sees it.

fn main() {
    println!("cargo:rerun-if-env-changed=PPTX_PDF_LO_TREE_SHA256");
}
