#!/bin/bash
# Build a stripped, relocatable LibreOffice tree for pptx-pdf to embed.
#
#   [PPTX_PDF_ARCH=amd64|arm64] tools/lo-tree/build.sh   -> tools/lo-tree/out/lo-tree-linux-<arch>.tar.zst
#   (defaults to the Docker host arch; other archs run under emulation, slower)
#
# Pipeline (all inside a throwaway Debian container except the final compression):
#   1. Dockerfile      install libreoffice-impress-nogui + LOK headers + strace
#   2. gen_corpus.py   generate decks that exercise text/fonts/shapes/tables/images/charts
#   3. trace.sh        convert the corpus through LibreOfficeKit under strace, fresh profile per run
#   4. strip.sh        delete every LO file the trace never opened; reconvert; compare text
#   5. export.sh       copy the survivors to a relocatable tree (symlinks resolved,
#                      BRAND_BASE_DIR=${ORIGIN}/..) plus the system-lib closure
#   6. zstd -19        pack it
#
# Requires: docker, uv (for python-pptx), zstd. ~5 minutes; the apt layer is cached after the first run.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
cd "$here"
arch=${PPTX_PDF_ARCH:-$(docker info --format '{{.Architecture}}' | sed 's/x86_64/amd64/; s/aarch64/arm64/')}
platform="linux/$arch"; image="lo-strip:trixie-$arch"
mkdir -p corpus out

docker build -q --platform "$platform" -t "$image" . >/dev/null
uv run --quiet --with python-pptx --with pillow python3 gen_corpus.py
rm -f out/trace_*.log
docker run --rm --platform "$platform" -v "$here":/work "$image" bash -c './trace.sh && ./strip.sh && ./export.sh'

tar -cf - -C out/tree --exclude=./syslib . | zstd -19 -T0 -q -f -o "out/lo-tree-linux-$arch.tar.zst"
tar -cf - -C out/tree . | zstd -19 -T0 -q -f -o "out/lo-tree-linux-$arch-portable.tar.zst"
rm -rf out/tree
ls -la out/*.tar.zst
echo "embed with: PPTX_PDF_LO_TREE=$here/out/lo-tree-linux-$arch.tar.zst cargo build --release --features embedded"
