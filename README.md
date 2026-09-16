# pptx-pdf

[![Release](https://img.shields.io/github/v/release/dsaad68/pptx-pdf?label=release)](https://github.com/dsaad68/pptx-pdf/releases/latest)
[![Build](https://img.shields.io/github/actions/workflow/status/dsaad68/pptx-pdf/release.yml?label=build)](https://github.com/dsaad68/pptx-pdf/actions/workflows/release.yml)
[![Platforms](https://img.shields.io/badge/platforms-macOS%20arm64%20%7C%20Linux%20arm64%20%7C%20Linux%20x86__64-lightgrey)](https://github.com/dsaad68/pptx-pdf/releases/latest)
[![Python](https://img.shields.io/badge/python-3.9%2B-blue)](docs/python.md)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE-MIT)

**`pptx-pdf` turns a PowerPoint deck into a PDF or into one PNG per slide, with LibreOffice's
renderer built into a single binary, so nobody installs LibreOffice.**

```bash
pptx-pdf deck.pptx                    # -> deck.pdf
pptx-pdf deck.pptx --png slides/      # -> slides/slide-01.png, slide-02.png, ...
```

## Why

- **PNG straight from the renderer.** `--png` paints each slide through LibreOffice's tile
  renderer, the code path Collabora Online draws with. There is no PDF and no PDF rasteriser
  in between: the pixels are LibreOffice's own, at any DPI, in about the time the PDF takes.
- **PDF as Impress exports it.** Same engine, same output as *File > Export as PDF*. Page
  ranges, notes pages, hidden slides handled. Fonts installed on the machine are used; missing
  ones fall back to metric-compatible substitutes.
- **Small.** A trimmed LibreOffice, about 100 files instead of 2,000, inside a 62 to 80 MB
  binary, or a 4 MB one that fetches the tree on first run. A normal
  `apt install libreoffice` is 708 packages and 2.2 GB.
- **Fast.** In-process through LibreOfficeKit, no `soffice` subprocess. A six-slide deck
  converts in 0.18 s on Linux, faster than LibreOffice itself started cold.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/dsaad68/pptx-pdf/main/install.sh | sh                  # self-contained, 62-80 MB
curl -fsSL https://raw.githubusercontent.com/dsaad68/pptx-pdf/main/install.sh | PPTX_PDF_SLIM=1 sh  # 4 MB, fetches LibreOffice on first run
cargo binstall --git https://github.com/dsaad68/pptx-pdf pptx-pdf                                   # self-contained, via cargo
```

Prebuilt for macOS (Apple Silicon), Linux arm64 and Linux x86_64 on the
[releases page](https://github.com/dsaad68/pptx-pdf/releases). Downloads are verified against
the release's `SHA256SUMS`; the slim binary verifies the tree it fetches against a hash pinned
inside it. `pptx-pdf --fetch` pre-downloads the tree for images and CI caches. The unpacked
tree lives in the user cache (`~/Library/Caches/pptx-pdf` or `~/.cache/pptx-pdf`).

**Python**, same renderer in-process, wheels on the release page:

```bash
uv add pptx-pdf --find-links https://github.com/dsaad68/pptx-pdf/releases/expanded_assets/v1.0.0
```

## Use

```bash
pptx-pdf deck.pptx -o out/deck.pdf --pages 2-5 --notes   # page range, speaker-notes pages
pptx-pdf deck.pptx --png slides/ --dpi 96                # 1280x720 per slide (150 dpi default: 2000x1125)
pptx-pdf deck.pptx --png slides/ --pages 3 --hidden      # one slide; include slides marked hidden
pptx-pdf deck.pptx --font-dir ./brand-fonts              # extra .ttf/.otf for this run
```

```python
import pptx_pdf
pptx_pdf.convert("deck.pptx", pages="2-5", notes=True)     # -> Path("deck.pdf")
pptx_pdf.render("deck.pptx", "slides/", dpi=150)           # -> [Path("slides/slide-01.png"), ...]
```

`isolated=True` keeps LibreOffice in a warm child interpreter so a crash there cannot take
your process down. Options, errors and uv variants: [docs/python.md](docs/python.md).

## Agent skill

[`skills/pptx-pdf/SKILL.md`](skills/pptx-pdf/SKILL.md) describes the CLI in the Agent Skills
format: install, convert, render, verify, and what the exit codes mean. Drop it where your
agent looks for skills, for example:

```bash
git clone --depth 1 https://github.com/dsaad68/pptx-pdf /tmp/pptx-pdf
cp -r /tmp/pptx-pdf/skills/pptx-pdf ~/.claude/skills/        # Claude Code, all projects
cp -r /tmp/pptx-pdf/skills/pptx-pdf .claude/skills/          # or this project only
```

The agent then picks it up when a task involves a presentation, PDF or slide images, and can
invoke it directly as `/pptx-pdf`.

## Results

Measured on the release pipeline and an Apple Silicon laptop; method and full tables in
[docs/how-pptx-pdf-works.md](docs/how-pptx-pdf-works.md#3-results).

**Size**: what installing LibreOffice normally costs, against `pptx-pdf`.

| | download | installed |
|---|---|---|
| `apt install libreoffice` as usually typed | 561 MB | 2,170 MB |
| `apt install --no-install-recommends libreoffice-impress-nogui` (the leanest) | 105 MB | 366 MB |
| macOS `LibreOffice.app` | 299 MB | 805 MB |
| **`pptx-pdf` self-contained** (Linux / macOS) | **62-64 / 80 MB** | 225-360 MB cache |
| **`pptx-pdf` slim** or Python wheel | **4 / 1-2 MB** | + the tree, fetched once |

**Speed**: cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf`, fresh profile, same LibreOffice build).

| deck | Linux: `soffice` | Linux: `pptx-pdf` | macOS: `soffice` | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | **0.6-0.7 s** |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | **1.7-1.8 s** |
| 12 slides to PNG, 150 dpi | | **1.1 s** | (PDF + `pdftoppm`: 3.5 s) | **1.3 s** |

**Fidelity**: pages from the trimmed tree are pixel-identical to a full LibreOffice install on
Linux. On a real deck set in Lato, stock `soffice --headless` on macOS silently substituted
Linux Libertine; `pptx-pdf` embeds the deck's own fonts, because it lets LibreOffice see the
fonts the deck asks for instead of only the 128 it bundles. PNGs differ from `pdftoppm`'s
rendering of the same PDF only at anti-aliased edges.

## More

- [How it works](docs/how-pptx-pdf-works.md): why, the strip pipeline step by step, results, license position
- [Python](docs/python.md): install with uv, in-process vs isolated, configuration
- [Building and internals](docs/building.md): crates, fonts, build commands, end-to-end checks, status

LibreOffice is MPL-2.0 and ships in the tree unmodified apart from relocation. `pptx-pdf` is
MIT OR Apache-2.0.
