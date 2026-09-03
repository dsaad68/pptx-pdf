#!/bin/bash
# Portable-tree check: run the Linux CLI in a bare debian:trixie-slim container — no LibreOffice,
# no fonts, none of its library dependencies installed — using the -portable tree only.
#
#   tools/e2e-portable.sh   (after tools/lo-tree/build.sh and tools/e2e-linux.sh)
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
arch=${PPTX_PDF_ARCH:-$(docker info --format '{{.Architecture}}' | sed 's/x86_64/amd64/; s/aarch64/arm64/')}
platform="linux/$arch"; image="lo-strip:trixie-$arch"
work=$repo/tools/lo-tree/out/e2e-portable
rm -rf "$work"; mkdir -p "$work/tree" "$work/pdf"
zstd -dc "$repo/tools/lo-tree/out/lo-tree-linux-$arch-portable.tar.zst" | tar -x -C "$work/tree"
docker run --rm --platform "$platform" \
  -v "$repo/target-linux-$arch/release/pptx-pdf":/usr/local/bin/pptx-pdf:ro \
  -v "$work/tree":/opt/lo:ro \
  -v "$repo/tools/lo-tree/corpus":/corpus:ro \
  -v "$work/pdf":/pdf -e OWNER="$(id -u):$(id -g)" \
  debian:trixie-slim bash -c '
    trap "chown -R $OWNER /pdf" EXIT
    set -e
    echo "  installed fonts: $(ls /usr/share/fonts 2>/dev/null | wc -l), libreoffice: $(ls /usr/lib/libreoffice 2>/dev/null | wc -l), libicu: $(ls /usr/lib/*/libicuuc* 2>/dev/null | wc -l)"
    export PPTX_PDF_LO_DIR=/opt/lo
    t0=$(date +%s%3N); pptx-pdf /corpus/text_fonts.pptx -o /pdf/text.pdf -q; echo "  text_fonts: $(( $(date +%s%3N) - t0 ))ms, $(stat -c %s /pdf/text.pdf) bytes"
    pptx-pdf /corpus/charts.pptx -o /pdf/charts.pdf -q; echo "  charts: $(stat -c %s /pdf/charts.pdf) bytes"
    pptx-pdf /corpus/shapes_table_image.pptx -o /pdf/shapes.pdf -q; echo "  shapes: $(stat -c %s /pdf/shapes.pdf) bytes"
    head -c 8 /pdf/text.pdf; echo
  '
# pdffonts lives in the lo-strip image; inspect the output there
docker run --rm --platform "$platform" -v "$work/pdf":/pdf:ro "$image" bash -c 'for f in /pdf/*.pdf; do echo "  $(basename $f): $(pdfinfo $f | awk "/Pages/{print \$2}") pages"; done; echo "  fonts embedded in text.pdf:"; pdffonts /pdf/text.pdf | tail -n +3 | awk "{print \"    \" \$1}" | sort -u'
