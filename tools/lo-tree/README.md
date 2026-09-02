# lo-tree: the LibreOffice runtime pptx-pdf embeds

pptx-pdf renders through LibreOffice in-process via LibreOfficeKit. Nothing is compiled from
LibreOffice source: `build.sh` takes a prebuilt headless LibreOffice, keeps only the files a
pptx-to-pdf conversion actually opens, and packs the survivors into a relocatable tarball that
the `embedded` cargo feature compiles into the binary.

## Measured (Debian trixie, LibreOffice 25.2, 2026-09)

| | arm64 | x86_64 |
|---|---|---|
| `libreoffice-impress-nogui` + deps as installed | 199 MB | 199 MB |
| `lo-tree-linux-<arch>.tar.zst` (Debian-host variant, fonts + all UI files) | **33.5 MB** | **35.4 MB** |
| `lo-tree-linux-<arch>-portable.tar.zst` (+ non-glibc `.so` closure, `$ORIGIN` rpaths) | **65 MB** | **67 MB** |
| `pptx-pdf` binary with the portable tree embedded | 64.5 MB | 66.6 MB |

Timings, in-process through LibreOfficeKit, fresh user profile every run (arm64 native;
x86_64 numbers here are under Rosetta emulation and ~3-4x slower than native hardware):

| | arm64 |
|---|---|
| 6-slide deck, cold start to PDF | ~280 ms |
| embedded binary, first run (unpack 224 MB + convert) | ~1.8 s |
| embedded binary, later runs | ~260 ms |

Fidelity: the corpus decks render **pixel-identical** from the stripped tree versus the full
install (`pdftoppm` at 60 dpi, every page, zero differing pixels). Every tree above passed the
settle check (`dbg/harness2.c` with `X_SLEEP=5` and a read-only input) and the three e2e
suites — the portable and embedded suites run in a bare `debian:trixie-slim` with no
LibreOffice, no fonts and none of its library dependencies installed.

## What survives the strip

Traced: `program/libmergedlo.so` (79 MB, most of LibreOffice), `libsdlo.so` (Impress), the UNO
runtime (`libuno_*`, `libreglo`, `libstorelo`, `libunoidllo`), `libpdffilterlo.so`,
`libpdfiumlo.so`, `liblocaledata_*`, `services/services.rdb`, `types/*.rdb`, `*rc`,
`share/filter`, `presets/`, the fonts the conversions opened (Liberation, Carlito, Caladea,
DejaVu, OpenSymbol) into `share/fonts/truetype`, and fontconfig's configuration into
`etc/fonts`.

Kept wholesale, not traced: `share/config/soffice.cfg` (every `.ui` — infobars and dialogs
are loaded lazily under conditions no corpus can enumerate, and a missing one aborts the
process), `share/registry` (all `.xcd`), `share/uno_packages`. Together they cost about
0.4 MB compressed.

## Two variants

- `lo-tree-linux-<arch>.tar.zst` — LibreOffice only. Runs where Debian trixie's shared
  libraries exist (a Debian/Ubuntu host or container). Smallest.
- `lo-tree-linux-<arch>-portable.tar.zst` — adds `syslib/` with every non-glibc `.so` the
  conversion mapped, and every library is `patchelf`ed with an `$ORIGIN`-relative RUNPATH so
  nothing outside the tree is needed except glibc/libstdc++. This is what `e2e-embedded.sh`
  embeds.

## Method

The keep-list is empirical, not hand-picked: `trace.sh` runs each corpus deck through a
40-line C program against the real LOK API under `strace -e openat`, with a **fresh user
profile per run** so first-run profile creation is captured too. `strip.sh` deletes everything
under the LibreOffice directories that no run opened, reconverts, and compares extracted text.
Three spellings of each opened path are kept — as opened, lexically normalised, and
symlink-resolved — because Debian links `share/registry/main.xcd` into `/etc`.

The traced and verifying runs **settle for a few seconds after load and after save**
(`LOKCONV_SETTLE`). LibreOffice's main-loop thread constructs the document frame as a deferred
event after a load — `sfx/ui/infobar.ui`, slide-sorter resources — and if that event finds a
file missing it aborts the whole process. A harness that exits immediately never triggers it,
which made a broken tree look verified. Directories the process opened are also preserved even
when the strip empties them.

`strace` does not work under CPU emulation (Rosetta/QEMU). When the trace comes back empty,
`strip.sh` borrows the keep-list from a native trace of another arch (`keep_files-<arch>.txt`)
by swapping the multiarch triplet, and still verifies natively on the current arch. CI on a
real x86_64 runner traces normally.

`export.sh` copies the survivors with symlinks dereferenced and rewrites
`BRAND_BASE_DIR=file:///usr/lib/libreoffice` to `${ORIGIN}/..` (the form TDF's own tarball
uses), which is what makes the tree work from any directory.

Extend `gen_corpus.py` when a deck exposes a code path the corpus doesn't cover — a file that
the strip removed is the failure mode, and it shows up as a LibreOffice bootstrap error or a
missing filter, never as a wrong-looking PDF.
