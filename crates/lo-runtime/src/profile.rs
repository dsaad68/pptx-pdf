//! The LibreOffice user profile for a run.
//!
//! LibreOffice writes its configuration, lock files, temp data and, importantly, its font
//! cache under a "user installation" directory. A fresh directory per run is safe for
//! concurrent processes but makes LibreOffice re-enumerate every font on every start, which
//! on macOS costs seconds. So [`Profile::acquire`] first tries a persistent profile in the
//! cache directory, guarded by a file lock, and falls back to a throw-away one in the temp
//! directory when another process holds it. Sharing a profile between processes without the
//! lock would deadlock on LibreOffice's own lock file.

use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use crate::error::Error;
use crate::{Result, cache, url::file_url};

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// Font file extensions copied by [`Profile::add_fonts`].
const FONT_EXTENSIONS: [&str; 3] = ["ttf", "otf", "ttc"];

/// A user profile directory: persistent and locked, or per-run and removed on drop.
#[derive(Debug)]
pub struct Profile {
    dir: PathBuf,
    url: String,
    /// Held for the profile's lifetime when persistent; `None` for a throw-away profile,
    /// which is deleted on drop.
    lock: Option<File>,
}

impl Profile {
    /// The persistent profile if it can be locked, else a fresh temporary one.
    ///
    /// # Errors
    ///
    /// [`Error::Io`] if neither directory can be created.
    pub fn acquire() -> Result<Self> {
        match Self::persistent() {
            Ok(Some(profile)) => return Ok(profile),
            Ok(None) => tracing::debug!("persistent profile is in use; using a temporary one"),
            Err(e) => {
                tracing::debug!(error = %e, "persistent profile unavailable; using a temporary one")
            }
        }
        Self::create()
    }

    /// Lock and return the persistent profile under the cache directory, or `None` if another
    /// process holds it.
    ///
    /// # Errors
    ///
    /// [`Error::NoCacheDir`] or [`Error::Io`] if the directory cannot be set up.
    pub fn persistent() -> Result<Option<Self>> {
        Self::persistent_in(cache::cache_dir()?.join("profile"))
    }

    /// Lock and return the persistent profile at `dir`, or `None` if another process holds it.
    ///
    /// # Errors
    ///
    /// [`Error::Io`] if the directory or its lock file cannot be created.
    pub fn persistent_in(dir: PathBuf) -> Result<Option<Self>> {
        std::fs::create_dir_all(&dir).map_err(Error::io("creating profile directory", &dir))?;
        let lock_path = dir.with_extension("lock");
        let lock =
            File::create(&lock_path).map_err(Error::io("creating profile lock", &lock_path))?;
        if lock.try_lock().is_err() {
            return Ok(None);
        }
        // We now hold the only handle on this profile. LibreOffice's own lock file survives an
        // abort and makes the next start refuse to load documents; ours makes it redundant.
        let _ = std::fs::remove_file(dir.join(".lock"));
        let url = file_url(&dir)?;
        tracing::debug!(dir = %dir.display(), "using persistent user profile");
        Ok(Some(Self {
            dir,
            url,
            lock: Some(lock),
        }))
    }

    /// Whether this is the shared persistent profile (as opposed to a per-run one).
    #[must_use]
    pub fn is_persistent(&self) -> bool {
        self.lock.is_some()
    }

    /// Create an empty per-run profile directory, removed on drop.
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
        Ok(Self {
            dir,
            url,
            lock: None,
        })
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
        if self.lock.is_some() {
            return; // persistent: keep it, the lock is released with the file
        }
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
    fn persistent_profile_is_exclusive_and_survives_drop() {
        let dir = std::env::temp_dir().join(format!("lo-runtime-profile-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let first = Profile::persistent_in(dir.clone())
            .unwrap()
            .expect("first lock");
        assert!(first.is_persistent());
        assert!(
            Profile::persistent_in(dir.clone()).unwrap().is_none(),
            "second acquirer is refused"
        );
        std::fs::write(dir.join(".lock"), b"stale LibreOffice lock").unwrap();
        drop(first);
        assert!(dir.is_dir(), "persistent profile is kept on drop");
        assert!(
            Profile::persistent_in(dir.clone()).unwrap().is_some(),
            "lock released on drop"
        );
        assert!(
            !dir.join(".lock").exists(),
            "a stale LibreOffice lock is cleared on acquire"
        );
        let _ = std::fs::remove_dir_all(&dir);
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
