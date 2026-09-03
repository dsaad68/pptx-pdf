# Using pptx-pdf from Python

The `pptx_pdf` package is a thin binding over the same Rust code as the CLI. LibreOffice is
loaded in-process through LibreOfficeKit; the tree is fetched on first use into the user cache
directory (shared with the CLI) and verified against a hash pinned in the build.

## Install with uv

The package is not on PyPI. Two ways to get it from GitHub:

**Prebuilt wheel from a release** (no Rust toolchain needed). GitHub's release page doubles as
a `--find-links` index:

```bash
uv add pptx-pdf --find-links https://github.com/dsaad68/pptx-pdf/releases/expanded_assets/v0.2.3
```

or pin it in `pyproject.toml`:

```toml
[project]
dependencies = ["pptx-pdf==0.2.3"]

[tool.uv]
find-links = ["https://github.com/dsaad68/pptx-pdf/releases/expanded_assets/v0.2.3"]
```

Wheels exist for macOS Apple Silicon, Linux arm64 and Linux x86_64 (glibc 2.36+), Python 3.9+
(stable ABI, one wheel per platform).

**From source** (needs a Rust toolchain; `uv` runs maturin for you):

```bash
uv add "pptx-pdf @ git+https://github.com/dsaad68/pptx-pdf#subdirectory=crates/pptx-pdf-py"
```

See uv's guide on [adding dependencies](https://docs.astral.sh/uv/concepts/projects/dependencies/#adding-dependencies)
for the `--tag`/`--branch` forms and lockfile behaviour.

## Use

```python
import pptx_pdf

pdf = pptx_pdf.convert("deck.pptx")                      # -> Path("deck.pdf")
pptx_pdf.convert("deck.pptx", "out/deck.pdf", pages="2-5", notes=True, font_dirs=["./brand"])

pptx_pdf.fetch()            # pre-download LibreOffice (~80 MB, once per version); good in Dockerfiles
pptx_pdf.libreoffice_version()
pptx_pdf.cache_dir()
```

Errors: `FileNotFoundError` (missing input), `ValueError` (bad page range or font directory),
`pptx_pdf.RuntimeUnavailable` (no tree found or fetched), `pptx_pdf.ConversionError`
(LibreOffice rejected the document).

## In-process vs isolated

By default LibreOffice runs inside your process on a dedicated thread; calls from any thread
are serialised through it and release the GIL while waiting. That is the fast path (~0.3 s per
deck on Linux after the first call), and it has one property to know about: LibreOffice is a
large C++ program, and if it ever aborts, it takes the interpreter with it.

For long-running services, use a child interpreter instead:

```python
with pptx_pdf.Converter(isolated=True) as conv:
    conv.convert("a.pptx", "a.pdf")
    conv.convert("b.pptx", "b.pdf")   # same warm child, no LibreOffice restart

pptx_pdf.convert("c.pptx", isolated=True)   # module-level shortcut, shared child
```

The child stays warm between calls. If it dies, the call that observed it is retried once on a
fresh child; if that dies too, `ConversionError` is raised and your process is unaffected.

## Configuration

| variable | effect |
|---|---|
| `PPTX_PDF_LO_DIR` | use an existing LibreOffice (`/Applications/LibreOffice.app`, `/usr/lib/libreoffice`) instead of the tree |
| `PPTX_PDF_CACHE_DIR` | where the tree is unpacked (default: `~/Library/Caches/pptx-pdf` or `~/.cache/pptx-pdf`) |
| `PPTX_PDF_LO_TREE_URL` | fetch the tree from a mirror instead of the GitHub release |
| `PPTX_PDF_LO_TREE_SHA256` | override the expected tree hash |
| `FONTCONFIG_FILE` | your own fontconfig configuration (otherwise one is generated per run) |

`lo_dir=` on the first call does the same as `PPTX_PDF_LO_DIR`; later calls in the same process
reuse whatever LibreOffice was started first.

One caveat for `PPTX_PDF_LO_DIR` pointing at a *full* LibreOffice installation (an
`.app` or a distribution package): LibreOffice bundles its own Python, and when it decides to
initialise it inside a process that already runs Python, the two interpreters collide and
LibreOffice aborts. The trees `pptx-pdf` ships contain no Python components, so the default
setup is unaffected; with a full installation, prefer `Converter(isolated=True)` or the CLI.

## Development

```bash
cd crates/pptx-pdf-py
uv venv && uv pip install pytest maturin
.venv/bin/maturin develop --release
.venv/bin/python -m pytest
```

The tests use `PPTX_PDF_LO_DIR` (or `/Applications/LibreOffice.app` when present) and never
touch the network.
