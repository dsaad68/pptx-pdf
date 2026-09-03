//! Fetching the LibreOffice tree over HTTPS on first run — the "slim" binary.
//!
//! Integrity: the tarball's SHA-256 is computed while it streams to disk and compared with
//! either a hash pinned into the binary at build time ([`TreeSource::sha256`]) or, when there
//! is no pin (a `cargo install` from crates.io), the `SHA256SUMS` file published next to the
//! tarball in the same release. A pin makes the download exactly as trustworthy as the binary;
//! `SHA256SUMS` protects against corruption and mismatched assets but, like the installer
//! script, ultimately trusts the release host over TLS.
//!
//! The unpacked tree lands in the same cache directory the embedded variant uses, keyed by the
//! tarball's hash, so an embedded and a slim binary built from the same tree share one copy.

use std::io::{Read, Write as _};
use std::path::PathBuf;

use sha2::{Digest as _, Sha256};

use crate::error::Error;
use crate::unpack::{tree_dir_name, unpack_zstd_tar};
use crate::{Result, cache};

/// Where to fetch the tree from and what it must hash to.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct TreeSource {
    /// URL of the `tar.zst` tree.
    pub url: String,
    /// Expected SHA-256 (lowercase hex). `None` means "look it up in `SHA256SUMS` next to
    /// the tarball".
    pub sha256: Option<String>,
}

impl TreeSource {
    /// A source with an optional pinned hash.
    #[must_use]
    pub fn new(url: impl Into<String>, sha256: Option<String>) -> Self {
        Self {
            url: url.into(),
            sha256: sha256.map(|s| s.trim().to_ascii_lowercase()),
        }
    }

    /// The tarball's file name (last path segment of the URL).
    #[must_use]
    pub fn file_name(&self) -> &str {
        self.url.rsplit('/').next().unwrap_or(&self.url)
    }

    /// URL of the `SHA256SUMS` published next to the tarball.
    #[must_use]
    pub fn sums_url(&self) -> String {
        match self.url.rfind('/') {
            Some(i) => format!("{}/SHA256SUMS", self.url.get(..i).unwrap_or_default()),
            None => String::from("SHA256SUMS"),
        }
    }
}

/// Progress events during [`fetch`], for a progress bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Progress {
    /// The download is starting; total size if the server said.
    Started {
        /// Content length, when known.
        total: Option<u64>,
    },
    /// Cumulative bytes received.
    Received(u64),
    /// Download complete and verified; unpacking into the cache.
    Unpacking,
}

/// Ensure the tree from `source` is unpacked in the cache and return its root.
///
/// # Errors
///
/// [`Error::Http`] for transport or non-2xx responses; [`Error::ChecksumUnavailable`] when
/// no pin was given and `SHA256SUMS` has no entry for the file; [`Error::ChecksumMismatch`]
/// when the download does not hash to the expected value (nothing is left in the cache);
/// [`Error::NoCacheDir`] / [`Error::Io`] for filesystem trouble.
pub fn fetch(source: &TreeSource, mut progress: impl FnMut(Progress)) -> Result<PathBuf> {
    let cache = cache::cache_dir()?;
    std::fs::create_dir_all(&cache).map_err(Error::io("creating cache directory", &cache))?;
    // Without a pin the hash comes from SHA256SUMS; remember it per URL so later runs need
    // no network at all.
    let remembered = cache
        .join("sources")
        .join(hex(&Sha256::digest(source.url.as_bytes())));
    let expected = match &source.sha256 {
        Some(pin) => pin.clone(),
        None => match std::fs::read_to_string(&remembered) {
            Ok(text) if text.trim().len() == 64 => text.trim().to_owned(),
            _ => lookup_sum(source)?,
        },
    };
    let root = cache.join(tree_dir_name(&expected));
    if root.is_dir() {
        remember(&remembered, &expected);
        return Ok(root);
    }
    let tarball = cache.join(format!(
        "{}.part-{}",
        source.file_name(),
        std::process::id()
    ));
    let result = download_verified(source, &expected, &tarball, &mut progress).and_then(|()| {
        progress(Progress::Unpacking);
        let file = std::fs::File::open(&tarball)
            .map_err(Error::io("opening downloaded tree", &tarball))?;
        unpack_zstd_tar(std::io::BufReader::new(file), &root)
    });
    let _ = std::fs::remove_file(&tarball);
    if result.is_ok() {
        remember(&remembered, &expected);
    }
    result
}

/// Best-effort note of which tree hash a URL resolved to.
fn remember(path: &std::path::Path, sha256: &str) {
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(path, sha256);
}

/// Read `SHA256SUMS` next to the tarball and return the hash listed for its file name.
fn lookup_sum(source: &TreeSource) -> Result<String> {
    let url = source.sums_url();
    let text = get(&url)?
        .body_mut()
        .read_to_string()
        .map_err(|source| Error::Http {
            url: url.clone(),
            source,
        })?;
    parse_sums(&text, source.file_name()).ok_or_else(|| Error::ChecksumUnavailable {
        file: source.file_name().to_owned(),
        url,
    })
}

/// Find `<hex>  <name>` (or `<hex> *<name>`) in a `sha256sum`-style listing.
fn parse_sums(text: &str, name: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let mut parts = line.split_whitespace();
        let hash = parts.next()?;
        let file = parts.next()?.trim_start_matches('*');
        (file == name && hash.len() == 64 && hash.bytes().all(|b| b.is_ascii_hexdigit()))
            .then(|| hash.to_ascii_lowercase())
    })
}

/// Stream the tarball to `dest`, hashing as it goes; delete it on any mismatch or failure.
fn download_verified(
    source: &TreeSource,
    expected: &str,
    dest: &std::path::Path,
    progress: &mut impl FnMut(Progress),
) -> Result<()> {
    let mut response = get(&source.url)?;
    progress(Progress::Started {
        total: response.body().content_length(),
    });
    // ureq caps bodies at 10 MB by default; the tree is far larger.
    let mut reader = response.body_mut().with_config().limit(u64::MAX).reader();
    let mut file =
        std::fs::File::create(dest).map_err(Error::io("creating download file", dest))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 16];
    let mut received = 0u64;
    let outcome = loop {
        let n = match reader.read(&mut buf) {
            Ok(0) => break Ok(()),
            Ok(n) => n,
            Err(e) => break Err(Error::io("reading download", dest)(e)),
        };
        let chunk = buf.get(..n).unwrap_or_default();
        hasher.update(chunk);
        if let Err(e) = file.write_all(chunk) {
            break Err(Error::io("writing download", dest)(e));
        }
        received += n as u64;
        progress(Progress::Received(received));
    };
    outcome?;
    file.flush().map_err(Error::io("writing download", dest))?;
    let actual = hex(&hasher.finalize());
    if actual != expected {
        let _ = std::fs::remove_file(dest);
        return Err(Error::ChecksumMismatch {
            file: source.file_name().to_owned(),
            expected: expected.to_owned(),
            actual,
        });
    }
    Ok(())
}

fn get(url: &str) -> Result<ureq::http::Response<ureq::Body>> {
    tracing::debug!(url, "GET");
    ureq::get(url)
        .header(
            "User-Agent",
            concat!("pptx-pdf/", env!("CARGO_PKG_VERSION")),
        )
        .call()
        .map_err(|source| Error::Http {
            url: url.to_owned(),
            source,
        })
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead as _, BufReader};
    use std::net::TcpListener;

    #[test]
    fn source_derives_file_name_and_sums_url() {
        let s = TreeSource::new(
            "https://x.example/r/v1/lo-tree-linux-arm64-v1.tar.zst",
            Some("ABC".into()),
        );
        assert_eq!(s.file_name(), "lo-tree-linux-arm64-v1.tar.zst");
        assert_eq!(s.sums_url(), "https://x.example/r/v1/SHA256SUMS");
        assert_eq!(s.sha256.as_deref(), Some("abc"));
    }

    #[test]
    fn parses_sha256sums_formats() {
        let text = "0011aa  a.tar.zst\n\
                    ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff *b.tar.zst\n\
                    ABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCD  c.tar.zst\n";
        assert_eq!(parse_sums(text, "a.tar.zst"), None, "too short");
        assert_eq!(parse_sums(text, "b.tar.zst").unwrap().len(), 64);
        assert!(parse_sums(text, "c.tar.zst").unwrap().starts_with("abcdef"));
        assert_eq!(parse_sums(text, "missing"), None);
    }

    /// A one-route-per-connection HTTP/1.1 server on localhost for the download tests.
    fn serve(routes: Vec<(&'static str, u16, Vec<u8>)>) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let mut stream = stream;
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                let path = line.split_whitespace().nth(1).unwrap_or("/").to_owned();
                loop {
                    let mut h = String::new();
                    if reader.read_line(&mut h).unwrap() == 0 || h == "\r\n" {
                        break;
                    }
                }
                let (status, body) = routes
                    .iter()
                    .find(|(p, _, _)| *p == path)
                    .map_or((404, Vec::new()), |(_, s, b)| (*s, b.clone()));
                let head = format!(
                    "HTTP/1.1 {status} X\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                stream.write_all(head.as_bytes()).unwrap();
                stream.write_all(&body).unwrap();
            }
        });
        base
    }

    fn tiny_tree() -> (Vec<u8>, String) {
        let mut tar = tar::Builder::new(Vec::new());
        let data = b"marker";
        let mut header = tar::Header::new_gnu();
        header.set_size(data.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "program/marker", &data[..])
            .unwrap();
        let tar = tar.into_inner().unwrap();
        let bytes = zstd::encode_all(&tar[..], 3).unwrap();
        let sha = hex(&Sha256::digest(&bytes));
        (bytes, sha)
    }

    fn with_cache<T>(name: &str, f: impl FnOnce() -> T) -> T {
        let dir = std::env::temp_dir().join(format!("lo-runtime-dl-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        // SAFETY: tests in this module are the only writers of this variable and run in
        // sequence under the lock below.
        let _guard = ENV_LOCK.lock().unwrap();
        unsafe { std::env::set_var(cache::ENV_CACHE_DIR, &dir) };
        let out = f();
        unsafe { std::env::remove_var(cache::ENV_CACHE_DIR) };
        out
    }
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn downloads_verifies_and_unpacks_with_pin() {
        let (bytes, sha) = tiny_tree();
        let base = serve(vec![("/t.tar.zst", 200, bytes)]);
        with_cache("pin", || {
            let mut events = Vec::new();
            let root = fetch(
                &TreeSource::new(format!("{base}/t.tar.zst"), Some(sha.clone())),
                |p| events.push(p),
            )
            .unwrap();
            assert!(root.join("program/marker").is_file());
            assert!(root.ends_with(tree_dir_name(&sha)));
            assert!(matches!(
                events.first(),
                Some(Progress::Started { total: Some(_) })
            ));
            assert!(events.contains(&Progress::Unpacking));
            // second call: cached, no network needed
            let again = fetch(
                &TreeSource::new("http://127.0.0.1:9/nope.tar.zst", Some(sha)),
                |_| {},
            )
            .unwrap();
            assert_eq!(again, root);
        });
    }

    #[test]
    fn looks_up_sha256sums_when_unpinned() {
        let (bytes, sha) = tiny_tree();
        let sums = format!("{sha}  t.tar.zst\n");
        let base = serve(vec![
            ("/t.tar.zst", 200, bytes),
            ("/SHA256SUMS", 200, sums.into_bytes()),
        ]);
        with_cache("sums", || {
            let url = format!("{base}/t.tar.zst");
            let root = fetch(&TreeSource::new(&url, None), |_| {}).unwrap();
            assert!(root.join("program/marker").is_file());
            // the resolved hash is remembered per URL: a later run needs no SHA256SUMS fetch
            let remembered = cache::cache_dir()
                .unwrap()
                .join("sources")
                .join(hex(&Sha256::digest(url.as_bytes())));
            assert_eq!(std::fs::read_to_string(remembered).unwrap().len(), 64);
        });
    }

    #[test]
    fn rejects_checksum_mismatch_and_leaves_no_cache() {
        let (bytes, _) = tiny_tree();
        let base = serve(vec![("/t.tar.zst", 200, bytes)]);
        with_cache("bad", || {
            let bad = "0".repeat(64);
            let err = fetch(
                &TreeSource::new(format!("{base}/t.tar.zst"), Some(bad.clone())),
                |_| {},
            )
            .unwrap_err();
            assert!(matches!(err, Error::ChecksumMismatch { .. }), "{err}");
            let cache = cache::cache_dir().unwrap();
            assert!(!cache.join(tree_dir_name(&bad)).exists());
            assert!(
                std::fs::read_dir(&cache)
                    .map(|d| d.count() == 0)
                    .unwrap_or(true),
                "leftover files"
            );
        });
    }

    #[test]
    fn reports_missing_sum_entry_and_http_errors() {
        let base = serve(vec![(
            "/SHA256SUMS",
            200,
            b"deadbeef  other.tar.zst\n".to_vec(),
        )]);
        with_cache("http", || {
            let err =
                fetch(&TreeSource::new(format!("{base}/t.tar.zst"), None), |_| {}).unwrap_err();
            assert!(matches!(err, Error::ChecksumUnavailable { .. }), "{err}");
            let err = fetch(
                &TreeSource::new(format!("{base}/missing.tar.zst"), Some("0".repeat(64))),
                |_| {},
            )
            .unwrap_err();
            assert!(matches!(err, Error::Http { .. }), "{err}");
        });
    }
}
