#!/bin/bash
# Export the (already stripped, in this container) LO tree as a relocatable directory.
set -euo pipefail
cd /work
triplet=$(uname -m)-linux-gnu
rm -rf out/tree; mkdir -p out/tree
# -L dereferences Debian's symlinks (share -> /usr/share/libreoffice/share, main.xcd -> /etc/...)
cp -aL /usr/lib/libreoffice/. out/tree/
# make the tree relocatable: Debian hardcodes its FHS paths, TDF's own tarball uses ${ORIGIN}
sed -i 's|^BRAND_BASE_DIR=.*|BRAND_BASE_DIR=${ORIGIN}/..|' out/tree/program/fundamentalrc
sed -i '/^FHS_CONFIG_FILE=/d' out/tree/program/sofficerc
grep -n 'BRAND_BASE_DIR=' out/tree/program/fundamentalrc
# system .so closure outside glibc/libstdc++/libgcc, for the portable variant
mkdir -p out/tree/syslib
grep -E "^/(usr/)?lib/$triplet/" out/keep_files.txt \
  | grep -vE 'libc\.so|libm\.so|libdl\.so|libpthread\.so|librt\.so|ld-linux|libresolv\.so|libgcc_s|libstdc\+\+' \
  | sort -u | while read -r f; do cp -a "$f" out/tree/syslib/; done   # -a keeps SONAME symlinks as links
# bundled fonts: every font file the conversions opened, into the directory VCL scans on its own
mkdir -p out/tree/share/fonts/truetype
grep -E '^/usr/share/fonts/' out/keep_files.txt | sort -u | while read -r f; do cp -L "$f" out/tree/share/fonts/truetype/; done
ls out/tree/share/fonts/truetype | wc -l | xargs echo "   fonts bundled:"
# fontconfig configuration the conversions read, so a bare host gets sane aliases and no warnings
mkdir -p out/tree/etc/fonts
grep -E '^/etc/fonts/' out/keep_files.txt | sort -u | while read -r f; do mkdir -p "out/tree/etc/fonts/$(dirname "${f#/etc/fonts/}")"; cp -L "$f" "out/tree/etc/fonts/${f#/etc/fonts/}"; done
find out/tree/etc/fonts -type f | wc -l | xargs echo "   fontconfig files bundled:"
# portable variant: make every LO library find syslib/ relative to itself, no LD_LIBRARY_PATH needed
find out/tree/program -name '*.so*' -type f -exec patchelf --set-rpath '$ORIGIN:$ORIGIN/../syslib' {} \;
find out/tree/syslib -name '*.so*' -type f -exec patchelf --set-rpath '$ORIGIN' {} \;
echo "== exported tree:"; du -sh out/tree/program out/tree/share out/tree/presets out/tree/syslib out/tree
