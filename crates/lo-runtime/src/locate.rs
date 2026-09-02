//! Resolve an installation root to the directory that holds the LOK library.

use std::path::{Path, PathBuf};

use crate::Result;
use crate::error::Error;

/// Environment variable naming a LibreOffice installation root.
pub const ENV_LO_DIR: &str = "PPTX_PDF_LO_DIR";

#[cfg(target_os = "macos")]
const LIB_NAMES: [&str; 2] = ["libsofficeapp.dylib", "libmergedlo.dylib"];
#[cfg(all(unix, not(target_os = "macos")))]
const LIB_NAMES: [&str; 2] = ["libsofficeapp.so", "libmergedlo.so"];
#[cfg(windows)]
const LIB_NAMES: [&str; 2] = ["sofficeapp.dll", "mergedlo.dll"];

/// Subdirectories, relative to an installation root, that may hold the LOK library.
/// Order: Linux/Debian `program/`, macOS bundle, the root itself.
const LIB_SUBDIRS: [&str; 3] = ["program", "Contents/Frameworks", "."];

/// A located LibreOffice installation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runtime {
    install_root: PathBuf,
    program_dir: PathBuf,
}

impl Runtime {
    /// Find a runtime: `explicit`, then [`ENV_LO_DIR`], then the embedded tree if compiled in.
    ///
    /// # Errors
    ///
    /// [`Error::NoRuntime`] when nothing is configured; [`Error::NotALibreOfficeTree`] when a
    /// configured directory has no LOK library; unpack failures for the embedded tree.
    pub fn locate(explicit: Option<&Path>) -> Result<Self> {
        if let Some(root) = explicit {
            tracing::debug!(root = %root.display(), "using explicit LibreOffice directory");
            return Self::from_root(root);
        }
        if let Some(root) = std::env::var_os(ENV_LO_DIR) {
            tracing::debug!(root = %Path::new(&root).display(), "using {ENV_LO_DIR}");
            return Self::from_root(Path::new(&root));
        }
        Self::from_embedded()
    }

    #[cfg(feature = "embedded")]
    fn from_embedded() -> Result<Self> {
        let root = crate::embedded::ensure_unpacked()?;
        tracing::debug!(root = %root.display(), "using embedded LibreOffice tree");
        Self::from_root(&root)
    }

    #[cfg(not(feature = "embedded"))]
    fn from_embedded() -> Result<Self> {
        Err(Error::NoRuntime)
    }

    /// Resolve an installation root without consulting the environment.
    ///
    /// # Errors
    ///
    /// [`Error::NotALibreOfficeTree`] if no known subdirectory holds a LOK library.
    pub fn from_root(root: &Path) -> Result<Self> {
        let program_dir = LIB_SUBDIRS
            .iter()
            .map(|sub| root.join(sub))
            .find(|dir| LIB_NAMES.iter().any(|lib| dir.join(lib).is_file()))
            .ok_or_else(|| Error::NotALibreOfficeTree(root.to_path_buf()))?;
        Ok(Self {
            install_root: root.to_path_buf(),
            program_dir,
        })
    }

    /// Directory that directly contains the LOK library — what `lok::Office::load` wants.
    #[must_use]
    pub fn program_dir(&self) -> &Path {
        &self.program_dir
    }

    /// The installation root as given.
    #[must_use]
    pub fn install_root(&self) -> &Path {
        &self.install_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_root(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("lo-runtime-test-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn rejects_empty_root() {
        let root = tmp_root("empty");
        assert!(matches!(
            Runtime::from_root(&root),
            Err(Error::NotALibreOfficeTree(_))
        ));
    }

    #[test]
    fn finds_linux_program_layout() {
        let root = tmp_root("linux");
        std::fs::create_dir_all(root.join("program")).unwrap();
        std::fs::write(root.join("program").join(LIB_NAMES[1]), b"x").unwrap();
        assert_eq!(
            Runtime::from_root(&root).unwrap().program_dir(),
            root.join("program")
        );
    }

    #[test]
    fn finds_macos_bundle_layout() {
        let root = tmp_root("mac");
        std::fs::create_dir_all(root.join("Contents/Frameworks")).unwrap();
        std::fs::write(root.join("Contents/Frameworks").join(LIB_NAMES[0]), b"x").unwrap();
        assert_eq!(
            Runtime::from_root(&root).unwrap().program_dir(),
            root.join("Contents/Frameworks")
        );
    }

    #[test]
    fn accepts_program_dir_itself_as_root() {
        let root = tmp_root("flat");
        std::fs::write(root.join(LIB_NAMES[1]), b"x").unwrap();
        assert_eq!(
            Runtime::from_root(&root).unwrap().program_dir(),
            root.join(".")
        );
    }
}
