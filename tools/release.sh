#!/bin/bash
# Package the verified embedded binaries as release assets and publish a GitHub release.
#
#   tools/release.sh v0.1.0 [--draft]
#
# Expects the binaries the e2e suites built and tested:
#   target-linux-arm64-embedded/release/pptx-pdf   (tools/e2e-embedded.sh, PPTX_PDF_ARCH=arm64)
#   target-linux-amd64-embedded/release/pptx-pdf   (tools/e2e-embedded.sh, PPTX_PDF_ARCH=amd64)
#   target-macos-embedded/release/pptx-pdf         (tools/e2e-macos.sh)
# plus the trees under tools/lo-tree/out, from which the slim binaries are built here with
# their hash pinned. Asset names follow the cargo-binstall template in crates/pptx-pdf/Cargo.toml.
# Only the current host's slim binary can be built locally; CI builds all of them.
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd); cd "$repo"
tag=${1:?usage: tools/release.sh vX.Y.Z [--draft]}; version=${tag#v}
extra=${2:-}
grep -q "^version = \"$version\"" crates/pptx-pdf/Cargo.toml || { echo "crates/pptx-pdf/Cargo.toml is not at version $version"; exit 1; }
dist=$repo/dist; rm -rf "$dist"; mkdir -p "$dist"
package() { # <target> <binary>
  [ -x "$2" ] || { echo "missing $2"; exit 1; }
  local asset="pptx-pdf-$tag-$1.tar.gz" stage; stage=$(mktemp -d)
  cp "$2" "$stage/pptx-pdf"; cp LICENSE-MIT LICENSE-APACHE README.md "$stage/"
  tar -czf "$dist/$asset" -C "$stage" pptx-pdf LICENSE-MIT LICENSE-APACHE README.md; rm -rf "$stage"
  printf "  %-48s %6.1f MB\n" "$asset" "$(echo "$(stat -f %z "$dist/$asset" 2>/dev/null || stat -c %s "$dist/$asset")/1048576" | bc -l)"
}
echo "== assets"
package aarch64-unknown-linux-gnu target-linux-arm64-embedded/release/pptx-pdf
package x86_64-unknown-linux-gnu  target-linux-amd64-embedded/release/pptx-pdf
package aarch64-apple-darwin      target-macos-embedded/release/pptx-pdf
echo "== trees"
for t in linux-arm64-portable linux-amd64-portable macos-arm64; do
  src=tools/lo-tree/out/lo-tree-$t.tar.zst; [ -f "$src" ] || { echo "missing $src"; exit 1; }
  cp "$src" "$dist/lo-tree-$t-$tag.tar.zst"; printf "  %-48s %6.1f MB\n" "lo-tree-$t-$tag.tar.zst" "$(echo "$(stat -f %z "$src" 2>/dev/null || stat -c %s "$src")/1048576" | bc -l)"
done
echo "== slim binary for this host (hash pinned)"
host=$(uname -s)-$(uname -m)
case "$host" in Darwin-arm64) tree=macos-arm64; target=aarch64-apple-darwin;; Linux-aarch64) tree=linux-arm64-portable; target=aarch64-unknown-linux-gnu;; Linux-x86_64) tree=linux-amd64-portable; target=x86_64-unknown-linux-gnu;; *) tree=""; esac
if [ -n "$tree" ]; then
  sha=$(shasum -a 256 "$dist/lo-tree-$tree-$tag.tar.zst" | cut -d' ' -f1)
  PPTX_PDF_LO_TREE_SHA256=$sha cargo build --release --locked --target-dir target-slim >/dev/null 2>&1
  stage=$(mktemp -d); cp target-slim/release/pptx-pdf "$stage/"; cp LICENSE-MIT LICENSE-APACHE README.md "$stage/"
  tar -czf "$dist/pptx-pdf-slim-$tag-$target.tar.gz" -C "$stage" pptx-pdf LICENSE-MIT LICENSE-APACHE README.md; rm -rf "$stage"
  printf "  %-48s %6.1f MB\n" "pptx-pdf-slim-$tag-$target.tar.gz" "$(echo "$(stat -f %z "$dist/pptx-pdf-slim-$tag-$target.tar.gz" 2>/dev/null || stat -c %s "$dist/pptx-pdf-slim-$tag-$target.tar.gz")/1048576" | bc -l)"
fi
cp install.sh "$dist/"
( cd "$dist" && (sha256sum *.tar.gz *.tar.zst 2>/dev/null || shasum -a 256 *.tar.gz *.tar.zst) > SHA256SUMS && cat SHA256SUMS )
echo "== release $tag"
git tag -f "$tag" -m "pptx-pdf $tag" && git push -q origin "$tag"
gh release create "$tag" $extra --title "pptx-pdf $tag" --notes-file - "$dist"/*.tar.gz "$dist"/*.tar.zst "$dist/SHA256SUMS" "$dist/install.sh" <<NOTES
Self-contained \`pptx-pdf\` binaries — LibreOffice's renderer built in, nothing to install.

\`\`\`
curl -fsSL https://raw.githubusercontent.com/dsaad68/pptx-pdf/main/install.sh | sh                  # self-contained
curl -fsSL https://raw.githubusercontent.com/dsaad68/pptx-pdf/main/install.sh | PPTX_PDF_SLIM=1 sh  # ~4 MB, fetches LibreOffice on first run
cargo binstall pptx-pdf   # self-contained;   cargo install pptx-pdf   # slim, built from source
\`\`\`

| asset | platform |
|---|---|
| \`pptx-pdf-$tag-aarch64-apple-darwin.tar.gz\` | macOS, Apple Silicon |
| \`pptx-pdf-$tag-aarch64-unknown-linux-gnu.tar.gz\` | Linux arm64 (glibc ≥ 2.36) |
| \`pptx-pdf-$tag-x86_64-unknown-linux-gnu.tar.gz\` | Linux x86_64 (glibc ≥ 2.36) |

\`pptx-pdf-slim-*\` binaries download \`lo-tree-<platform>-$tag.tar.zst\` from this release on first run and verify
it against the hash pinned in the binary. First run unpacks the tree to the user cache directory; later runs reuse it.
Verify downloads against \`SHA256SUMS\`.
NOTES
gh release view "$tag" --json url --jq .url
