# Building, internals and status

The user-facing story is in the [README](../README.md); the long version of why and how is
[how-pptx-pdf-works.md](how-pptx-pdf-works.md). This page is for people building or hacking
on `pptx-pdf` itself.

## How it works

LibreOffice is not compiled here. `tools/lo-tree/build.sh` takes a prebuilt headless
LibreOffice, traces a real pptx-to-pdf conversion under `strace`, keeps only the ~100 files
that were opened, and packs them (33 MB, or 65 MB with its shared-library closure for
non-Debian hosts) into a tarball that the `embedded` cargo feature compiles into the binary.
On first run the tree is unpacked to the user's cache directory; every run after that loads
it in-process through [LibreOfficeKit](https://docs.libreoffice.org/libreofficekit.html) via
`dlopen`. There is no subprocess: about 300 ms from cold start to finished PDF for a six-slide
deck on Linux, a couple of seconds the very first time while the tree unpacks.

PNG output uses LibreOfficeKit's tile API (`setPart`, `getDocumentSize`, `paintTile`), the
same calls Collabora Online draws with. Each slide is one tile of its full size at the
requested DPI, byte-swapped from BGRA and PNG-encoded in Rust. No PDF is produced on the way.

```
crates/lok         safe LibreOfficeKit binding (the only `unsafe`; struct layouts transcribed
                   from LibreOfficeKit.h with an ABI-offset test); PDF export and tile rendering
crates/lo-runtime  find, download or unpack the tree; user profile; fontconfig; deck font scan
crates/pptx-pdf    the CLI
crates/pptx-pdf-py the Python extension (PyO3, maturin)
tools/lo-tree      builds the tree; its README has the measured numbers and the method
skills/pptx-pdf    the Agent Skill for the CLI
```

Fidelity is LibreOffice's, not a reimplementation: the stripped tree renders pixel-identically
to a full install on the test corpus.

## Fonts

Headless LibreOffice resolves fonts through fontconfig on every platform, macOS included, and
with no configuration it sees only the fonts it bundles and substitutes silently (Lato becomes
Linux Libertine, and so on). `lo-runtime` therefore writes a `fonts.conf` per run listing
LibreOffice's own fonts, the platform's system font directories, anything passed with
`--font-dir`, and a cache directory, unless the tree ships `etc/fonts` (the Linux trees do)
or `FONTCONFIG_FILE`/`FONTCONFIG_PATH` is already set, which is honoured as-is.

LibreOffice inspects every font it can see at startup, about a millisecond each. To keep the
boot fast on a Mac with 700+ system fonts, the CLI reads the families and scripts the deck
actually uses from its XML and lets LibreOffice see only those, the metric-compatible
substitutes (Carlito for Calibri, Liberation for Arial/Times/Courier, Caladea for Cambria),
any `--font-dir`, and fonts covering the deck's scripts. `--all-fonts` shows everything, which
only changes how a *missing* style or family is substituted.

## Building

```
cargo build --release                       # slim: downloads the tree for this version on first run
cargo build --release --no-default-features # needs --lo-dir or PPTX_PDF_LO_DIR at run time
tools/lo-tree/build.sh                      # docker + uv + zstd, ~30 s after the first run
PPTX_PDF_LO_TREE=tools/lo-tree/out/lo-tree-linux-arm64-portable.tar.zst \
  cargo build --release --features embedded # self-contained 64 MB Linux binary
tools/lo-tree/macos/build.sh                # from an installed LibreOffice.app, ~80 s
PPTX_PDF_LO_TREE=tools/lo-tree/out/lo-tree-macos-arm64.tar.zst \
  cargo build --release --features embedded # self-contained 80 MB macOS binary
```

The Python extension is built with maturin; see [python.md](python.md#development).

## End-to-end checks

- `tools/e2e-linux.sh` builds the CLI in a Rust container and runs it against the tree
  unpacked at a path LibreOffice has never seen: page ranges, notes pages, `--font-dir`, PNG
  rendering with and without hidden slides, error paths.
- `tools/e2e-portable.sh` and `tools/e2e-embedded.sh` repeat that in a bare
  `debian:trixie-slim` with nothing installed, the latter with the single embedded binary and
  no `--lo-dir`.
- `tools/e2e-slim.sh` serves the tree from a local HTTP server and checks the slim binary's
  download, verification, caching and rejection of a bad hash.
- `tools/e2e-macos.sh` is the macOS equivalent of the embedded check, with `LibreOffice.app`
  hidden from the process, plus a PNG comparison against `pdftoppm` when poppler and
  ImageMagick are installed.

Releases are cut by pushing a `vX.Y.Z` tag: `.github/workflows/release.yml` builds the trees,
both binary flavours and the wheels on GitHub runners and publishes them with `SHA256SUMS`.

## Status

- Linux arm64 and x86_64: both trees built and verified end to end, including the embedded
  binary in a bare container. (`PPTX_PDF_ARCH=amd64 tools/lo-tree/build.sh` builds the x86_64
  tree under emulation on an Apple Silicon host.)
- macOS (Apple Silicon): self-contained binary built and verified with `LibreOffice.app`
  hidden from the process. `tools/lo-tree/macos/build.sh` strips the official `.app`;
  `tools/e2e-macos.sh` proves it. Also runs against an installed `LibreOffice.app` with
  `--lo-dir /Applications/LibreOffice.app`.
- PNG output (`--png`, `pptx_pdf.render`) verified on both platforms, including hidden slides.
- Windows: LibreOfficeKit supports it and the library names are in place; untested.

LibreOffice is MPL-2.0; the tree ships its libraries unmodified apart from a relocatable
`fundamentalrc` and rpath patching, both reproducible from `tools/lo-tree`. `pptx-pdf` itself
is MIT OR Apache-2.0.
