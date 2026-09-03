#!/bin/bash
# macOS end-to-end: build pptx-pdf with the macOS tree embedded and run it with LibreOffice.app
# made invisible (sandbox-exec denies reading it) and no --lo-dir, on the corpus and, if present,
# ./test.pptx. Then the same binary with --font-dir and --pages.
#
#   tools/e2e-macos.sh   (after tools/lo-tree/macos/build.sh)
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
arch=$(uname -m | sed 's/x86_64/amd64/')
tree="$repo/tools/lo-tree/out/lo-tree-macos-$arch.tar.zst"; [ -f "$tree" ] || { echo "missing $tree"; exit 1; }
work=$repo/tools/lo-tree/out/macos/e2e; rm -rf "$work"; mkdir -p "$work/cache"

echo "== build embedded CLI for macos/$arch"
( cd "$repo" && PPTX_PDF_LO_TREE=$tree cargo build -p pptx-pdf --release --locked --features embedded --target-dir "$repo/target-macos-embedded" 2>&1 | tail -1 )
bin=$repo/target-macos-embedded/release/pptx-pdf
ls -la "$bin" | awk '{printf "  binary: %.1f MB\n", $5/1048576}'

# no LibreOffice.app visible, no PPTX_PDF_LO_DIR, cache redirected so the first run really unpacks
profile='(version 1)(allow default)(deny file-read* (subpath "/Applications/LibreOffice.app"))'
run() { env -u PPTX_PDF_LO_DIR PPTX_PDF_CACHE_DIR="$work/cache" sandbox-exec -p "$profile" "$bin" "$@"; }
echo "== run with LibreOffice.app hidden"
t0=$(date +%s); run "$repo/tools/lo-tree/corpus/text_fonts.pptx" -o "$work/first.pdf" -v 2>&1 | grep -E "unpacking|ready|done" | sed 's/\x1b\[[0-9;]*m//g; s/^/    /'; echo "  first run (unpack + convert): $(( $(date +%s) - t0 ))s"
ms() { python3 -c 'import time; print(int(time.time()*1000))'; }
t0=$(ms); run "$repo/tools/lo-tree/corpus/charts.pptx" -o "$work/second.pdf" -q; echo "  second run (cached): $(( $(ms) - t0 ))ms"
t0=$(ms); run "$repo/tools/lo-tree/corpus/charts.pptx" -o "$work/third.pdf" -q; echo "  third run (cached): $(( $(ms) - t0 ))ms"
run "$repo/tools/lo-tree/corpus/text_fonts.pptx" -o "$work/range.pdf" --pages 2-3 -q
run "$repo/tools/lo-tree/corpus/text_fonts.pptx" -o "$work/fontdir.pdf" --font-dir "$repo/tools/lo-tree/out/fonts" -q 2>/dev/null || true
[ -f "$repo/test.pptx" ] && run "$repo/test.pptx" -o "$work/test.pdf" -q
echo "== png"
t0=$(ms); run "$repo/tools/lo-tree/corpus/text_fonts.pptx" --png "$work/png" -q; echo "  6 slides at 150 dpi: $(( $(ms) - t0 ))ms, $(ls "$work/png" | wc -l | tr -d ' ') files, $(sips -g pixelWidth -g pixelHeight "$work/png/slide-01.png" | awk '/pixel/{printf "%s ", $2}')px"
run "$repo/tools/lo-tree/corpus/hidden.pptx" --png "$work/png-hidden" -q; echo "  hidden.pptx default: $(ls "$work/png-hidden" | tr '\n' ' ')"
run "$repo/tools/lo-tree/corpus/hidden.pptx" --png "$work/png-hidden-all" --hidden -q; echo "  hidden.pptx --hidden: $(ls "$work/png-hidden-all" | tr '\n' ' ')"
if command -v pdftoppm >/dev/null && command -v magick >/dev/null; then
  pdftoppm -r 150 -png "$work/first.pdf" "$work/png/ref" 2>/dev/null
  echo "  vs pdftoppm of the PDF (RMSE, 0 = identical): $(magick compare -metric RMSE "$work/png/slide-01.png" "$work/png/ref-1.png" null: 2>&1 | sed 's/.*(\(.*\))/\1/')"
fi
echo "  cache: $(du -sh "$work/cache" | cut -f1)"
python3 - "$work" <<'PY'
import re,sys,os,glob
w=sys.argv[1]
for f in sorted(glob.glob(w+'/*.pdf')):
    d=open(f,'rb').read(); pages=len(re.findall(rb'/Type\s*/Page[^s]',d)); fonts=sorted(set(m.decode() for m in re.findall(rb'/BaseFont\s*/[A-Z]{6}\+([A-Za-z0-9_-]+)',d)))
    print(f"  {os.path.basename(f)}: {pages} pages, fonts: {', '.join(fonts)[:110]}")
PY
