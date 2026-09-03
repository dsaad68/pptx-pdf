# pptx-pdf

Convert PowerPoint decks to PDF from the command line, with LibreOffice's renderer built in.
Users install one binary; there is no LibreOffice to install, no Java, no `soffice` on PATH.

```
pptx-pdf deck.pptx                    # -> deck.pdf
pptx-pdf deck.pptx -o out/deck.pdf --pages 2-5 --notes
pptx-pdf deck.pptx --font-dir ./brand-fonts
```

## Install

Prebuilt binaries for macOS (Apple Silicon), Linux arm64 and Linux x86_64 are on the
[releases page](https://github.com/dsaad68/pptx-pdf/releases), in two flavours:

| | self-contained | slim |
|---|---|---|
| download | 62–80 MB | ~4 MB |
| LibreOffice | inside the binary, unpacked to the cache on first run | fetched from the release on first run (needs network once), verified against a hash pinned in the binary |
| disk after first run | binary + cache | cache only |
| `curl \| sh` | `curl -fsSL https://raw.githubusercontent.com/dsaad68/pptx-pdf/main/install.sh \| sh` | same, with `PPTX_PDF_SLIM=1 sh` |
| cargo | `cargo binstall --git https://github.com/dsaad68/pptx-pdf pptx-pdf` | `cargo install --git https://github.com/dsaad68/pptx-pdf pptx-pdf` (builds from source) |

The installer verifies downloads against the release's `SHA256SUMS`; the slim binary verifies
its tree the same way. (The crates are not on crates.io yet, hence `--git`.) `pptx-pdf --fetch` pre-downloads the tree, for images and CI caches.
Both flavours keep the unpacked tree (225–360 MB) in the user cache directory
(`~/Library/Caches/pptx-pdf` or `~/.cache/pptx-pdf`; `PPTX_PDF_CACHE_DIR` overrides).

## Python

```bash
uv add pptx-pdf --find-links https://github.com/dsaad68/pptx-pdf/releases/expanded_assets/v0.2.3
```

```python
import pptx_pdf
pptx_pdf.convert("deck.pptx", pages="2-5", notes=True)
```

Same renderer, in-process, with an `isolated=True` mode that keeps LibreOffice in a warm child
interpreter. Details, uv variants and the error model are in [docs/python.md](docs/python.md).
For agents, [skills/pptx-pdf/SKILL.md](skills/pptx-pdf/SKILL.md) describes the CLI in the
Agent Skills format.

## Results

Measured on the release pipeline and an Apple Silicon laptop; the full story is in
[docs/how-pptx-pdf-works.md](docs/how-pptx-pdf-works.md#3-results).

What installing LibreOffice normally costs, measured from a bare Debian trixie image with every
dependency apt pulls in, and from the official macOS disk image:

| | packages | download | installed |
|---|---|---|---|
| `apt install libreoffice` as usually typed (with recommends: Java, fonts, everything) | 708 | 561 MB | **2,170 MB** |
| `apt install --no-install-recommends libreoffice` (full suite) | 209 | 151 MB | 564 MB |
| `apt install --no-install-recommends libreoffice-impress` | 170 | 117 MB | 422 MB |
| `apt install --no-install-recommends libreoffice-impress-nogui` (the leanest) | 151 | 105 MB | 366 MB |
| macOS `LibreOffice.app` | | 299 MB | 805 MB |

What `pptx-pdf` costs instead:

| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| self-contained binary (download = install) | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| tree the slim variant fetches once | 62 MB | 64 MB | 79 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | **0.6-0.7 s** |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | **1.7-1.8 s** |

LibreOffice inspects every font it can see when it starts, about a millisecond each. `soffice
--headless` is quick because it only ever sees the 128 fonts it bundles, and silently
substitutes for everything else. `pptx-pdf` reads the families and scripts a deck actually uses
from its XML and lets LibreOffice see those, the metric-compatible substitutes, any
`--font-dir`, and fonts covering the deck's scripts: LibreOffice boots in ~230 ms on a Mac with
700+ system fonts, and the deck's own fonts are used. `--all-fonts` shows everything (about a
second slower here), which only matters for how a *missing* style or family is substituted.

Fidelity: pages rendered from the stripped tree are pixel-identical to the full LibreOffice
install on Linux, and embed the same fonts on macOS. On a real 12-slide deck, stock
`soffice --headless` on macOS silently substituted Linux Libertine for Lato; `pptx-pdf` embeds
Lato, IBM Plex Mono and Caskaydia Cove, the deck's actual fonts.

## How it works

The long version — why, the strip pipeline step by step, measured results and the license
position — is in [docs/how-pptx-pdf-works.md](docs/how-pptx-pdf-works.md).

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

## Fonts

Headless LibreOffice resolves fonts through fontconfig on every platform, macOS included, and
with no configuration it sees only the fonts it bundles and substitutes silently (Lato becomes
Linux Libertine, and so on). `lo-runtime` therefore writes a `fonts.conf` per run listing
LibreOffice's own fonts, the platform's system font directories, anything passed with
`--font-dir`, and a cache directory — unless the tree ships `etc/fonts` (the Linux trees do)
or `FONTCONFIG_FILE`/`FONTCONFIG_PATH` is already set, which is honoured as-is. A font that is
installed on the machine is used; one that is not falls back to LibreOffice's metric-compatible
substitutes (Carlito for Calibri, Liberation for Arial/Times/Courier, Caladea for Cambria).

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

`tools/e2e-linux.sh` builds the CLI in a Rust container and runs it against the tree unpacked
at a path LibreOffice has never seen — page ranges, notes pages, `--font-dir`, error paths.
`tools/e2e-portable.sh` and `tools/e2e-embedded.sh` repeat that in a bare `debian:trixie-slim`
with nothing installed, the latter with the single embedded binary and no `--lo-dir`.
`tools/e2e-slim.sh` serves the tree from a local HTTP server and checks the slim binary's
download, verification, caching and rejection of a bad hash. `tools/e2e-macos.sh` is the
macOS equivalent of the embedded check.

## Status

- Linux arm64 and x86_64: both trees built and verified end to end, including the embedded
  binary in a bare container. (`PPTX_PDF_ARCH=amd64 tools/lo-tree/build.sh` builds the x86_64
  tree under emulation on an Apple Silicon host.)
- macOS (Apple Silicon): self-contained binary built and verified with `LibreOffice.app`
  hidden from the process — `tools/lo-tree/macos/build.sh` strips the official `.app`
  (82 MB tree, 84 MB binary), `tools/e2e-macos.sh` proves it. Also runs against an installed
  `LibreOffice.app` with `--lo-dir /Applications/LibreOffice.app`. Per-run cost is ~2-3 s
  (LibreOffice profile creation and font enumeration each run) versus ~0.3 s on Linux.
- Windows: LOK supports it and the library names are in place; untested.

LibreOffice is MPL-2.0; the tree ships its libraries unmodified apart from a relocatable
`fundamentalrc` and rpath patching, both reproducible from `tools/lo-tree`.
