//! A throw-away LibreOffice user profile for one run.
//!
//! LibreOffice writes its configuration, lock files and temp data under a "user installation"
//! directory. Sharing one between concurrent processes deadlocks on its lock file, so each run
//! gets a fresh directory under the system temp dir, removed on drop.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use crate::error::Error;
use crate::{Result, url::file_url};

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// Font file extensions copied by [`Profile::add_fonts`].
const FONT_EXTENSIONS: [&str; 3] = ["ttf", "otf", "ttc"];

/// A per-run user profile directory.
#[derive(Debug)]
pub struct Profile {
    dir: PathBuf,
    url: String,
}

impl Profile {
    /// Create an empty profile directory and compute its `file://` URL.
    ///
    /// # Errors
    ///
    /// [`Error::Io`] if the directory cannot be created.
    pub fn create() -> Result<Self> {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("pptx-pdf-profile-{}-{n}", std::process::id()));
        std::fs::create_dir_all(&dir).map_err(Error::io("creating profile directory", &dir))?;
        let url = file_url(&dir)?;
        tracing::debug!(dir = %dir.display(), "created user profile");
        Ok(Self { dir, url })
    }

    /// The `file://` URL to hand to `lok::Office::load`.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// The profile directory on disk.
    #[must_use]
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// `user/fonts` inside the profile — scanned by LibreOffice and listed in the generated
    /// fontconfig configuration.
    #[must_use]
    pub fn fonts_dir(&self) -> PathBuf {
        self.dir.join("user").join("fonts")
    }

    /// Copy every font file directly inside `source` into the profile's `user/fonts`, which
    /// LibreOffice adds to its font path at startup. Returns the number of fonts copied.
    ///
    /// # Errors
    ///
    /// [`Error::Io`] if `source` cannot be read or a copy fails.
    pub fn add_fonts(&self, source: &Path) -> Result<usize> {
        let dest = self.fonts_dir();
        std::fs::create_dir_all(&dest)
            .map_err(Error::io("creating profile fonts directory", &dest))?;
        let mut copied = 0;
        for entry in
            std::fs::read_dir(source).map_err(Error::io("reading font directory", source))?
        {
            let path = entry
                .map_err(Error::io("reading font directory", source))?
                .path();
            if !is_font_file(&path) {
                continue;
            }
            let Some(name) = path.file_name() else {
                continue;
            };
            std::fs::copy(&path, dest.join(name)).map_err(Error::io("copying font", &path))?;
            copied += 1;
        }
        tracing::debug!(source = %source.display(), copied, "added fonts to profile");
        Ok(copied)
    }
}

fn is_font_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| FONT_EXTENSIONS.iter().any(|f| f.eq_ignore_ascii_case(e)))
}

impl Drop for Profile {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_dir_all(&self.dir) {
            tracing::warn!(dir = %self.dir.display(), error = %e, "failed to remove profile");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_removes_directory() {
        let dir = {
            let profile = Profile::create().unwrap();
            assert!(profile.dir().is_dir());
            assert!(profile.url().starts_with("file:///"));
            profile.dir().to_path_buf()
        };
        assert!(!dir.exists());
    }

    #[test]
    fn copies_only_font_files() {
        let src = std::env::temp_dir().join(format!("lo-runtime-fonts-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&src);
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("a.ttf"), b"x").unwrap();
        std::fs::write(src.join("b.OTF"), b"x").unwrap();
        std::fs::write(src.join("readme.txt"), b"x").unwrap();
        let profile = Profile::create().unwrap();
        assert_eq!(profile.add_fonts(&src).unwrap(), 2);
        assert!(profile.dir().join("user/fonts/a.ttf").is_file());
        assert!(!profile.dir().join("user/fonts/readme.txt").exists());
    }

    #[test]
    fn add_fonts_reports_missing_source() {
        let profile = Profile::create().unwrap();
        let missing = std::env::temp_dir().join("lo-runtime-definitely-missing");
        assert!(matches!(profile.add_fonts(&missing), Err(Error::Io { .. })));
    }
}
