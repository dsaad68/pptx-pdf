#!/bin/bash
# Package the verified embedded binaries as release assets and publish a GitHub release.
#
#   tools/release.sh v0.1.0 [--draft]
#
# Expects the binaries the e2e suites built and tested:
#   target-linux-arm64-embedded/release/pptx-pdf   (tools/e2e-embedded.sh, PPTX_PDF_ARCH=arm64)
#   target-linux-amd64-embedded/release/pptx-pdf   (tools/e2e-embedded.sh, PPTX_PDF_ARCH=amd64)
#   target-macos-embedded/release/pptx-pdf         (tools/e2e-macos.sh)
# Asset names follow the cargo-binstall template in crates/pptx-pdf/Cargo.toml.
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
cp install.sh "$dist/"
( cd "$dist" && (sha256sum *.tar.gz 2>/dev/null || shasum -a 256 *.tar.gz) > SHA256SUMS && cat SHA256SUMS )
echo "== release $tag"
git tag -f "$tag" -m "pptx-pdf $tag" && git push -q origin "$tag"
gh release create "$tag" $extra --title "pptx-pdf $tag" --notes-file - "$dist"/*.tar.gz "$dist/SHA256SUMS" "$dist/install.sh" <<NOTES
Self-contained \`pptx-pdf\` binaries — LibreOffice's renderer built in, nothing to install.

\`\`\`
curl -fsSL https://raw.githubusercontent.com/dsaad68/pptx-pdf/main/install.sh | sh
# or
cargo binstall pptx-pdf
\`\`\`

| asset | platform |
|---|---|
| \`pptx-pdf-$tag-aarch64-apple-darwin.tar.gz\` | macOS, Apple Silicon |
| \`pptx-pdf-$tag-aarch64-unknown-linux-gnu.tar.gz\` | Linux arm64 (glibc ≥ 2.36) |
| \`pptx-pdf-$tag-x86_64-unknown-linux-gnu.tar.gz\` | Linux x86_64 (glibc ≥ 2.36) |

First run unpacks the embedded LibreOffice tree to the user cache directory; later runs reuse it.
Verify downloads against \`SHA256SUMS\`.
NOTES
gh release view "$tag" --json url --jq .url
