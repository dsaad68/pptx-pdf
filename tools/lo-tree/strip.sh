#!/bin/bash
# Build the keep-list from strace logs, delete everything else under the LO dirs,
# reconvert the corpus, and compare. Runs inside the throwaway container.
set -euo pipefail
cd /work; export HOME=/tmp/home; mkdir -p $HOME
export LOKCONV_SETTLE=${LOKCONV_SETTLE:-4}   # verification must exercise the deferred startup path too
triplet=$(uname -m)-linux-gnu
export HOME=/tmp/home; mkdir -p $HOME
LO_DIRS="/usr/lib/libreoffice /usr/share/libreoffice /etc/libreoffice"

# 1. successful opens: lines with openat(...) = <fd> (not -1), extract the path
grep -h 'openat(' out/trace_*.log | grep -v '= -1 ' | sed -E 's/.*openat\([^,]+, "([^"]+)".*/\1/' | sort -u > out/opened_all.txt || true   # empty under emulation
# keep three spellings of each path: as opened, lexically normalised (the symlink itself),
# and fully resolved (the symlink target)
while read -r p; do [ -e "$p" ] && { realpath -sm "$p"; readlink -f "$p"; }; done < out/opened_all.txt | sort -u > out/opened_resolved.txt
cat out/opened_all.txt out/opened_resolved.txt | sort -u > out/opened_union.txt
# only regular files; directories that were opened are kept separately so an emptied one survives
while read -r p; do [ -f "$p" ] && echo "$p" || true; done < out/opened_union.txt > out/keep_files.txt
while read -r p; do [ -d "$p" ] && echo "$p" || true; done < out/opened_union.txt | { grep -E "^(/usr/lib/libreoffice|/usr/share/libreoffice|/etc/libreoffice)" || true; } | sort -u > out/keep_dirs.txt
# Conditional UI (infobars, dialogs) is loaded lazily and only under conditions a corpus cannot
# enumerate; a missing .ui aborts the whole process. Keep those directories wholesale — they are
# small text files — so the keep-list is trace ∪ share/config ∪ share/registry ∪ share/uno_packages.
for d in /usr/lib/libreoffice/share/config/soffice.cfg /usr/lib/libreoffice/share/registry /usr/lib/libreoffice/share/uno_packages; do
  [ -e "$d" ] && find -L "$d" -type f -exec readlink -f {} \; ; done | sort -u >> out/keep_files.txt || true
sort -u -o out/keep_files.txt out/keep_files.txt
if [ -s out/opened_all.txt ]; then
  cp out/keep_files.txt "out/keep_files-$(uname -m).txt"   # a real trace: remember it for other arches
else
  # ptrace does not work under CPU emulation (Rosetta/QEMU), so the trace is empty. LibreOffice's
  # file layout is identical across Debian arches apart from the multiarch triplet, so borrow a
  # native trace from another arch; the strip below still verifies natively on this one.
  src=$(ls out/keep_files-*.txt 2>/dev/null | grep -v "$(uname -m)" | head -1)
  [ -n "$src" ] || { echo "no usable trace and no keep_files-<arch>.txt to borrow from"; exit 1; }
  echo "NOTE: strace unusable here; deriving keep-list from $src by triplet substitution"
  sed -E "s@/(aarch64|x86_64)-linux-gnu/@/$triplet/@" "$src" | while read -r p; do [ -f "$p" ] && echo "$p" || true; done > out/keep_files.txt
fi

echo "== opened files by area"
for area in /usr/lib/libreoffice /usr/share/libreoffice /etc/libreoffice /usr/share/fonts /etc/fonts /usr/lib/$triplet /lib/$triplet /usr/share; do
  n=$(grep -c "^$area" out/keep_files.txt || true); sz=$(grep "^$area" out/keep_files.txt | xargs -r du -ch 2>/dev/null | tail -1 | cut -f1)
  printf "%-28s %4s files %6s\n" "$area" "$n" "${sz:-0}"
done
echo "== LO tree: before"; du -sh /usr/lib/libreoffice /usr/share/libreoffice | tr '\n' ' '; echo
echo "== LO .so kept:"; grep "^/usr/lib/libreoffice/program/.*\.so" out/keep_files.txt | xargs -r du -ch | sort -h | tail -12
echo "== system .so kept (outside glibc):"; grep -E "^/(usr/)?lib/$triplet/" out/keep_files.txt | grep -vE 'libc\.so|libm\.so|libdl\.so|libpthread\.so|librt\.so|ld-linux|libresolv\.so|libgcc_s|libstdc\+\+' | xargs -r du -ch | tail -1
grep -E "^/(usr/)?lib/$triplet/" out/keep_files.txt | sort -u | xargs -r du -k | sort -rn > out/syslibs_sized.txt
echo "== top 25 system libs (KB):"; head -25 out/syslibs_sized.txt

# 2. delete everything under LO dirs not in keep list
grep -E "^(/usr/lib/libreoffice|/usr/share/libreoffice|/etc/libreoffice)" out/keep_files.txt | sort -u > out/keep_lo.txt
find $LO_DIRS -type f -o -type l | sort > out/all_lo.txt
comm -23 out/all_lo.txt out/keep_lo.txt > out/delete_lo.txt
echo "== deleting $(wc -l < out/delete_lo.txt) of $(wc -l < out/all_lo.txt) LO files"
xargs -r rm -f < out/delete_lo.txt
find $LO_DIRS -type d -empty -delete 2>/dev/null || true
xargs -r mkdir -p < out/keep_dirs.txt
echo "== LO tree: after"; du -sh /usr/lib/libreoffice /usr/share/libreoffice 2>/dev/null | tr '\n' ' '; echo

# 3. reconvert and compare; the second pass converts from a read-only location so the
#    read-only infobar path (extrabutton.ui) is exercised too
mkdir -p out/stripped /tmp/ro; cp corpus/*.pptx /tmp/ro/; chmod 444 /tmp/ro/*.pptx; chmod 555 /tmp/ro
./lokconv /usr/lib/libreoffice/program file:///tmp/lokprofile_ro file:///tmp/ro/charts.pptx file:///tmp/ro_charts.pdf && echo "read-only input: ok" || { echo "read-only input: FAILED"; exit 1; }
for f in corpus/*.pptx; do b=$(basename "$f" .pptx)
  t0=$(date +%s%3N)
  ./lokconv /usr/lib/libreoffice/program file:///tmp/lokprofile2 "file:///work/$f" "file:///work/out/stripped/$b.pdf" '{"ExportNotesPages":{"type":"boolean","value":"false"}}'
  echo "$b: $(( $(date +%s%3N) - t0 ))ms  pages=$(pdfinfo out/stripped/$b.pdf | awk '/Pages/{print $2}')  text_same=$( [ "$(pdftotext out/$b.pdf - | md5sum)" = "$(pdftotext out/stripped/$b.pdf - | md5sum)" ] && echo yes || echo NO )  bytes_same=$( cmp -s out/$b.pdf out/stripped/$b.pdf && echo yes || echo no )"
done
