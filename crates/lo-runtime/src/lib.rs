//! Finds a LibreOffice runtime tree for [`lok`](https://docs.rs/lok) to load, and prepares
//! the per-run user profile LibreOffice needs.
//!
//! Resolution order in [`Runtime::locate`]:
//!
//! 1. An explicit directory (the CLI's `--lo-dir`).
//! 2. The [`ENV_LO_DIR`] environment variable.
//! 3. With the `embedded` feature: the tree compiled into the binary, unpacked to the
//!    user's cache directory on first use.
//!
//! A "tree" is any directory LibreOffice's own bootstrap accepts: an `instdir`-style layout
//! with `program/` and `share/`, a Debian `/usr/lib/libreoffice`, or a macOS `.app` bundle.

#[cfg(feature = "embedded")]
mod embedded;
mod error;
mod locate;
mod profile;
mod url;

pub use error::Error;
pub use locate::{ENV_LO_DIR, Runtime};
pub use profile::Profile;
pub use url::file_url;

/// Convenience alias for results in this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;
