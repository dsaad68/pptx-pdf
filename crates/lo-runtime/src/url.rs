//! `file://` URLs in the form LibreOffice accepts.

use std::path::{Component, Path, PathBuf};

use crate::Result;
use crate::error::Error;

/// Build an absolute `file://` URL for `path`. Relative paths are resolved against the
/// current directory; `.` and `..` are folded lexically so the target need not exist yet.
///
/// # Errors
///
/// [`Error::Io`] if the current directory cannot be read for a relative path.
pub fn file_url(path: &Path) -> Result<String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(Error::io("reading current directory", path))?
            .join(path)
    };
    let normalised = normalise(&absolute);
    let mut url = String::from("file://");
    for component in normalised.components() {
        match component {
            Component::RootDir => {}
            Component::Prefix(_) | Component::CurDir | Component::ParentDir => continue,
            Component::Normal(seg) => {
                url.push('/');
                percent_encode(seg.as_encoded_bytes(), &mut url);
            }
        }
    }
    if url == "file://" {
        url.push('/');
    }
    Ok(url)
}

fn normalise(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => out.push(component),
        }
    }
    out
}

/// RFC 3986 unreserved characters pass through; everything else, including UTF-8 bytes, is
/// `%XX`-encoded.
fn percent_encode(bytes: &[u8], out: &mut String) {
    for &b in bytes {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_absolute_path() {
        assert_eq!(
            file_url(Path::new("/tmp/a b/deck.pptx")).unwrap(),
            "file:///tmp/a%20b/deck.pptx"
        );
    }

    #[test]
    fn encodes_unicode_and_reserved_characters() {
        assert_eq!(
            file_url(Path::new("/x/ä#?.pptx")).unwrap(),
            "file:///x/%C3%A4%23%3F.pptx"
        );
    }

    #[test]
    fn folds_dot_and_dotdot() {
        assert_eq!(
            file_url(Path::new("/a/b/../c/./d")).unwrap(),
            "file:///a/c/d"
        );
    }

    #[test]
    fn resolves_relative_against_cwd() {
        let cwd = std::env::current_dir().unwrap();
        let expected = file_url(&cwd.join("x.pptx")).unwrap();
        assert_eq!(file_url(Path::new("x.pptx")).unwrap(), expected);
    }

    #[test]
    fn root_is_a_single_slash() {
        assert_eq!(file_url(Path::new("/")).unwrap(), "file:///");
    }
}
