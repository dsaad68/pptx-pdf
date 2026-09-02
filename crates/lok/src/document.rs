//! A loaded document and its export entry point.

use std::ffi::{CString, c_int};
use std::ptr::NonNull;

use crate::error::Error;
use crate::office::Office;
use crate::pdf::PdfExportOptions;
use crate::{Result, ffi};

/// Which LibreOffice application owns a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DocumentType {
    /// Writer.
    Text,
    /// Calc.
    Spreadsheet,
    /// Impress.
    Presentation,
    /// Draw.
    Drawing,
    /// Anything else, or a build that cannot report the type.
    Other,
}

impl DocumentType {
    fn from_raw(raw: c_int) -> Self {
        match raw {
            ffi::LOK_DOCTYPE_TEXT => Self::Text,
            ffi::LOK_DOCTYPE_SPREADSHEET => Self::Spreadsheet,
            ffi::LOK_DOCTYPE_PRESENTATION => Self::Presentation,
            ffi::LOK_DOCTYPE_DRAWING => Self::Drawing,
            _ => Self::Other,
        }
    }
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Text => "text",
            Self::Spreadsheet => "spreadsheet",
            Self::Presentation => "presentation",
            Self::Drawing => "drawing",
            Self::Other => "other",
        })
    }
}

/// An open document. Closed on drop; cannot outlive the [`Office`] that loaded it.
pub struct Document<'o> {
    ptr: NonNull<ffi::LibreOfficeKitDocument>,
    office: &'o Office,
}

impl std::fmt::Debug for Document<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("ptr", &self.ptr)
            .finish_non_exhaustive()
    }
}

impl<'o> Document<'o> {
    /// # Safety
    ///
    /// `ptr` must have been returned by `office`'s `documentLoad` and not yet destroyed.
    pub(crate) unsafe fn from_raw(
        ptr: NonNull<ffi::LibreOfficeKitDocument>,
        office: &'o Office,
    ) -> Self {
        Self { ptr, office }
    }

    /// The application that owns this document.
    #[must_use]
    pub fn document_type(&self) -> DocumentType {
        let offset = std::mem::offset_of!(ffi::LibreOfficeKitDocumentClass, get_document_type);
        if !ffi::has_member(offset, size_of::<usize>(), self.class().n_size) {
            return DocumentType::Other;
        }
        let Some(f) = self.class().get_document_type else {
            return DocumentType::Other;
        };
        // SAFETY: `ptr` is a live document on the creating thread.
        DocumentType::from_raw(unsafe { f(self.ptr.as_ptr()) })
    }

    /// Export to `url` (a `file://` URL) in `format` — the LOK short name such as `"pdf"` —
    /// with an optional filter-options string.
    ///
    /// # Errors
    ///
    /// [`Error::SaveAs`] with LibreOffice's message when the export filter fails;
    /// [`Error::InteriorNul`] if any argument contains a NUL byte.
    pub fn save_as(&self, url: &str, format: &str, filter_options: Option<&str>) -> Result<()> {
        let save = self.class().save_as.ok_or(Error::AbiTooOld("saveAs"))?;
        let c_url = c_string(url)?;
        let c_format = c_string(format)?;
        let c_opts = filter_options.map(c_string).transpose()?;
        let opts_ptr = c_opts.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());
        tracing::debug!(
            url,
            format,
            options = filter_options.unwrap_or(""),
            "saveAs"
        );
        // SAFETY: `ptr` is live; all C strings outlive the call; a null options pointer is
        // explicitly permitted by the API.
        let ok = unsafe {
            save(
                self.ptr.as_ptr(),
                c_url.as_ptr(),
                c_format.as_ptr(),
                opts_ptr,
            )
        };
        if ok == 0 {
            Err(Error::SaveAs(self.office.take_error()))
        } else {
            Ok(())
        }
    }

    /// [`Self::save_as`] with `"pdf"` and the given options.
    ///
    /// # Errors
    ///
    /// As for [`Self::save_as`].
    pub fn export_pdf(&self, url: &str, options: &PdfExportOptions) -> Result<()> {
        let filter = options.to_filter_options();
        self.save_as(url, "pdf", (!filter.is_empty()).then_some(filter.as_str()))
    }

    fn class(&self) -> &ffi::LibreOfficeKitDocumentClass {
        // SAFETY: `ptr` is a live document whose `p_class` is set for its lifetime.
        unsafe { &*(*self.ptr.as_ptr()).p_class }
    }
}

impl Drop for Document<'_> {
    fn drop(&mut self) {
        if let Some(destroy) = self.class().destroy {
            // SAFETY: `ptr` is live and is not used after this call.
            unsafe { destroy(self.ptr.as_ptr()) };
        }
    }
}

fn c_string(s: &str) -> Result<CString> {
    CString::new(s).map_err(|_| Error::InteriorNul(s.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_known_and_unknown_doc_types() {
        assert_eq!(
            DocumentType::from_raw(ffi::LOK_DOCTYPE_PRESENTATION),
            DocumentType::Presentation
        );
        assert_eq!(
            DocumentType::from_raw(ffi::LOK_DOCTYPE_OTHER),
            DocumentType::Other
        );
        assert_eq!(DocumentType::from_raw(99), DocumentType::Other);
        assert_eq!(DocumentType::from_raw(-1), DocumentType::Other);
    }
}
