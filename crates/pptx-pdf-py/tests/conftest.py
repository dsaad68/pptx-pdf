import atexit
import os
import shutil
from pathlib import Path

import pytest

REPO = Path(__file__).resolve().parents[3]
CORPUS = REPO / "tools" / "lo-tree" / "corpus"

# Tests never touch the network: point the slim download at a dead URL and use an existing
# LibreOffice (PPTX_PDF_LO_DIR, or LibreOffice.app on a Mac) for real conversions.
os.environ.setdefault("PPTX_PDF_LO_TREE_URL", "http://127.0.0.1:9/unreachable.tar.zst")
if "PPTX_PDF_LO_DIR" not in os.environ and Path("/Applications/LibreOffice.app").is_dir():
    os.environ["PPTX_PDF_LO_DIR"] = "/Applications/LibreOffice.app"
# A fresh cache (and so a fresh persistent profile) per session: a full LibreOffice.app under
# test would otherwise initialise its bundled Python on a reused profile and collide with the
# interpreter running the tests. Shipped trees carry no Python and do not have this problem.
_cache = Path(os.environ.get("TMPDIR", "/tmp")) / f"pptx-pdf-pytest-cache-{os.getpid()}"
shutil.rmtree(_cache, ignore_errors=True)
os.environ["PPTX_PDF_CACHE_DIR"] = str(_cache)
atexit.register(shutil.rmtree, _cache, True)

HAVE_LO = "PPTX_PDF_LO_DIR" in os.environ


@pytest.fixture(scope="session")
def deck() -> Path:
    path = CORPUS / "text_fonts.pptx"
    if not path.is_file():
        pytest.skip("corpus not generated (tools/lo-tree/build.sh)")
    return path


needs_lo = pytest.mark.skipif(not HAVE_LO, reason="no LibreOffice available (set PPTX_PDF_LO_DIR)")
