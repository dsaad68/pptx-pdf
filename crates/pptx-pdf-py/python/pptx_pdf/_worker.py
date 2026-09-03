"""Child-interpreter protocol for ``Converter(isolated=True)``.

The parent starts ``python -m pptx_pdf._worker``; each request is one JSON line on stdin and
each reply one JSON line on stdout. LibreOffice lives in the child, so if it aborts, the parent
sees a closed pipe and raises :class:`ConversionError` rather than dying.
"""

from __future__ import annotations

import json
import subprocess
import sys
from typing import Any


class Child:
    """A warm child interpreter serving conversions."""

    def __init__(self) -> None:
        self._proc = subprocess.Popen(
            [sys.executable, "-m", "pptx_pdf._worker"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True,
            bufsize=1,
        )

    def alive(self) -> bool:
        return self._proc.poll() is None

    def stop(self) -> None:
        if self.alive():
            try:
                self._proc.stdin.close()
                self._proc.wait(timeout=5)
            except Exception:
                self._proc.kill()

    def exchange(self, request: dict[str, Any]) -> dict[str, Any] | None:
        """Send one request; ``None`` means the child died before replying."""
        try:
            self._proc.stdin.write(json.dumps(request) + "\n")
            self._proc.stdin.flush()
            line = self._proc.stdout.readline()
        except (BrokenPipeError, OSError):
            line = ""
        if not line:
            self._proc.kill()
            self._proc.wait()
            return None
        return json.loads(line)


def request(child_slot: list, request: dict[str, Any]) -> dict[str, Any]:
    """Run a request on the child in ``child_slot[0]``, restarting it once if it has died.

    A child that died while idle, or during this very request, is replaced and the request
    retried once; if the fresh child dies too, the document is the problem and
    :class:`ConversionError` is raised.
    """
    from . import ConversionError, RuntimeUnavailable

    reply = None
    for attempt in (1, 2):
        child = child_slot[0]
        if child is None or not child.alive():
            child = child_slot[0] = Child()
        reply = child.exchange(request)
        if reply is not None:
            break
        child_slot[0] = None
        if attempt == 2:
            raise ConversionError("LibreOffice worker process died twice on this document")
    if reply.get("ok"):
        return reply
    kind, message = reply.get("error", "ConversionError"), reply.get("message", "")
    exc = {
        "FileNotFoundError": FileNotFoundError,
        "ValueError": ValueError,
        "RuntimeUnavailable": RuntimeUnavailable,
    }.get(kind, ConversionError)
    raise exc(message)


def _serve() -> None:
    from . import _native

    for line in sys.stdin:
        try:
            req = json.loads(line)
            if req.get("op", "convert") == "render":
                paths = _native.render_py(
                    req["input"],
                    req["output_dir"],
                    dpi=int(req.get("dpi", 150)),
                    pages=req.get("pages"),
                    hidden=bool(req.get("hidden", False)),
                    font_dirs=list(req.get("font_dirs", [])),
                    lo_dir=req.get("lo_dir"),
                )
                reply = {"ok": True, "paths": paths}
            else:
                _native.convert_py(
                    req["input"],
                    req["output"],
                    pages=req.get("pages"),
                    notes=bool(req.get("notes", False)),
                    font_dirs=list(req.get("font_dirs", [])),
                    lo_dir=req.get("lo_dir"),
                )
                reply = {"ok": True}
        except Exception as e:  # noqa: BLE001 - everything crosses the pipe as a reply
            reply = {"ok": False, "error": type(e).__name__, "message": str(e)}
        sys.stdout.write(json.dumps(reply) + "\n")
        sys.stdout.flush()


if __name__ == "__main__":
    _serve()
