#!/bin/bash
# Build a stripped, relocatable LibreOffice tree for macOS from an installed LibreOffice.app.
#
#   tools/lo-tree/macos/build.sh [/Applications/LibreOffice.app]
#     -> tools/lo-tree/out/lo-tree-macos-<arch>.tar.zst
#
# Same method as the Linux pipeline (../build.sh): trace what a conversion opens, keep only
# that plus the wholesale UI/registry directories, verify with settled and read-only runs.
# macOS has no strace and fs_usage needs root, so the trace is an in-process interposer
# (fstrace.c, DYLD_INSERT_LIBRARIES) logging open/opendir/dlopen calls. The bundle's
# fundamentalrc is already ${ORIGIN}-relative, so no rc patching is needed; the tree keeps the
# bundle's Contents/{Frameworks,Resources} layout, which lo-runtime resolves.
#
# Requires: cc, uv (python-pptx corpus), zstd. About a minute.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); root=$(cd "$here/../../.." && pwd)
app=${1:-/Applications/LibreOffice.app}
arch=$(uname -m | sed 's/x86_64/amd64/')
out=$here/../out; mkdir -p "$out/macos" "$here/../corpus"
tree=$out/macos/tree; work=$out/macos
[ -f "$app/Contents/Frameworks/libmergedlo.dylib" ] || { echo "no LibreOffice at $app"; exit 1; }

echo "== corpus + tools"
( cd "$here/.." && uv run --quiet --with python-pptx --with pillow python3 gen_corpus.py >/dev/null )
cc -O1 -dynamiclib -o "$here/fstrace.dylib" "$here/fstrace.c"
cc -O1 -I "$here/../include" -o "$here/harness2" "$here/../dbg/harness2.c"
ro=$work/ro; [ -d "$ro" ] && chmod -R u+w "$ro"; rm -rf "$ro"; mkdir -p "$ro"; cp "$here"/../corpus/charts.pptx "$ro/"; chmod 444 "$ro"/*.pptx; chmod 555 "$ro"

echo "== trace (settled, fresh profile per run, plus a read-only input)"
log=$work/trace.log; rm -f "$log"; n=0
# dyld loads dependent dylibs with its own syscalls, invisible to the interposer; DYLD_PRINT_LIBRARIES
# reports every image it maps, so both logs together give the complete set
trace() { n=$((n+1)); FSTRACE_OUT=$log DYLD_INSERT_LIBRARIES=$here/fstrace.dylib DYLD_PRINT_LIBRARIES=1 X_SLEEP=4 X_DOCTYPE=1 \
  "$here/harness2" "$app/Contents/Frameworks" "file://$work/prof-$n" "file://$1" "file://$work/base-$(basename "$1" .pptx).pdf" 2>"$work/dyld-$n.log" >/dev/null || true; }
for f in "$here"/../corpus/*.pptx; do trace "$f"; done
trace "$ro/charts.pptx"
chmod 755 "$ro"

echo "== keep-list"
# opened files (kind o) inside the bundle, normalised (Frameworks/../Resources -> Resources)
# opened files and mapped images inside the bundle, normalised (Frameworks/../Resources -> Resources);
# fonts and icon themes are excluded here and added by policy below (fontconfig opens every font,
# the image tree probes every theme zip)
{ grep '^o ' "$log" | cut -c3-; grep -ohE "$app/[^[:space:]]+" "$work"/dyld-*.log; } | grep "^$app/" | sed -E 's|/[^/]+/\.\./|/|g' | sort -u \
  | grep -vE '/Resources/fonts/|/Resources/config/images_' \
  | while read -r p; do [ -f "$p" ] && echo "${p#$app/}" || true; done > "$work/keep.txt"
# icon themes: sukapura is the macOS default, colibre the fallback
( cd "$app" && ls Contents/Resources/config/images_sukapura.zip Contents/Resources/config/images_colibre.zip 2>/dev/null ) >> "$work/keep.txt" || true
# wholesale: lazily loaded UI, registry, presets, rc files, fontconfig bits
( cd "$app" && find Contents/Resources/config/soffice.cfg Contents/Resources/registry Contents/Resources/presets -type f; ls Contents/Resources/*rc | sed 's|^\./||' ) >> "$work/keep.txt"
# fonts: the metric-compatible set only (system fonts come through the generated fonts.conf)
( cd "$app" && ls Contents/Resources/fonts/truetype | grep -E '^(Liberation|Carlito|Caladea|DejaVu|opens___)' | sed 's|^|Contents/Resources/fonts/truetype/|' ) >> "$work/keep.txt"
sort -u -o "$work/keep.txt" "$work/keep.txt"
echo "   $(wc -l < "$work/keep.txt") files"

echo "== build tree"
rm -rf "$tree"; mkdir -p "$tree"
while read -r rel; do mkdir -p "$tree/$(dirname "$rel")"; cp -p "$app/$rel" "$tree/$rel"; done < "$work/keep.txt"
du -sh "$tree" "$tree/Contents/Frameworks" "$tree/Contents/Resources" | sed 's|^|   |'

echo "== verify: settled + read-only runs against the tree (expect 0 0 0)"
chmod 555 "$ro"; fails=0
for i in 1 2 3; do
  if X_SLEEP=5 X_DOCTYPE=1 "$here/harness2" "$tree/Contents/Frameworks" "file://$work/vprof-$i" "file://$ro/charts.pptx" "file://$work/v-$i.pdf" >"$work/v-$i.log" 2>&1; then echo -n "0 "; else echo -n "$? "; fails=$((fails+1)); fi
done; echo; chmod 755 "$ro"
[ "$fails" -eq 0 ] || { echo "settle check failed; last output:"; tail -3 "$work/v-1.log"; exit 1; }
echo "== verify: every corpus deck through the CLI, tree vs full bundle (pages, embedded fonts)"
cli=$root/target/release/pptx-pdf; [ -x "$cli" ] || ( cd "$root" && cargo build --release -q )
for f in "$here"/../corpus/*.pptx; do b=$(basename "$f" .pptx)
  "$cli" "$f" -o "$work/base-$b.pdf" --lo-dir "$app" -q 2>/dev/null
  "$cli" "$f" -o "$work/strip-$b.pdf" --lo-dir "$tree" -q
  python3 - "$work/base-$b.pdf" "$work/strip-$b.pdf" <<'PY'
import re,sys
def info(p):
    d=open(p,'rb').read(); pages=len(re.findall(rb'/Type\s*/Page[^s]',d)); fonts=sorted(set(re.findall(rb'/BaseFont\s*/[A-Z]{6}\+([A-Za-z0-9_-]+)',d)))
    return pages,fonts
a,b=info(sys.argv[1]),info(sys.argv[2]); ok=a==b
print(f"   {sys.argv[2].split('/')[-1]}: pages={b[0]} fonts_same={'yes' if a[1]==b[1] else 'NO'} pages_same={'yes' if a[0]==b[0] else 'NO'}")
sys.exit(0 if ok else 1)
PY
done

echo "== pack"
# COPYFILE_DISABLE: bsdtar would otherwise add AppleDouble "._*" entries, which configmgr then
# tries to parse as .xcd files inside registry/
find "$tree" -name '._*' -delete
COPYFILE_DISABLE=1 tar --no-xattrs -cf - -C "$tree" . | zstd -19 -T0 -q -f -o "$out/lo-tree-macos-$arch.tar.zst"
zstd -dc "$out/lo-tree-macos-$arch.tar.zst" | tar -tf - | grep -c '/\._' | xargs -I{} sh -c '[ {} -eq 0 ] || { echo "AppleDouble entries in tarball"; exit 1; }'
ls -la "$out/lo-tree-macos-$arch.tar.zst" | awk '{printf "   %.1f MB\n", $5/1048576}'
echo "embed with: PPTX_PDF_LO_TREE=$out/lo-tree-macos-$arch.tar.zst cargo build --release --features embedded"
