#!/bin/bash
# The whole point: one self-contained binary in a bare container. Builds pptx-pdf with the
# portable tree embedded, runs it in debian:trixie-slim with nothing else installed and no
# --lo-dir, and checks the first run unpacks to the cache and the second run reuses it.
#
#   tools/e2e-embedded.sh   (after tools/lo-tree/build.sh)
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
arch=${PPTX_PDF_ARCH:-$(docker info --format '{{.Architecture}}' | sed 's/x86_64/amd64/; s/aarch64/arm64/')}
platform="linux/$arch"; image="lo-strip:trixie-$arch"
tree="tools/lo-tree/out/lo-tree-linux-$arch-portable.tar.zst"
work=$repo/tools/lo-tree/out/e2e-embedded
rm -rf "$work"; mkdir -p "$work/pdf" "$work/cache"

echo "== build embedded CLI for linux/$arch"
docker run --rm --platform "$platform" -v "$repo":/src -v pptx-pdf-cargo-registry:/usr/local/cargo/registry -w /src \
  -e PPTX_PDF_LO_TREE="/src/$tree" \
  rust:1.98-slim bash -c "cargo build --release --locked --features embedded --target-dir /src/target-linux-$arch-embedded && chown -R $(id -u):$(id -g) /src/target-linux-$arch-embedded" 2>&1 | tail -1
ls -la "$repo/target-linux-$arch-embedded/release/pptx-pdf" | awk '{printf "  binary: %.1f MB\n", $5/1048576}'

echo "== run in bare debian:trixie-slim (no LibreOffice, no fonts, no --lo-dir)"
docker run --rm --platform "$platform" \
  -v "$repo/target-linux-$arch-embedded/release/pptx-pdf":/usr/local/bin/pptx-pdf:ro \
  -v "$repo/tools/lo-tree/corpus":/corpus:ro \
  -v "$work/pdf":/pdf -v "$work/cache":/root/.cache -e OWNER="$(id -u):$(id -g)" \
  debian:trixie-slim bash -c '
    trap "chown -R $OWNER /pdf /root/.cache" EXIT
    set -e
    t0=$(date +%s%3N); pptx-pdf /corpus/text_fonts.pptx -o /pdf/first.pdf -v 2>&1 | grep -E "unpacking|ready|done" | sed "s/^/    /"; echo "  first run (unpack + convert): $(( $(date +%s%3N) - t0 ))ms"
    t0=$(date +%s%3N); pptx-pdf /corpus/charts.pptx -o /pdf/second.pdf -q; echo "  second run (cached): $(( $(date +%s%3N) - t0 ))ms"
    echo "  cache: $(du -sh /root/.cache/pptx-pdf | cut -f1) at $(ls /root/.cache/pptx-pdf)"
    head -c 8 /pdf/first.pdf; echo
  '
docker run --rm --platform "$platform" -v "$work/pdf":/pdf:ro "$image" bash -c 'for f in /pdf/*.pdf; do echo "  $(basename $f): $(pdfinfo $f | awk "/Pages/{print \$2}") pages"; done; echo "  fonts in first.pdf:"; pdffonts /pdf/first.pdf | tail -n +3 | awk "{print \"    \" \$1}" | sort -u'
