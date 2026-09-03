# pptx-pdf (Python)

```python
import pptx_pdf

pptx_pdf.convert("deck.pptx", "deck.pdf", pages="2-5", notes=True, font_dirs=["./brand"])
pptx_pdf.render("deck.pptx", "slides/", dpi=150)   # -> [Path("slides/slide-01.png"), ...]
pptx_pdf.fetch()   # pre-download LibreOffice (about 80 MB, once per version)
```

LibreOffice's renderer, loaded in-process through LibreOfficeKit; nothing to install. The
tree is fetched on first use into the user cache directory (shared with the `pptx-pdf` CLI)
and verified against a hash pinned in the build. Set `PPTX_PDF_LO_DIR` to use an existing
LibreOffice installation instead.

`pptx_pdf.Converter(isolated=True)` runs conversions in a child interpreter that stays warm,
so a LibreOffice crash cannot take your process down. See the project README for details.
