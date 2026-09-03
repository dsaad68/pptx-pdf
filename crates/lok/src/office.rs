//! The process-wide LibreOfficeKit instance.

use std::ffi::{CStr, CString, c_char};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, offset_of, size_of};
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::document::Document;
use crate::error::Error;
use crate::{Result, ffi};

/// Set once the hook has been called. Never cleared: LOK does not support re-initialisation
/// even after `destroy`, so a failed or destroyed instance still forbids a second one.
static INITIALISED: AtomicBool = AtomicBool::new(false);

/// Candidate library names in the order LOK's own `lok_dlopen` tries them.
#[cfg(target_os = "macos")]
const LIB_NAMES: [&str; 2] = ["libsofficeapp.dylib", "libmergedlo.dylib"];
#[cfg(all(unix, not(target_os = "macos")))]
const LIB_NAMES: [&str; 2] = ["libsofficeapp.so", "libmergedlo.so"];
#[cfg(windows)]
const LIB_NAMES: [&str; 2] = ["sofficeapp.dll", "mergedlo.dll"];

/// A merged-libs build leaves a one-line text stub in place of `libsofficeapp`; LOK skips
/// any candidate at or below this size.
const STUB_SIZE_LIMIT: u64 = 1000;

/// A booted LibreOffice instance.
///
/// Exactly one may exist per process. It is `!Send` and `!Sync`: every call, including
/// [`Drop`], must happen on the thread that created it.
pub struct Office {
    kit: NonNull<ffi::LibreOfficeKit>,
    /// Never `dlclose`d. LibreOffice cannot be unloaded from a live process; LOK's own
    /// `lok_init_2` leaks the handle on purpose and so do we.
    _library: ManuallyDrop<libloading::Library>,
    _not_send: PhantomData<*mut ()>,
}

impl std::fmt::Debug for Office {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Office")
            .field("kit", &self.kit)
            .finish_non_exhaustive()
    }
}

impl Office {
    /// Load LibreOfficeKit from `program_dir` and boot it.
    ///
    /// `program_dir` is the directory that directly contains `libsofficeapp` or `libmergedlo`
    /// (`program/` in a Linux tree, `Contents/Frameworks` in a macOS bundle).
    /// `user_profile_url` must be a `file://` URL, not a path; LibreOffice creates the
    /// profile there on first use.
    ///
    /// # Errors
    ///
    /// - [`Error::AlreadyInitialised`] if any `Office` was created earlier in this process.
    /// - [`Error::LibraryNotFound`] / [`Error::LibraryLoad`] / [`Error::MissingHook`] if the
    ///   tree at `program_dir` is not a usable LibreOffice.
    /// - [`Error::Init`] if LibreOffice itself refused to start (it prints why on stderr).
    /// - [`Error::AbiTooOld`] if the library predates the LOK members this crate relies on.
    pub fn load(program_dir: &Path, user_profile_url: &str) -> Result<Self> {
        let lib_path = pick_library(program_dir)?;
        tracing::debug!(path = %lib_path.display(), "loading LibreOfficeKit");

        // SAFETY: loading LibreOffice runs its static initialisers, which is the documented
        // way to bring it up and takes no arguments from us. The library is never unloaded.
        let library = unsafe { libloading::Library::new(&lib_path) }.map_err(|source| {
            Error::LibraryLoad {
                path: lib_path.clone(),
                source,
            }
        })?;

        let install_path = c_string_from_path(program_dir)?;
        let profile = c_string(user_profile_url)?;

        // Scoped so the symbol's borrow of `library` ends before `library` moves into `Self`.
        let raw = {
            // SAFETY: the symbol's signature is fixed by LibreOfficeKitInit.h (`LokHookFunction2`).
            let hook = unsafe { library.get::<ffi::LokHookFunction2>(ffi::HOOK_SYMBOL) }
                .map_err(|_| Error::MissingHook(lib_path.clone()))?;
            if INITIALISED.swap(true, Ordering::SeqCst) {
                return Err(Error::AlreadyInitialised);
            }
            // SAFETY: both pointers are valid NUL-terminated strings for the duration of the call.
            unsafe { hook(install_path.as_ptr(), profile.as_ptr()) }
        };
        let kit = NonNull::new(raw).ok_or(Error::Init)?;

        let office = Self {
            kit,
            _library: ManuallyDrop::new(library),
            _not_send: PhantomData,
        };
        office.require_member(offset_of!(ffi::LibreOfficeKitClass, destroy), "destroy")?;
        office.require_member(
            offset_of!(ffi::LibreOfficeKitClass, document_load),
            "documentLoad",
        )?;
        office.require_member(offset_of!(ffi::LibreOfficeKitClass, get_error), "getError")?;
        office.require_member(
            offset_of!(ffi::LibreOfficeKitClass, free_error),
            "freeError",
        )?;
        tracing::debug!(version = ?office.version_info(), "LibreOfficeKit initialised");
        Ok(office)
    }

    /// `{"ProductName": ..., "ProductVersion": ..., "BuildId": ...}` as reported by the
    /// library, or `None` on builds too old to expose it.
    #[must_use]
    pub fn version_info(&self) -> Option<String> {
        let offset = offset_of!(ffi::LibreOfficeKitClass, get_version_info);
        if !ffi::has_member(offset, size_of::<usize>(), self.class().n_size) {
            return None;
        }
        let f = self.class().get_version_info?;
        // SAFETY: `kit` is a live instance created on this thread; the returned buffer is
        // owned by us until handed to `free_error`.
        let raw = unsafe { f(self.kit.as_ptr()) };
        self.take_c_string(raw)
    }

    /// Load the document at `url` (a `file://` URL).
    ///
    /// # Errors
    ///
    /// [`Error::DocumentLoad`] with LibreOffice's message if the file cannot be opened or
    /// no import filter accepts it; [`Error::InteriorNul`] if `url` contains a NUL byte.
    pub fn load_document(&self, url: &str) -> Result<Document<'_>> {
        let c_url = c_string(url)?;
        let load = self
            .class()
            .document_load
            .ok_or(Error::AbiTooOld("documentLoad"))?;
        tracing::debug!(url, "loading document");
        // SAFETY: `kit` is live and `c_url` outlives the call.
        let raw = unsafe { load(self.kit.as_ptr(), c_url.as_ptr()) };
        let ptr = NonNull::new(raw).ok_or_else(|| Error::DocumentLoad(self.take_error()))?;
        // SAFETY: `ptr` was just returned by `documentLoad` and belongs to this `Office`.
        Ok(unsafe { Document::from_raw(ptr, self) })
    }

    /// Drain LibreOffice's last error message, or a placeholder when it left none.
    pub(crate) fn take_error(&self) -> String {
        let Some(get_error) = self.class().get_error else {
            return String::from("unknown error (getError unavailable)");
        };
        // SAFETY: `kit` is live; ownership of the buffer passes to us.
        let raw = unsafe { get_error(self.kit.as_ptr()) };
        self.take_c_string(raw)
            .unwrap_or_else(|| String::from("unknown error"))
    }

    /// Copy a library-allocated string and release it through `freeError`.
    pub(crate) fn take_c_string(&self, raw: *mut c_char) -> Option<String> {
        if raw.is_null() {
            return None;
        }
        // SAFETY: LOK returns NUL-terminated strings that stay valid until freed below.
        let text = unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned();
        if let Some(free) = self.class().free_error {
            // SAFETY: `raw` came from this library and is freed exactly once.
            unsafe { free(raw) };
        }
        Some(text)
    }

    fn require_member(&self, offset: usize, name: &'static str) -> Result<()> {
        if ffi::has_member(offset, size_of::<usize>(), self.class().n_size) {
            Ok(())
        } else {
            Err(Error::AbiTooOld(name))
        }
    }

    pub(crate) fn class(&self) -> &ffi::LibreOfficeKitClass {
        // SAFETY: `kit` points at a live instance whose `p_class` is set for its lifetime.
        unsafe { &*(*self.kit.as_ptr()).p_class }
    }
}

impl Drop for Office {
    fn drop(&mut self) {
        if let Some(destroy) = self.class().destroy {
            // SAFETY: `kit` is live and is not used after this call.
            unsafe { destroy(self.kit.as_ptr()) };
        }
    }
}

/// Mirror of `lok_dlopen`'s candidate selection.
fn pick_library(program_dir: &Path) -> Result<PathBuf> {
    LIB_NAMES
        .iter()
        .map(|name| program_dir.join(name))
        .find(|p| {
            std::fs::metadata(p)
                .map(|m| m.is_file() && m.len() > STUB_SIZE_LIMIT)
                .unwrap_or(false)
        })
        .ok_or_else(|| Error::LibraryNotFound(program_dir.to_path_buf()))
}

fn c_string(s: &str) -> Result<CString> {
    CString::new(s).map_err(|_| Error::InteriorNul(s.to_owned()))
}

#[cfg(unix)]
fn c_string_from_path(path: &Path) -> Result<CString> {
    use std::os::unix::ffi::OsStrExt;
    CString::new(path.as_os_str().as_bytes())
        .map_err(|_| Error::InteriorNul(path.to_string_lossy().into_owned()))
}

#[cfg(not(unix))]
fn c_string_from_path(path: &Path) -> Result<CString> {
    c_string(&path.to_string_lossy())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("lok-test-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn rejects_directory_without_library() {
        let dir = tmp_dir("empty");
        assert!(matches!(pick_library(&dir), Err(Error::LibraryNotFound(_))));
    }

    #[test]
    fn skips_stub_and_falls_back_to_merged() {
        let dir = tmp_dir("stub");
        std::fs::write(dir.join(LIB_NAMES[0]), "stub").unwrap();
        std::fs::write(dir.join(LIB_NAMES[1]), vec![0u8; 2000]).unwrap();
        assert_eq!(pick_library(&dir).unwrap(), dir.join(LIB_NAMES[1]));
    }

    #[test]
    fn prefers_real_sofficeapp() {
        let dir = tmp_dir("real");
        std::fs::write(dir.join(LIB_NAMES[0]), vec![0u8; 2000]).unwrap();
        std::fs::write(dir.join(LIB_NAMES[1]), vec![0u8; 2000]).unwrap();
        assert_eq!(pick_library(&dir).unwrap(), dir.join(LIB_NAMES[0]));
    }

    #[test]
    fn rejects_interior_nul() {
        assert!(matches!(c_string("a\0b"), Err(Error::InteriorNul(_))));
    }
}
