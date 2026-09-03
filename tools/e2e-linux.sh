#!/bin/bash
# End-to-end check on Linux: build the real CLI in a Rust container, then run it inside the
# lo-tree container against the stripped tree unpacked at a path LibreOffice has never seen.
#
#   tools/e2e-linux.sh [tools/lo-tree/out/lo-tree-linux-<arch>.tar.zst]
#
# Requires: docker, zstd; the corpus from tools/lo-tree/build.sh; a font at
# tools/lo-tree/out/fonts/ for the --font-dir check (optional).
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
arch=${PPTX_PDF_ARCH:-$(docker info --format '{{.Architecture}}' | sed 's/x86_64/amd64/; s/aarch64/arm64/')}
platform="linux/$arch"; image="lo-strip:trixie-$arch"
tree_tar=${1:-$repo/tools/lo-tree/out/lo-tree-linux-$arch.tar.zst}
work=$repo/tools/lo-tree/out/e2e
rm -rf "$work"; mkdir -p "$work/relocated/tree" "$work/pdf"

echo "== build CLI for linux/$arch"
docker run --rm --platform "$platform" -v "$repo":/src -v pptx-pdf-cargo-registry:/usr/local/cargo/registry -w /src \
  rust:1.98-slim cargo build -p pptx-pdf --release --locked --target-dir "/src/target-linux-$arch" 2>&1 | tail -2

echo "== unpack tree somewhere new"
zstd -dc "$tree_tar" | tar -x -C "$work/relocated/tree"

echo "== run"
docker run --rm --platform "$platform" \
  -v "$repo/target-linux-$arch/release/pptx-pdf":/usr/local/bin/pptx-pdf:ro \
  -v "$work/relocated/tree":/opt/lo-relocated:ro \
  -v "$repo/tools/lo-tree/corpus":/corpus:ro \
  -v "$repo/tools/lo-tree/out/fonts":/fonts:ro \
  -v "$work/pdf":/pdf -e OWNER="$(id -u):$(id -g)" \
  "$image" bash -c '
    trap "chown -R $OWNER /pdf" EXIT
    set -e
    export PPTX_PDF_LO_DIR=/opt/lo-relocated
    t0=$(date +%s%3N); pptx-pdf /corpus/text_fonts.pptx -o /pdf/text.pdf -v; echo "  full: $(( $(date +%s%3N) - t0 ))ms, $(pdfinfo /pdf/text.pdf | awk "/Pages/{print \$2}") pages"
    pptx-pdf /corpus/text_fonts.pptx -o /pdf/range.pdf --pages 2-3 -q; echo "  --pages 2-3: $(pdfinfo /pdf/range.pdf | awk "/Pages/{print \$2}") pages"
    pptx-pdf /corpus/text_fonts.pptx -o /pdf/notes.pdf --notes -q; echo "  --notes: $(pdfinfo /pdf/notes.pdf | awk "/Pages/{print \$2}") pages"
    pptx-pdf /corpus/charts.pptx -o /pdf/charts.pdf -q; echo "  charts: $(pdfinfo /pdf/charts.pdf | awk "/Pages/{print \$2}") pages"
    echo "  fonts without --font-dir:"; pdffonts /pdf/text.pdf | tail -n +3 | awk "{print \"    \" \$1}" | sort -u
    if ls /fonts/*.ttf >/dev/null 2>&1; then
      pptx-pdf /corpus/text_fonts.pptx -o /pdf/fontdir.pdf --font-dir /fonts -q
      echo "  fonts with --font-dir:"; pdffonts /pdf/fontdir.pdf | tail -n +3 | awk "{print \"    \" \$1}" | sort -u
    fi
    echo "  error path (bad input):"; pptx-pdf /corpus/nope.pptx -o /pdf/x.pdf 2>&1 | head -1 || true
    printf "%s" "PK" > /pdf/garbage.pptx; echo "  error path (garbage file):"; pptx-pdf /pdf/garbage.pptx -o /pdf/g.pdf 2>&1 | head -2 || true
  '
