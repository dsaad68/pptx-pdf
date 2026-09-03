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

use std::ffi::{c_char, c_int, c_long, c_uint, c_void};

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

    // Everything below is behind `LOK_USE_UNSTABLE_API` in the header: LibreOffice reserves
    // the right to change these between releases. Each binary of ours pins one LibreOffice
    // build, and `abi_offsets_match_header` pins this transcription to that header.
    /// Number of parts: slides in Impress, sheets in Calc.
    pub get_parts: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument) -> c_int>,
    /// Page rectangles of a text document as a malloc'd string.
    pub get_part_page_rectangles:
        Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument) -> *mut c_char>,
    /// Currently selected part.
    pub get_part: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument) -> c_int>,
    /// Select a part; later rendering calls operate on it.
    pub set_part: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument, n_part: c_int)>,
    /// Name of a part as a malloc'd string.
    pub get_part_name: Option<
        unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument, n_part: c_int) -> *mut c_char,
    >,
    /// `LOK_PARTMODE_*`: slides or notes.
    pub set_part_mode:
        Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument, n_mode: c_int)>,
    /// Render a rectangle of the current part (twips) into a `canvas_width * canvas_height`
    /// buffer of 4-byte pixels in the order [`Self::get_tile_mode`] reports.
    pub paint_tile: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_buffer: *mut u8,
            n_canvas_width: c_int,
            n_canvas_height: c_int,
            n_tile_pos_x: c_int,
            n_tile_pos_y: c_int,
            n_tile_width: c_int,
            n_tile_height: c_int,
        ),
    >,
    /// `LOK_TILEMODE_*`: the pixel order of [`Self::paint_tile`] buffers.
    pub get_tile_mode: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument) -> c_int>,
    /// Size of the current part in twips.
    pub get_document_size: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_width: *mut c_long,
            p_height: *mut c_long,
        ),
    >,
    /// Prepare the document for tiled rendering; takes a JSON object of `.uno:` settings.
    pub initialize_for_rendering: Option<
        unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument, p_arguments: *const c_char),
    >,
    /// Register a per-document callback.
    pub register_callback: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_callback: LibreOfficeKitCallback,
            p_data: *mut c_void,
        ),
    >,
    /// Editing: keyboard input.
    pub post_key_event: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_type: c_int,
            n_char_code: c_int,
            n_key_code: c_int,
        ),
    >,
    /// Editing: mouse input.
    pub post_mouse_event: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_type: c_int,
            n_x: c_int,
            n_y: c_int,
            n_count: c_int,
            n_buttons: c_int,
            n_modifier: c_int,
        ),
    >,
    /// Dispatch a `.uno:` command with JSON arguments.
    pub post_uno_command: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_command: *const c_char,
            p_arguments: *const c_char,
            b_notify_when_finished: bool,
        ),
    >,
    /// Editing: text selection.
    pub set_text_selection: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_type: c_int,
            n_x: c_int,
            n_y: c_int,
        ),
    >,
    /// Editing: selected text as a malloc'd string.
    pub get_text_selection: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_mime_type: *const c_char,
            p_used_mime_type: *mut *mut c_char,
        ) -> *mut c_char,
    >,
    /// Editing: paste.
    pub paste: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_mime_type: *const c_char,
            p_data: *const c_char,
            n_size: usize,
        ) -> bool,
    >,
    /// Editing: graphic selection.
    pub set_graphic_selection: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_type: c_int,
            n_x: c_int,
            n_y: c_int,
        ),
    >,
    /// Editing: clear the selection.
    pub reset_selection: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument)>,
    /// State of a `.uno:` command as a malloc'd JSON string.
    pub get_command_values: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_command: *const c_char,
        ) -> *mut c_char,
    >,
    /// Tell Calc the client's tile geometry.
    pub set_client_zoom: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_tile_pixel_width: c_int,
            n_tile_pixel_height: c_int,
            n_tile_twip_width: c_int,
            n_tile_twip_height: c_int,
        ),
    >,
    /// Tell the document the client's visible area (twips).
    pub set_client_visible_area: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_x: c_int,
            n_y: c_int,
            n_width: c_int,
            n_height: c_int,
        ),
    >,
    /// Views: create.
    pub create_view: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument) -> c_int>,
    /// Views: destroy.
    pub destroy_view:
        Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument, n_id: c_int)>,
    /// Views: select.
    pub set_view: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument, n_id: c_int)>,
    /// Views: current.
    pub get_view: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument) -> c_int>,
    /// Views: count.
    pub get_views_count: Option<unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument) -> c_int>,
    /// Render a font sample as a malloc'd pixel buffer.
    pub render_font: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_font_name: *const c_char,
            p_char: *const c_char,
            p_font_width: *mut c_int,
            p_font_height: *mut c_int,
        ) -> *mut u8,
    >,
    /// Hash of a part as a malloc'd string.
    pub get_part_hash: Option<
        unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument, n_part: c_int) -> *mut c_char,
    >,
    /// [`Self::paint_tile`] for an explicit part and edit mode.
    pub paint_part_tile: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_buffer: *mut u8,
            n_part: c_int,
            n_mode: c_int,
            n_canvas_width: c_int,
            n_canvas_height: c_int,
            n_tile_pos_x: c_int,
            n_tile_pos_y: c_int,
            n_tile_width: c_int,
            n_tile_height: c_int,
        ),
    >,
    /// Views: ids.
    pub get_view_ids: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            p_array: *mut c_int,
            n_size: usize,
        ) -> bool,
    >,
    /// Calc: outline state.
    pub set_outline_state: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            b_column: bool,
            n_level: c_int,
            n_index: c_int,
            b_hidden: bool,
        ),
    >,
    /// Dialogs: paint a window.
    pub paint_window: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_window_id: c_uint,
            p_buffer: *mut u8,
            x: c_int,
            y: c_int,
            width: c_int,
            height: c_int,
        ),
    >,
    /// Dialogs: post an action.
    pub post_window: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_window_id: c_uint,
            n_action: c_int,
            p_data: *const c_char,
        ),
    >,
    /// Dialogs: keyboard input.
    pub post_window_key_event: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_window_id: c_uint,
            n_type: c_int,
            n_char_code: c_int,
            n_key_code: c_int,
        ),
    >,
    /// Dialogs: mouse input.
    pub post_window_mouse_event: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_window_id: c_uint,
            n_type: c_int,
            n_x: c_int,
            n_y: c_int,
            n_count: c_int,
            n_buttons: c_int,
            n_modifier: c_int,
        ),
    >,
    /// Views: UI language.
    pub set_view_language: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_id: c_int,
            language: *const c_char,
        ),
    >,
    /// Dialogs: extended text input.
    pub post_window_ext_text_input_event: Option<
        unsafe extern "C" fn(
            p_this: *mut LibreOfficeKitDocument,
            n_window_id: c_uint,
            n_type: c_int,
            p_text: *const c_char,
        ),
    >,
    /// JSON about a part (Impress: `visible`, `selected`, size, name) as a malloc'd string.
    pub get_part_info: Option<
        unsafe extern "C" fn(p_this: *mut LibreOfficeKitDocument, n_part: c_int) -> *mut c_char,
    >,
}

/// `LibreOfficeKitTileMode`: pixels are `R,G,B,A` bytes.
pub const LOK_TILEMODE_RGBA: c_int = 0;
/// `LibreOfficeKitTileMode`: pixels are `B,G,R,A` bytes.
pub const LOK_TILEMODE_BGRA: c_int = 1;

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
        // Offsets observed with `offsetof` against the vendored header (tools/lo-tree/dbg/paint.c).
        assert_eq!(offset_of!(LibreOfficeKitDocumentClass, get_parts), 4 * PTR);
        assert_eq!(offset_of!(LibreOfficeKitDocumentClass, set_part), 7 * PTR);
        assert_eq!(
            offset_of!(LibreOfficeKitDocumentClass, paint_tile),
            10 * PTR
        );
        assert_eq!(
            offset_of!(LibreOfficeKitDocumentClass, get_tile_mode),
            11 * PTR
        );
        assert_eq!(
            offset_of!(LibreOfficeKitDocumentClass, get_document_size),
            12 * PTR
        );
        assert_eq!(
            offset_of!(LibreOfficeKitDocumentClass, initialize_for_rendering),
            13 * PTR
        );
        assert_eq!(
            offset_of!(LibreOfficeKitDocumentClass, paint_part_tile),
            33 * PTR
        );
        assert_eq!(
            offset_of!(LibreOfficeKitDocumentClass, get_part_info),
            42 * PTR
        );
        assert_eq!(size_of::<LibreOfficeKitDocumentClass>(), 43 * PTR);
    }

    #[test]
    fn has_member_is_inclusive_of_last_byte() {
        assert!(has_member(2 * PTR, PTR, 3 * PTR));
        assert!(!has_member(2 * PTR, PTR, 3 * PTR - 1));
        assert!(!has_member(2 * PTR, PTR, 0));
    }
}
