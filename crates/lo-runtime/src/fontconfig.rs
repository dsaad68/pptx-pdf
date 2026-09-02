//! Point fontconfig at the fonts LibreOffice should see.
//!
//! In headless mode LibreOffice resolves fonts through fontconfig on every platform, macOS
//! included. Without a configuration it sees only the fonts it bundles and silently
//! substitutes for everything else; the bundled `libfontconfig` also warns about a missing
//! cache directory on every run. So unless the tree ships its own `etc/fonts` (the Linux
//! trees do) or the user has set `FONTCONFIG_FILE`/`FONTCONFIG_PATH`, a small configuration is
//! generated per run listing LibreOffice's font directory, the platform's system font
//! directories, the profile's `user/fonts` (where `--font-dir` fonts go) and a cache directory.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::locate::Runtime;
use crate::profile::Profile;
use crate::{Result, cache};

/// How fontconfig ended up configured for this run.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FontConfig {
    /// The user had already set `FONTCONFIG_FILE` or `FONTCONFIG_PATH`; left untouched.
    External,
    /// The tree carries `etc/fonts`; `FONTCONFIG_PATH` points there.
    Bundled(PathBuf),
    /// A `fonts.conf` was written into the profile; `FONTCONFIG_FILE` points at it.
    Generated(PathBuf),
}

/// Font directories relative to an installation root, in the layouts we know.
const TREE_FONT_DIRS: [&str; 2] = ["share/fonts/truetype", "Contents/Resources/fonts/truetype"];

#[cfg(target_os = "macos")]
const SYSTEM_FONT_DIRS: [&str; 4] = [
    "/System/Library/Fonts",
    "/System/Library/Fonts/Supplemental",
    "/Library/Fonts",
    "~/Library/Fonts",
];
#[cfg(not(target_os = "macos"))]
const SYSTEM_FONT_DIRS: [&str; 4] = [
    "/usr/share/fonts",
    "/usr/local/share/fonts",
    "~/.local/share/fonts",
    "~/.fonts",
];

impl Runtime {
    /// Configure fontconfig for this run. See the module documentation.
    ///
    /// Must be called before LibreOffice is loaded and while the process is still
    /// single-threaded: it sets an environment variable, which is only sound then.
    ///
    /// # Errors
    ///
    /// [`Error::Io`] if the generated configuration cannot be written;
    /// [`Error::NoCacheDir`] if no cache directory can be determined for fontconfig's cache.
    pub fn configure_fontconfig(&self, profile: &Profile) -> Result<FontConfig> {
        if std::env::var_os("FONTCONFIG_FILE").is_some()
            || std::env::var_os("FONTCONFIG_PATH").is_some()
        {
            return Ok(FontConfig::External);
        }
        let bundled = self.install_root().join("etc").join("fonts");
        if bundled.join("fonts.conf").is_file() {
            tracing::debug!(dir = %bundled.display(), "using bundled fontconfig configuration");
            // SAFETY: documented precondition — called before any thread is spawned and
            // before LibreOffice (which reads the variable lazily) is loaded.
            unsafe { std::env::set_var("FONTCONFIG_PATH", &bundled) };
            return Ok(FontConfig::Bundled(bundled));
        }
        let cache = cache::cache_dir()?.join("fontconfig");
        std::fs::create_dir_all(&cache)
            .map_err(Error::io("creating fontconfig cache directory", &cache))?;
        let conf = profile.dir().join("fonts.conf");
        let text = render(&self.font_dirs(), profile.fonts_dir().as_path(), &cache);
        std::fs::write(&conf, text)
            .map_err(Error::io("writing fontconfig configuration", &conf))?;
        tracing::debug!(file = %conf.display(), "generated fontconfig configuration");
        // SAFETY: as above.
        unsafe { std::env::set_var("FONTCONFIG_FILE", &conf) };
        Ok(FontConfig::Generated(conf))
    }

    /// The tree's own font directories that exist.
    fn font_dirs(&self) -> Vec<PathBuf> {
        TREE_FONT_DIRS
            .iter()
            .map(|d| self.install_root().join(d))
            .filter(|d| d.is_dir())
            .collect()
    }
}

/// Build the `fonts.conf` text. `~/` in the system directories is fontconfig's own notation
/// and is passed through; everything else is XML-escaped.
fn render(tree_dirs: &[PathBuf], profile_fonts: &Path, cache: &Path) -> String {
    let mut out = String::from(
        "<?xml version=\"1.0\"?>\n<!DOCTYPE fontconfig SYSTEM \"fonts.dtd\">\n<fontconfig>\n",
    );
    for dir in tree_dirs {
        let _ = writeln!(out, "  <dir>{}</dir>", xml_escape(&dir.to_string_lossy()));
    }
    let _ = writeln!(
        out,
        "  <dir>{}</dir>",
        xml_escape(&profile_fonts.to_string_lossy())
    );
    for dir in SYSTEM_FONT_DIRS {
        let _ = writeln!(out, "  <dir>{}</dir>", xml_escape(dir));
    }
    let _ = writeln!(
        out,
        "  <cachedir>{}</cachedir>",
        xml_escape(&cache.to_string_lossy())
    );
    out.push_str("</fontconfig>\n");
    out
}

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_all_directories_and_cache() {
        let text = render(
            &[PathBuf::from("/lo/share/fonts/truetype")],
            Path::new("/p/user/fonts"),
            Path::new("/c/fc"),
        );
        assert!(text.starts_with("<?xml"));
        assert!(text.contains("<dir>/lo/share/fonts/truetype</dir>"));
        assert!(text.contains("<dir>/p/user/fonts</dir>"));
        assert!(text.contains("<cachedir>/c/fc</cachedir>"));
        for d in SYSTEM_FONT_DIRS {
            assert!(text.contains(&format!("<dir>{d}</dir>")), "{d}");
        }
        assert!(text.trim_end().ends_with("</fontconfig>"));
    }

    #[test]
    fn escapes_xml_in_paths() {
        let text = render(&[], Path::new("/a&b/<x>"), Path::new("/c"));
        assert!(text.contains("<dir>/a&amp;b/&lt;x&gt;</dir>"));
    }
}
