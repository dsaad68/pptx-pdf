# pptx-pdf

Convert PowerPoint decks to PDF from the command line, with LibreOffice's renderer built in.
Users install one binary; there is no LibreOffice to install, no Java, no `soffice` on PATH.

```
pptx-pdf deck.pptx                    # -> deck.pdf
pptx-pdf deck.pptx -o out/deck.pdf --pages 2-5 --notes
pptx-pdf deck.pptx --font-dir ./brand-fonts
```

## How it works

LibreOffice is not compiled here. `tools/lo-tree/build.sh` takes a prebuilt headless
LibreOffice, traces a real pptx-to-pdf conversion under `strace`, keeps only the ~100 files
that were opened, and packs them (33 MB, or 65 MB with its shared-library closure for
non-Debian hosts) into a tarball that the `embedded` cargo feature
compiles into the binary. On first run the tree is unpacked to the user's cache directory;
every run after that loads it in-process through [LibreOfficeKit](https://docs.libreoffice.org/libreofficekit.html)
via `dlopen` — no subprocess; ~300 ms from cold start to finished PDF for a six-slide deck,
~2.4 s the very first time while the tree unpacks.

```
crates/lok         safe LibreOfficeKit binding (the only `unsafe`; struct layouts transcribed
                   from LibreOfficeKit.h with an ABI-offset test)
crates/lo-runtime  find or unpack the tree, per-run user profile, file:// URLs
crates/pptx-pdf    the CLI
tools/lo-tree      builds the tree; README has the measured numbers
```

Fidelity is LibreOffice's, not a reimplementation: the stripped tree renders pixel-identically
to a full install on the test corpus.

## Building

```
cargo build --release                       # needs --lo-dir or PPTX_PDF_LO_DIR at run time
tools/lo-tree/build.sh                      # docker + uv + zstd, ~30 s after the first run
PPTX_PDF_LO_TREE=tools/lo-tree/out/lo-tree-linux-arm64-portable.tar.zst \
  cargo build --release --features embedded # self-contained 64 MB binary
```

`tools/e2e-linux.sh` builds the CLI in a Rust container and runs it against the tree unpacked
at a path LibreOffice has never seen — page ranges, notes pages, `--font-dir`, error paths.
`tools/e2e-portable.sh` and `tools/e2e-embedded.sh` repeat that in a bare `debian:trixie-slim`
with nothing installed, the latter with the single embedded binary and no `--lo-dir`.

## Status

- Linux arm64 and x86_64: both trees built and verified end to end, including the embedded
  binary in a bare container. (`PPTX_PDF_ARCH=amd64 tools/lo-tree/build.sh` builds the x86_64
  tree under emulation on an Apple Silicon host.)
- macOS: works natively against an installed `LibreOffice.app` (`brew install --cask
  libreoffice`, then `--lo-dir /Applications/LibreOffice.app` or `PPTX_PDF_LO_DIR`), verified
  with LibreOffice 26.8 on Apple Silicon: ~3 s per conversion, dominated by per-run profile
  creation and CoreText font enumeration. No embedded macOS tree yet — the strip pipeline
  needs a macOS variant of `trace.sh` (`fs_usage`/`dtrace` instead of `strace`).

LibreOffice is MPL-2.0; the tree ships its libraries unmodified apart from a relocatable
`fundamentalrc` and rpath patching, both reproducible from `tools/lo-tree`.
