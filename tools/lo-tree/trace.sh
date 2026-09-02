#!/bin/bash
set -euo pipefail
cd /work
echo "== LO version"; dpkg -l libreoffice-core-nogui | tail -1 | awk '{print $2, $3}'
echo "== installed size"; du -sh /usr/lib/libreoffice /usr/share/libreoffice 2>/dev/null
ls corpus/*.pptx
echo "== gcc lokconv"; gcc -O1 -o lokconv lokconv.c -ldl
export HOME=/tmp/home; mkdir -p $HOME
export LOKCONV_SETTLE=${LOKCONV_SETTLE:-4}   # let deferred main-loop work happen inside the trace
echo "== warm run (profile creation)"
time ./lokconv /usr/lib/libreoffice/program file:///tmp/lokprofile file:///work/corpus/text_fonts.pptx file:///work/out/warm.pdf
echo "== traced run over corpus"
rm -f out/trace.log
for f in corpus/*.pptx; do b=$(basename "$f" .pptx)
  t0=$(date +%s%3N); strace -f -qq -e trace=openat,open,readlink -o "out/trace_$b.log" ./lokconv /usr/lib/libreoffice/program "file:///tmp/lokprofile_$b" "file:///work/$f" "file:///work/out/$b.pdf" '{"ExportNotesPages":{"type":"boolean","value":"false"}}'
  echo "$b: $(( $(date +%s%3N) - t0 ))ms"
done
# a read-only input (the common "file I don't own" case) shows a different infobar; trace it too
mkdir -p /tmp/ro; cp corpus/charts.pptx /tmp/ro/; chmod 444 /tmp/ro/charts.pptx; chmod 555 /tmp/ro
strace -f -qq -e trace=openat,open,readlink -o out/trace_readonly.log ./lokconv /usr/lib/libreoffice/program file:///tmp/lokprofile_ro file:///tmp/ro/charts.pptx file:///work/out/readonly.pdf
echo "== outputs"; ls -la out/*.pdf; for p in out/*.pdf; do echo "$p: $(pdfinfo "$p" | grep Pages)"; done
