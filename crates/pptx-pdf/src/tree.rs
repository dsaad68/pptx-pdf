//! Where a slim build fetches its LibreOffice tree, and how the download is shown.

use lo_runtime::Locator;

#[cfg(feature = "download")]
/// The release asset for this platform, or `None` when no tree is published for it.
#[must_use]
pub(crate) fn platform_asset() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "aarch64") => Some("linux-arm64-portable"),
        ("linux", "x86_64") => Some("linux-amd64-portable"),
        ("macos", "aarch64") => Some("macos-arm64"),
        _ => None,
    }
}

#[cfg(feature = "download")]
/// `<repository>/releases/download/v<version>/lo-tree-<platform>-v<version>.tar.zst`.
#[must_use]
pub(crate) fn default_url(platform: &str) -> String {
    let repo = env!("CARGO_PKG_REPOSITORY").trim_end_matches('/');
    let version = env!("CARGO_PKG_VERSION");
    format!("{repo}/releases/download/v{version}/lo-tree-{platform}-v{version}.tar.zst")
}

#[cfg(feature = "download")]
/// Hash pinned at build time (`PPTX_PDF_LO_TREE_SHA256`), if any.
#[must_use]
pub(crate) fn pinned_sha256() -> Option<&'static str> {
    option_env!("PPTX_PDF_LO_TREE_SHA256").filter(|s| !s.is_empty())
}

/// Attach the download source and a progress bar to a locator.
#[cfg(feature = "download")]
pub(crate) fn with_download<'a>(
    locator: Locator<'a>,
    url_override: Option<String>,
    sha_override: Option<String>,
    quiet: bool,
) -> Locator<'a> {
    use lo_runtime::{Progress, TreeSource};

    let url = match url_override.or_else(|| platform_asset().map(default_url)) {
        Some(url) => url,
        None => return locator,
    };
    let sha = sha_override.or_else(|| pinned_sha256().map(str::to_owned));
    let source = TreeSource::new(url, sha);
    let bar = progress::Bar::new(quiet);
    locator
        .download(source)
        .on_progress(move |event| match event {
            Progress::Started { total } => bar.start(total),
            Progress::Received(n) => bar.set(n),
            Progress::Unpacking => bar.unpacking(),
            // `Progress` is non_exhaustive in lo-runtime; future events need no UI here.
            other => tracing::trace!(?other, "download event"),
        })
}

#[cfg(not(feature = "download"))]
pub(crate) fn with_download<'a>(
    locator: Locator<'a>,
    _url_override: Option<String>,
    _sha_override: Option<String>,
    _quiet: bool,
) -> Locator<'a> {
    locator
}

#[cfg(feature = "download")]
mod progress {
    use std::time::Duration;

    use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

    /// A download bar on stderr; hidden with `--quiet` or when stderr is not a terminal.
    pub(super) struct Bar {
        bar: ProgressBar,
    }

    impl Bar {
        pub(super) fn new(quiet: bool) -> Self {
            let target = if quiet {
                ProgressDrawTarget::hidden()
            } else {
                ProgressDrawTarget::stderr()
            };
            Self {
                bar: ProgressBar::with_draw_target(None, target),
            }
        }

        pub(super) fn start(&self, total: Option<u64>) {
            let template = match total {
                Some(_) => {
                    "{msg} [{bar:32}] {bytes}/{total_bytes} {binary_bytes_per_sec} eta {eta}"
                }
                None => "{msg} {spinner} {bytes} {binary_bytes_per_sec}",
            };
            let style = ProgressStyle::with_template(template)
                .unwrap_or_else(|_| ProgressStyle::default_bar())
                .progress_chars("=> ")
                .tick_chars("|/-\\ ");
            self.bar.set_style(style);
            self.bar.set_length(total.unwrap_or(0));
            self.bar.set_message("downloading LibreOffice");
            self.bar.enable_steady_tick(Duration::from_millis(120));
        }

        pub(super) fn set(&self, received: u64) {
            self.bar.set_position(received);
        }

        pub(super) fn unpacking(&self) {
            let style = ProgressStyle::with_template("{msg} {spinner}")
                .unwrap_or_else(|_| ProgressStyle::default_spinner())
                .tick_chars("|/-\\ ");
            self.bar.set_style(style);
            self.bar.set_message("unpacking LibreOffice");
        }
    }

    impl Drop for Bar {
        fn drop(&mut self) {
            self.bar.finish_and_clear();
        }
    }
}

#[cfg(all(test, feature = "download"))]
mod tests {
    use super::*;

    #[test]
    fn default_url_points_at_this_release() {
        let url = default_url("macos-arm64");
        assert!(url.starts_with("https://github.com/"), "{url}");
        assert!(
            url.ends_with(&format!(
                "/releases/download/v{v}/lo-tree-macos-arm64-v{v}.tar.zst",
                v = env!("CARGO_PKG_VERSION")
            )),
            "{url}"
        );
    }

    #[test]
    fn platform_asset_matches_this_host_or_is_none() {
        let asset = platform_asset();
        if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            assert_eq!(asset, Some("macos-arm64"));
        }
        if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            assert_eq!(asset, Some("linux-amd64-portable"));
        }
    }
}
