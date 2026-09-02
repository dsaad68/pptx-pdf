//! Raw C ABI of LibreOfficeKit, transcribed from `include/LibreOfficeKit/LibreOfficeKit.h`.
//!
//! LOK keeps ABI stability by only ever *appending* members to its class structs and
//! reporting the populated byte count in `n_size`. Callers never allocate these structs;
//! they only read them through pointers the library hands out. So declaring a prefix is
//! sound, provided every member is checked against `n_size` before use — see
//! [`has_member`].
//!
//! Member order here must match the header exactly. The `abi_offsets` test pins the
//! offsets LibreOffice's own `desktop/qa/desktop_lib/test_desktop_lib.cxx` asserts.

use std::ffi::{c_char, c_int, c_void};

/// Opaque LOK instance; only `p_class` is visible.
#[repr(C)]
#[derive(Debug)]
pub struct LibreOfficeKit {
    /// Method table.
    pub p_class: *const LibreOfficeKitClass,
}

/// Opaque LOK document; only `p_class` is visible.
#[repr(C)]
#[derive(Debug)]
pub struct LibreOfficeKitDocument {
    /// Method table.
    pub p_class: *const LibreOfficeKitDocumentClass,
}

/// `void (*)(int nType, const char* pPayload, void* pData)`.
pub type LibreOfficeKitCallback =
    Option<unsafe extern "C" fn(n_type: c_int, p_payload: *const c_char, p_data: *mut c_void)>;

/// Prefix of `LibreOfficeKitClassStruct`.
#[repr(C)]
#[derive(Debug)]
pub struct LibreOfficeKitClass {
    /// Number of bytes of this struct the library populated.
    pub n_size: usize,
    /// Tear down the instance.
    pub destroy: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKit)>,
    /// Load a document by URL.
    pub document_load: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKit,
            p_url: *const c_char,
        ) -> *mut LibreOfficeKitDocument,
    >,
    /// Last error as a malloc'd string; free with [`Self::free_error`].
    pub get_error: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKit) -> *mut c_char>,
    /// Load a document by URL with a filter-options string.
    pub document_load_with_options: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKit,
            p_url: *const c_char,
            p_options: *const c_char,
        ) -> *mut LibreOfficeKitDocument,
    >,
    /// Free a string returned by [`Self::get_error`] or [`Self::get_version_info`].
    pub free_error: Option<unsafe extern "C" fn(p_free: *mut c_char)>,
    /// Register a global callback.
    pub register_callback: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKit,
            p_callback: LibreOfficeKitCallback,
            p_data: *mut c_void,
        ),
    >,
    /// JSON list of import/export filters as a malloc'd string.
    pub get_filter_types: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKit) -> *mut c_char>,
    /// Bitmask of `LibreOfficeKitOptionalFeatures`.
    pub set_optional_features:
        Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKit, features: u64)>,
    /// Supply a password for a URL after a password-required callback.
    pub set_document_password: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKit,
            p_url: *const c_char,
            p_password: *const c_char,
        ),
    >,
    /// JSON with `ProductName`, `ProductVersion`, `BuildId` as a malloc'd string.
    pub get_version_info: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKit) -> *mut c_char>,
}

/// Prefix of `LibreOfficeKitDocumentClassStruct`.
#[repr(C)]
#[derive(Debug)]
pub struct LibreOfficeKitDocumentClass {
    /// Number of bytes of this struct the library populated.
    pub n_size: usize,
    /// Close the document.
    pub destroy: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument)>,
    /// Export to `p_url` in `p_format` (e.g. `"pdf"`) with an optional filter-options string.
    /// Returns non-zero on success.
    pub save_as: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_url: *const c_char,
            p_format: *const c_char,
            p_filter_options: *const c_char,
        ) -> c_int,
    >,
    /// One of the `LOK_DOCTYPE_*` values.
    pub get_document_type:
        Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument) -> c_int>,
}

/// `LibreOfficeKitDocumentType` from `LibreOfficeKitEnums.h`.
pub const LOK_DOCTYPE_TEXT: c_int = 0;
/// `LibreOfficeKitDocumentType` from `LibreOfficeKitEnums.h`.
pub const LOK_DOCTYPE_SPREADSHEET: c_int = 1;
/// `LibreOfficeKitDocumentType` from `LibreOfficeKitEnums.h`.
pub const LOK_DOCTYPE_PRESENTATION: c_int = 2;
/// `LibreOfficeKitDocumentType` from `LibreOfficeKitEnums.h`.
pub const LOK_DOCTYPE_DRAWING: c_int = 3;
/// `LibreOfficeKitDocumentType` from `LibreOfficeKitEnums.h`.
pub const LOK_DOCTYPE_OTHER: c_int = 4;

/// Signature of the exported `libreofficekit_hook_2(install_path, user_profile_url)`.
pub type LokHookFunction2 = unsafe extern "C" fn(
    install_path: *const c_char,
    user_profile_url: *const c_char,
) -> *mut LibreOfficeKit;

/// Symbol name of the hook, NUL-terminated for `dlsym`.
pub const HOOK_SYMBOL: &[u8] = b"libreofficekit_hook_2\0";

/// Mirror of `LIBREOFFICEKIT_HAS_MEMBER`: a member is usable only if the populated size
/// covers it entirely.
#[must_use]
pub const fn has_member(member_offset: usize, member_size: usize, n_size: usize) -> bool {
    member_offset + member_size <= n_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{offset_of, size_of};

    const PTR: usize = size_of::<usize>();

    /// Offsets asserted by LibreOffice's own ABI test; a drift here means the transcription
    /// above is wrong, not the library.
    #[test]
    fn abi_offsets_match_header() {
        assert_eq!(offset_of!(LibreOfficeKitClass, n_size), 0);
        assert_eq!(offset_of!(LibreOfficeKitClass, destroy), PTR);
        assert_eq!(offset_of!(LibreOfficeKitClass, document_load), 2 * PTR);
        assert_eq!(offset_of!(LibreOfficeKitClass, get_error), 3 * PTR);
        assert_eq!(
            offset_of!(LibreOfficeKitClass, document_load_with_options),
            4 * PTR
        );
        assert_eq!(offset_of!(LibreOfficeKitClass, free_error), 5 * PTR);
        assert_eq!(offset_of!(LibreOfficeKitClass, register_callback), 6 * PTR);
        assert_eq!(offset_of!(LibreOfficeKitClass, get_filter_types), 7 * PTR);
        assert_eq!(
            offset_of!(LibreOfficeKitClass, set_optional_features),
            8 * PTR
        );
        assert_eq!(
            offset_of!(LibreOfficeKitClass, set_document_password),
            9 * PTR
        );
        assert_eq!(offset_of!(LibreOfficeKitClass, get_version_info), 10 * PTR);

        assert_eq!(offset_of!(LibreOfficeKitDocumentClass, n_size), 0);
        assert_eq!(offset_of!(LibreOfficeKitDocumentClass, destroy), PTR);
        assert_eq!(offset_of!(LibreOfficeKitDocumentClass, save_as), 2 * PTR);
        assert_eq!(
            offset_of!(LibreOfficeKitDocumentClass, get_document_type),
            3 * PTR
        );
    }

    #[test]
    fn has_member_is_inclusive_of_last_byte() {
        assert!(has_member(2 * PTR, PTR, 3 * PTR));
        assert!(!has_member(2 * PTR, PTR, 3 * PTR - 1));
        assert!(!has_member(2 * PTR, PTR, 0));
    }
}
