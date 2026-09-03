---
name: pptx-pdf
description: Convert PowerPoint decks (.pptx, .ppt, .odp) to PDF, or render slides to PNG images, with LibreOffice's renderer and no LibreOffice installation. Use when a task needs a faithful PDF of a presentation, page ranges or speaker notes included, or one image per slide, on a machine, in CI or in a container.
---

# pptx-pdf

One binary, LibreOffice's renderer inside (a normal `apt install libreoffice` is 2.2 GB and 708
packages; this is 4-80 MB). Output matches what LibreOffice Impress exports;
fonts installed on the machine are used, missing ones fall back to metric-compatible substitutes.

## Install (once)

```bash
curl -fsSL https://raw.githubusercontent.com/dsaad68/pptx-pdf/main/install.sh | sh          # ~80 MB, self-contained
curl -fsSL https://raw.githubusercontent.com/dsaad68/pptx-pdf/main/install.sh | PPTX_PDF_SLIM=1 sh   # ~4 MB, fetches LibreOffice on first run
```

Installs to `~/.local/bin/pptx-pdf`. Check with `pptx-pdf --version`. The first run unpacks
LibreOffice into the user cache (a few seconds; ~1 min for the slim variant on a slow link);
`pptx-pdf --fetch` does that step ahead of time.

## Convert

```bash
pptx-pdf deck.pptx                          # writes deck.pdf next to the input
pptx-pdf deck.pptx -o out/deck.pdf          # output path (directory must exist)
pptx-pdf deck.pptx --pages 2-5              # slides 2 to 5; also "1,3,5-" style ranges
pptx-pdf deck.pptx --notes                  # append notes pages (slide + speaker notes)
pptx-pdf deck.pptx --font-dir ./fonts       # extra .ttf/.otf for this run (repeatable)
pptx-pdf deck.pptx --all-fonts              # let LibreOffice see every installed font (slower start)
pptx-pdf deck.pptx -q                       # nothing on stdout; errors still on stderr
```

Success: exit 0 and the PDF path on stdout. Failure: non-zero exit, one `error: ...` line on
stderr. Common causes: input is not a file; output directory missing; a file that is not a
presentation ("failed to load document").

## Render slides as images

```bash
pptx-pdf deck.pptx --png out/                # out/slide-01.png, slide-02.png, ... at 150 dpi
pptx-pdf deck.pptx --png out/ --dpi 96       # 16:9 slide: 1280x720 px (150 dpi: 2000x1125)
pptx-pdf deck.pptx --png out/ --pages 3      # one slide
pptx-pdf deck.pptx --png out/ --hidden       # include slides marked hidden (the PDF never has them)
```

Same renderer as the PDF, no PDF in between, so this is the way to get a picture of a slide;
do not convert to PDF and rasterise that. Success: exit 0 and one path per line on stdout,
in slide order. Files are numbered by position in the deck, so with `--pages 3` the file is
`slide-03.png`. The directory is created if missing.

## Verify the result

```bash
python3 -c "import re,sys; d=open(sys.argv[1],'rb').read(); print(len(re.findall(rb'/Type\s*/Page[^s]', d)), 'pages')" deck.pdf
```

`pdfinfo`/`pdffonts` (poppler) give page count and embedded fonts when available.

## Notes for agents

- Conversion is ~0.2-1.5 s on Linux and ~0.6-2 s on macOS per deck after the first run, deck-size dependent. Run decks
  sequentially; each invocation is an independent process.
- No network is needed after the first run (slim variant) or at all (self-contained variant).
- Existing LibreOffice can be used instead of the bundled tree: `--lo-dir /path/to/LibreOffice`
  or `PPTX_PDF_LO_DIR`.
- Cache location: `~/Library/Caches/pptx-pdf` (macOS) or `~/.cache/pptx-pdf` (Linux);
  `PPTX_PDF_CACHE_DIR` overrides. Safe to delete; it is re-created on the next run.
- Python: `uv add pptx-pdf --find-links https://github.com/dsaad68/pptx-pdf/releases/expanded_assets/v0.2.3`, then `pptx_pdf.convert("deck.pptx")` or `pptx_pdf.render("deck.pptx", "out/")`.
