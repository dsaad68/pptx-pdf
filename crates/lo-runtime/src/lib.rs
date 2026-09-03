//! Finds a LibreOffice runtime tree for [`lok`](https://docs.rs/lok) to load, and prepares
//! the per-run user profile LibreOffice needs.
//!
//! Resolution order in [`Runtime::locate`]:
//!
//! 1. An explicit directory (the CLI's `--lo-dir`).
//! 2. The [`ENV_LO_DIR`] environment variable.
//! 3. With the `embedded` feature: the tree compiled into the binary, unpacked to the
//!    user's cache directory on first use.
//! 4. With the `download` feature: the tree fetched over HTTPS from the [`TreeSource`] the
//!    caller supplies, verified, and unpacked into the same cache.
//!
//! [`Runtime::configure_fontconfig`] makes the platform's fonts visible to LibreOffice; without
//! it headless LibreOffice sees only its bundled fonts and substitutes silently.
//!
//! A "tree" is any directory LibreOffice's own bootstrap accepts: an `instdir`-style layout
//! with `program/` and `share/`, a Debian `/usr/lib/libreoffice`, or a macOS `.app` bundle.

pub mod cache;
#[cfg(feature = "download")]
mod download;
#[cfg(feature = "embedded")]
mod embedded;
mod error;
mod fontconfig;
mod locate;
mod profile;
#[cfg(any(feature = "embedded", feature = "download"))]
mod unpack;
mod url;

#[cfg(feature = "download")]
pub use download::{Progress, TreeSource, fetch};
pub use error::Error;
pub use fontconfig::FontConfig;
pub use locate::{ENV_LO_DIR, Locator, Runtime};
pub use profile::Profile;
pub use url::file_url;

/// Convenience alias for results in this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;
