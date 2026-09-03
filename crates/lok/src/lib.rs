//! Minimal, safe binding to [LibreOfficeKit] (LOK) for headless document conversion.
//!
//! LOK is LibreOffice's stable C ABI for embedding. This crate loads it at runtime with
//! `dlopen` from a directory containing `libsofficeapp` / `libmergedlo`, so there is no
//! link-time dependency on LibreOffice and no C compiler involved.
//!
//! The surface is deliberately small: boot an [`Office`], load a [`Document`], call
//! [`Document::save_as`] or [`Document::render_slides`]. Two constraints inherited from LOK itself are encoded in the types:
//!
//! - **One [`Office`] per process, ever.** LOK's own header says a second instance is not
//!   supported, not even after destroying the first. [`Office::load`] enforces this with a
//!   process-wide flag.
//! - **Single-threaded.** LOK guards its internals with LibreOffice's global "solar mutex" and
//!   assumes one calling thread. [`Office`] is `!Send` so it cannot leave the thread that
//!   created it.
//!
//! [LibreOfficeKit]: https://docs.libreoffice.org/libreofficekit.html

mod document;
mod error;
pub mod ffi;
mod office;
mod pdf;
mod render;

pub use document::{Document, DocumentType};
pub use error::Error;
pub use office::Office;
pub use pdf::{PageRange, PdfExportOptions};
pub use render::{Raster, RenderOptions};

/// Convenience alias for results in this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;
