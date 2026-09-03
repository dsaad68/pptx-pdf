"""Convert PowerPoint decks to PDF with LibreOffice's renderer, no LibreOffice install.

In-process by default: LibreOffice is loaded once per process on a dedicated thread and every
call is served from it; concurrent calls are serialised. ``Converter(isolated=True)`` moves
LibreOffice into a child interpreter that stays warm between calls, so a crash there raises
:class:`ConversionError` here instead of ending the process.
"""

from __future__ import annotations

import atexit
import os
import sys
from collections.abc import Callable, Iterable
from pathlib import Path

from . import _native
from ._native import ConversionError, RuntimeUnavailable, __version__

# LibreOffice keeps recovery state and a lock in its profile until it is destroyed; do that
# before the interpreter goes away so the persistent profile is clean for the next process.
atexit.register(_native.shutdown)

__all__ = [
    "ConversionError",
    "Converter",
    "RuntimeUnavailable",
    "__version__",
    "cache_dir",
    "convert",
    "fetch",
    "libreoffice_version",
    "render",
]

_StrPath = "str | os.PathLike[str]"


def _default_progress(received: int, total: int | None, done: bool) -> None:
    if not sys.stderr.isatty():
        return
    if done:
        sys.stderr.write("\rdownloading LibreOffice: done, unpacking\n")
    elif total:
        sys.stderr.write(f"\rdownloading LibreOffice: {received / 1048576:6.1f} / {total / 1048576:.1f} MB")
    else:
        sys.stderr.write(f"\rdownloading LibreOffice: {received / 1048576:6.1f} MB")
    sys.stderr.flush()


def fetch(
    *,
    lo_dir: _StrPath | None = None,
    progress: Callable[[int, int | None, bool], None] | None = _default_progress,
) -> Path:
    """Make sure LibreOffice is available (download or unpack the tree) and started.

    Useful in Dockerfiles and CI caches so the first :func:`convert` is fast.
    ``progress(received_bytes, total_bytes_or_None, done)`` is called during a download.
    """
    _native.fetch_py(lo_dir=os.fspath(lo_dir) if lo_dir else None, progress=progress)
    return cache_dir()


def cache_dir() -> Path:
    """Where the LibreOffice tree and font caches live (``PPTX_PDF_CACHE_DIR`` overrides)."""
    return Path(_native.cache_dir())


def libreoffice_version() -> str:
    """LibreOffice's own version information as a JSON string (starts LibreOffice if needed)."""
    return _native.libreoffice_version()


def convert(
    input: _StrPath,
    output: _StrPath | None = None,
    *,
    pages: str | None = None,
    notes: bool = False,
    font_dirs: Iterable[_StrPath] = (),
    lo_dir: _StrPath | None = None,
    isolated: bool = False,
) -> Path:
    """Convert ``input`` to a PDF and return its path.

    ``output`` defaults to the input path with a ``.pdf`` extension. ``pages`` is a range such
    as ``"1-3,5"`` or ``"4-"``; ``notes`` appends notes pages; ``font_dirs`` are directories of
    extra ``.ttf``/``.otf`` files to make available. ``lo_dir`` points at an existing LibreOffice
    installation and only matters for the first call in the process. ``isolated=True`` uses a
    shared child interpreter (see :class:`Converter`).

    Raises :class:`FileNotFoundError` for a missing input, :class:`ValueError` for a bad page
    range or font directory, :class:`RuntimeUnavailable` if no LibreOffice tree could be found
    or fetched, and :class:`ConversionError` if LibreOffice rejected the document.
    """
    if isolated:
        return _shared_isolated().convert(input, output, pages=pages, notes=notes, font_dirs=font_dirs, lo_dir=lo_dir)
    out = _resolve_output(input, output)
    _native.convert_py(
        os.fspath(input),
        os.fspath(out),
        pages=pages,
        notes=notes,
        font_dirs=[os.fspath(d) for d in font_dirs],
        lo_dir=os.fspath(lo_dir) if lo_dir else None,
    )
    return out


def render(
    input: _StrPath,
    output_dir: _StrPath,
    *,
    dpi: int = 150,
    pages: str | None = None,
    hidden: bool = False,
    font_dirs: Iterable[_StrPath] = (),
    lo_dir: _StrPath | None = None,
    isolated: bool = False,
) -> list[Path]:
    """Render ``input``'s slides as PNG files in ``output_dir`` and return their paths.

    Files are named ``slide-01.png``, ``slide-02.png``, ... by position in the deck. ``dpi``
    is pixels per inch of slide (a 16:9 slide at 150 dpi is 2000x1125). ``pages`` selects slide
    numbers such as ``"1-3,5"``; ``hidden`` includes slides marked hidden, which the PDF export
    never contains. The rendering is LibreOffice's own, the same as the PDF's, so no PDF
    rasteriser is involved. Other arguments are as for :func:`convert`.
    """
    if isolated:
        return _shared_isolated().render(
            input, output_dir, dpi=dpi, pages=pages, hidden=hidden, font_dirs=font_dirs, lo_dir=lo_dir
        )
    paths = _native.render_py(
        os.fspath(input),
        os.fspath(output_dir),
        dpi=dpi,
        pages=pages,
        hidden=hidden,
        font_dirs=[os.fspath(d) for d in font_dirs],
        lo_dir=os.fspath(lo_dir) if lo_dir else None,
    )
    return [Path(p) for p in paths]


def _resolve_output(input: _StrPath, output: _StrPath | None) -> Path:
    return Path(output) if output is not None else Path(input).with_suffix(".pdf")


class Converter:
    """A conversion context.

    ``isolated=False`` (default) converts in this process. ``isolated=True`` keeps a child
    interpreter running LibreOffice; a crash there is reported as :class:`ConversionError` and
    the child is restarted on the next call. Use as a context manager to stop the child early.
    """

    def __init__(self, *, isolated: bool = False) -> None:
        self.isolated = isolated
        self._child: list = [None]  # one-element slot so the worker module can replace it

    def convert(
        self,
        input: _StrPath,
        output: _StrPath | None = None,
        *,
        pages: str | None = None,
        notes: bool = False,
        font_dirs: Iterable[_StrPath] = (),
        lo_dir: _StrPath | None = None,
    ) -> Path:
        """As :func:`convert`, in this converter's context."""
        if not self.isolated:
            return convert(input, output, pages=pages, notes=notes, font_dirs=font_dirs, lo_dir=lo_dir)
        out = _resolve_output(input, output)
        request = {
            "op": "convert",
            "input": os.fspath(input),
            "output": os.fspath(out),
            "pages": pages,
            "notes": notes,
            "font_dirs": [os.fspath(d) for d in font_dirs],
            "lo_dir": os.fspath(lo_dir) if lo_dir else None,
        }
        self._request(request)
        return out

    def render(
        self,
        input: _StrPath,
        output_dir: _StrPath,
        *,
        dpi: int = 150,
        pages: str | None = None,
        hidden: bool = False,
        font_dirs: Iterable[_StrPath] = (),
        lo_dir: _StrPath | None = None,
    ) -> list[Path]:
        """As :func:`render`, in this converter's context."""
        if not self.isolated:
            return render(input, output_dir, dpi=dpi, pages=pages, hidden=hidden, font_dirs=font_dirs, lo_dir=lo_dir)
        request = {
            "op": "render",
            "input": os.fspath(input),
            "output_dir": os.fspath(output_dir),
            "dpi": dpi,
            "pages": pages,
            "hidden": hidden,
            "font_dirs": [os.fspath(d) for d in font_dirs],
            "lo_dir": os.fspath(lo_dir) if lo_dir else None,
        }
        reply = self._request(request)
        return [Path(p) for p in reply.get("paths", [])]

    def close(self) -> None:
        """Stop the child interpreter, if any."""
        child, self._child[0] = self._child[0], None
        if child is not None:
            child.stop()

    def __enter__(self) -> Converter:
        return self

    def __exit__(self, *exc: object) -> None:
        self.close()

    def _request(self, request: dict) -> dict:
        from ._worker import request as run_request

        return run_request(self._child, request)


_isolated_singleton: Converter | None = None


def _shared_isolated() -> Converter:
    global _isolated_singleton
    if _isolated_singleton is None:
        _isolated_singleton = Converter(isolated=True)
    return _isolated_singleton
