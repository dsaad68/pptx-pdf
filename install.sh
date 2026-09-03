#!/bin/sh
# Install pptx-pdf: downloads the prebuilt, self-contained binary for this machine from the
# latest GitHub release, verifies its SHA-256 against the release's SHA256SUMS, and puts it in
# ~/.local/bin (override with PPTX_PDF_INSTALL_DIR; pin a version with PPTX_PDF_VERSION=0.2.1).
# PPTX_PDF_SLIM=1 installs the ~4 MB variant that downloads LibreOffice on first run instead
# of carrying it inside the binary.
#
#   curl -fsSL https://raw.githubusercontent.com/dsaad68/pptx-pdf/main/install.sh | sh
#   curl -fsSL https://raw.githubusercontent.com/dsaad68/pptx-pdf/main/install.sh | PPTX_PDF_SLIM=1 sh
set -eu
REPO=dsaad68/pptx-pdf
case "$(uname -s)-$(uname -m)" in
  Darwin-arm64)  target=aarch64-apple-darwin ;;
  Linux-aarch64) target=aarch64-unknown-linux-gnu ;;
  Linux-x86_64)  target=x86_64-unknown-linux-gnu ;;
  *) echo "pptx-pdf: no prebuilt binary for $(uname -s)/$(uname -m); see https://github.com/$REPO#building" >&2; exit 1 ;;
esac
version=${PPTX_PDF_VERSION:-$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | sed -n 's/.*"tag_name": *"v\([^"]*\)".*/\1/p')}
[ -n "$version" ] || { echo "pptx-pdf: could not determine the latest version" >&2; exit 1; }
name=pptx-pdf; [ "${PPTX_PDF_SLIM:-0}" = "1" ] && name=pptx-pdf-slim
asset="$name-v$version-$target.tar.gz"
base="https://github.com/$REPO/releases/download/v$version"
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
echo "downloading $asset"
curl -fsSL --progress-bar "$base/$asset" -o "$tmp/$asset"
curl -fsSL "$base/SHA256SUMS" -o "$tmp/SHA256SUMS"
expected=$(grep " $asset\$" "$tmp/SHA256SUMS" | cut -d' ' -f1)
if command -v sha256sum >/dev/null 2>&1; then actual=$(sha256sum "$tmp/$asset" | cut -d' ' -f1); else actual=$(shasum -a 256 "$tmp/$asset" | cut -d' ' -f1); fi
[ "$expected" = "$actual" ] || { echo "pptx-pdf: checksum mismatch for $asset" >&2; exit 1; }
dest=${PPTX_PDF_INSTALL_DIR:-$HOME/.local/bin}
mkdir -p "$dest"
tar -xzf "$tmp/$asset" -C "$dest" pptx-pdf
chmod +x "$dest/pptx-pdf"
echo "installed $dest/pptx-pdf ($("$dest/pptx-pdf" --version))"
case ":$PATH:" in *":$dest:"*) ;; *) echo "note: $dest is not on your PATH" ;; esac
