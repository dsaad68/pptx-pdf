import re
import threading
from pathlib import Path

import pytest

import pptx_pdf
from conftest import needs_lo


def pages(pdf: Path) -> int:
    return len(re.findall(rb"/Type\s*/Page[^s]", pdf.read_bytes()))


def test_version_and_cache_dir():
    assert pptx_pdf.__version__.count(".") == 2
    assert isinstance(pptx_pdf.cache_dir(), Path)


@needs_lo
def test_convert_writes_a_pdf(deck, tmp_path):
    out = pptx_pdf.convert(deck, tmp_path / "out.pdf")
    assert out == tmp_path / "out.pdf"
    assert out.read_bytes().startswith(b"%PDF-")
    assert pages(out) == 6


@needs_lo
def test_default_output_next_to_input(deck, tmp_path):
    src = tmp_path / "deck.pptx"
    src.write_bytes(deck.read_bytes())
    assert pptx_pdf.convert(src) == tmp_path / "deck.pdf"
    assert (tmp_path / "deck.pdf").is_file()


@needs_lo
def test_pages_and_notes(deck, tmp_path):
    assert pages(pptx_pdf.convert(deck, tmp_path / "r.pdf", pages="2-3")) == 2
    assert pages(pptx_pdf.convert(deck, tmp_path / "n.pdf", notes=True)) == 12


def png_size(path: Path) -> tuple[int, int]:
    head = path.read_bytes()[:24]
    assert head.startswith(b"\x89PNG\r\n\x1a\n")
    return int.from_bytes(head[16:20], "big"), int.from_bytes(head[20:24], "big")


@needs_lo
def test_render_writes_one_png_per_slide(deck, tmp_path):
    out = pptx_pdf.render(deck, tmp_path / "png", dpi=72)
    assert [p.name for p in out] == [f"slide-{i:02d}.png" for i in range(1, 7)]
    w, h = png_size(out[0])
    assert w in (720, 960) and h in (405, 540, 720)  # 10x7.5 or 13.33x7.5 inch slide at 72 dpi
    assert pptx_pdf.render(deck, tmp_path / "png2", pages="2,5-") == [
        tmp_path / "png2" / n for n in ("slide-02.png", "slide-05.png", "slide-06.png")
    ]


@needs_lo
def test_render_rejects_bad_dpi(deck, tmp_path):
    with pytest.raises(ValueError):
        pptx_pdf.render(deck, tmp_path / "x", dpi=0)


@needs_lo
def test_missing_input_raises_file_not_found(tmp_path):
    with pytest.raises(FileNotFoundError):
        pptx_pdf.convert(tmp_path / "nope.pptx", tmp_path / "x.pdf")


@needs_lo
def test_bad_page_range_raises_value_error(deck, tmp_path):
    with pytest.raises(ValueError):
        pptx_pdf.convert(deck, tmp_path / "x.pdf", pages="3-1")


@needs_lo
def test_garbage_input_raises_conversion_error(tmp_path):
    bad = tmp_path / "bad.pptx"
    bad.write_bytes(b"PK\x03\x04" + b"\x00" * 64)  # a truncated zip, not a text file
    with pytest.raises(pptx_pdf.ConversionError):
        pptx_pdf.convert(bad, tmp_path / "x.pdf")


@needs_lo
def test_concurrent_calls_are_serialised(deck, tmp_path):
    results, errors = [], []

    def work(i):
        try:
            results.append(pages(pptx_pdf.convert(deck, tmp_path / f"t{i}.pdf")))
        except Exception as e:  # noqa: BLE001
            errors.append(e)

    threads = [threading.Thread(target=work, args=(i,)) for i in range(3)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    assert errors == []
    assert results == [6, 6, 6]


@needs_lo
def test_libreoffice_version_is_json():
    assert '"ProductName"' in pptx_pdf.libreoffice_version()


@needs_lo
def test_isolated_converts_and_survives_a_crash(deck, tmp_path):
    with pptx_pdf.Converter(isolated=True) as conv:
        assert pages(conv.convert(deck, tmp_path / "a.pdf")) == 6
        conv._child[0]._proc.kill()  # simulate LibreOffice taking the child down
        assert pages(conv.convert(deck, tmp_path / "b.pdf")) == 6  # restarted transparently
        with pytest.raises(FileNotFoundError):
            conv.convert(tmp_path / "missing.pptx", tmp_path / "c.pdf")
        with pytest.raises(ValueError):
            conv.convert(deck, tmp_path / "d.pdf", pages="x")


@needs_lo
def test_isolated_module_level(deck, tmp_path):
    assert pages(pptx_pdf.convert(deck, tmp_path / "iso.pdf", isolated=True)) == 6
    out = pptx_pdf.render(deck, tmp_path / "iso-png", pages="1", isolated=True)
    assert [p.name for p in out] == ["slide-01.png"] and out[0].is_file()
