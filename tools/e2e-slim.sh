#!/bin/bash
# Slim-binary end-to-end on the host: build the default (download) binary, serve this host's
# tree from a local HTTP server with a SHA256SUMS, and convert with the tree URL overridden,
# LibreOffice.app hidden (macOS) and a fresh cache. Checks both the SHA256SUMS lookup path and a
# pinned-hash build, and that a corrupted download is rejected.
#
#   tools/e2e-slim.sh   (after tools/lo-tree/build.sh or tools/lo-tree/macos/build.sh)
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd); cd "$repo"
case "$(uname -s)-$(uname -m)" in
  Darwin-arm64)  tree=macos-arm64 ;;
  Linux-aarch64) tree=linux-arm64-portable ;;
  Linux-x86_64)  tree=linux-amd64-portable ;;
  *) echo "unsupported host"; exit 1 ;;
esac
tarball=tools/lo-tree/out/lo-tree-$tree.tar.zst; [ -f "$tarball" ] || { echo "missing $tarball"; exit 1; }
work=$repo/tools/lo-tree/out/e2e-slim; rm -rf "$work"; mkdir -p "$work/srv" "$work/cache"
cp "$tarball" "$work/srv/"; ( cd "$work/srv" && (sha256sum *.tar.zst 2>/dev/null || shasum -a 256 *.tar.zst) > SHA256SUMS )
sha=$(cut -d' ' -f1 "$work/srv/SHA256SUMS")
port=$(( 20000 + RANDOM % 20000 ))
python3 -m http.server "$port" --bind 127.0.0.1 --directory "$work/srv" >"$work/http.log" 2>&1 &
server=$!; trap 'kill $server 2>/dev/null' EXIT; sleep 1
url="http://127.0.0.1:$port/lo-tree-$tree.tar.zst"
if [ "$(uname -s)" = Darwin ]; then
  hide='(version 1)(allow default)(deny file-read* (subpath "/Applications/LibreOffice.app"))'
  run() { env -u PPTX_PDF_LO_DIR sandbox-exec -p "$hide" "$@"; }
else
  run() { env -u PPTX_PDF_LO_DIR "$@"; }
fi
deck=tools/lo-tree/corpus/text_fonts.pptx; [ -f test.pptx ] && deck=test.pptx

echo "== slim binary (SHA256SUMS lookup)"
cargo build -p pptx-pdf --release --locked --target-dir target-slim 2>&1 | tail -1
ls -la target-slim/release/pptx-pdf | awk '{printf "  binary: %.1f MB\n", $5/1048576}'
t0=$(date +%s); run env PPTX_PDF_CACHE_DIR="$work/cache" PPTX_PDF_LO_TREE_URL="$url" target-slim/release/pptx-pdf "$deck" -o "$work/a.pdf" -q; echo "  first run (download + unpack + convert): $(( $(date +%s) - t0 ))s"
t0=$(date +%s); run env PPTX_PDF_CACHE_DIR="$work/cache" PPTX_PDF_LO_TREE_URL="$url" target-slim/release/pptx-pdf "$deck" -o "$work/b.pdf" -q; echo "  second run: $(( $(date +%s) - t0 ))s"
echo "  requests: $(grep -c 'GET /lo-tree' "$work/http.log") tarball, $(grep -c 'GET /SHA256SUMS' "$work/http.log") SHA256SUMS (expect 1 and 1)"
echo "== --fetch only, pinned hash, fresh cache"
rm -rf "$work/cache"
run env PPTX_PDF_CACHE_DIR="$work/cache" PPTX_PDF_LO_TREE_URL="$url" PPTX_PDF_LO_TREE_SHA256="$sha" target-slim/release/pptx-pdf --fetch -q && echo "  fetched"
echo "== wrong pin is rejected and leaves nothing behind"
rm -rf "$work/cache"
if run env PPTX_PDF_CACHE_DIR="$work/cache" PPTX_PDF_LO_TREE_URL="$url" PPTX_PDF_LO_TREE_SHA256="$(printf '0%.0s' $(seq 64))" target-slim/release/pptx-pdf --fetch -q 2>"$work/err.log"; then echo "  NOT rejected"; exit 1; else echo "  rejected: $(head -c 120 "$work/err.log")"; fi
[ -z "$(ls -A "$work/cache" 2>/dev/null | grep -v sources)" ] && echo "  cache clean" || { echo "  leftover in cache"; ls "$work/cache"; exit 1; }
python3 - "$work/a.pdf" <<'PY'
import re,sys
d=open(sys.argv[1],'rb').read(); print(f"  a.pdf: {len(re.findall(rb'/Type\s*/Page[^s]',d))} pages")
PY
