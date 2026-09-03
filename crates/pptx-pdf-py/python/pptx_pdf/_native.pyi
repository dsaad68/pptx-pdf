from collections.abc import Callable

__version__: str

class ConversionError(RuntimeError): ...
class RuntimeUnavailable(RuntimeError): ...

def convert_py(
    input: str,
    output: str,
    *,
    pages: str | None = None,
    notes: bool = False,
    font_dirs: list[str] = ...,
    lo_dir: str | None = None,
) -> str: ...
def render_py(
    input: str,
    output_dir: str,
    *,
    dpi: int = 150,
    pages: str | None = None,
    hidden: bool = False,
    font_dirs: list[str] = ...,
    lo_dir: str | None = None,
) -> list[str]: ...
def fetch_py(*, lo_dir: str | None = None, progress: Callable[[int, int | None, bool], None] | None = None) -> None: ...
def libreoffice_version() -> str: ...
def cache_dir() -> str: ...
def shutdown() -> None: ...
