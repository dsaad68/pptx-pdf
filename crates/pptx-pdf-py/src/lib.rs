//! `pptx_pdf._native`: the Python extension behind the `pptx_pdf` package.
//!
//! LibreOfficeKit allows one `Office` per process, driven from the thread that created it.
//! Python code calls from any thread, so this module owns a dedicated worker thread that boots
//! LibreOffice lazily on the first job and serves conversions from a channel. Python callers
//! post a job and wait with the GIL released, so other Python threads keep running.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Sender};
use std::sync::{Mutex, OnceLock};

use lo_runtime::{Locator, Profile, Runtime, file_url};
use lok::{Document, Office, PageRange, PdfExportOptions};
use pyo3::exceptions::{PyFileNotFoundError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;

pyo3::create_exception!(
    _native,
    ConversionError,
    PyRuntimeError,
    "LibreOffice could not convert the document."
);
pyo3::create_exception!(
    _native,
    RuntimeUnavailable,
    PyRuntimeError,
    "No LibreOffice tree could be found or fetched."
);

/// What a Python call asks the worker to do.
enum Job {
    Convert {
        input: PathBuf,
        output: PathBuf,
        pages: Option<String>,
        notes: bool,
        font_dirs: Vec<PathBuf>,
    },
    Fetch,
    LibreOfficeVersion,
}

/// Failures, categorised so the Python side can raise the right exception type.
enum Failure {
    Input(String),
    Runtime(String),
    Conversion(String),
}

struct Request {
    job: Job,
    reply: Sender<Result<String, Failure>>,
}

/// Options that only matter for booting LibreOffice; consumed by the first boot of the process.
#[derive(Default)]
struct BootOptions {
    lo_dir: Option<PathBuf>,
    progress: Option<Py<PyAny>>,
}

/// `None` if the worker thread could not be spawned.
static WORKER: OnceLock<Option<Mutex<Sender<Request>>>> = OnceLock::new();
static BOOT: Mutex<BootOptions> = Mutex::new(BootOptions {
    lo_dir: None,
    progress: None,
});

fn worker() -> Option<&'static Mutex<Sender<Request>>> {
    WORKER
        .get_or_init(|| {
            let (tx, rx) = mpsc::channel::<Request>();
            std::thread::Builder::new()
                .name("libreoffice".into())
                .spawn(move || serve(rx))
                .ok()
                .map(|_| Mutex::new(tx))
        })
        .as_ref()
}

/// The worker loop: boot once, then serve jobs until the process ends.
fn serve(rx: mpsc::Receiver<Request>) {
    let mut office: Option<(Office, Profile)> = None;
    for Request { job, reply } in rx {
        let result = match &job {
            Job::Fetch => boot(&mut office).map(|_| String::new()),
            Job::LibreOfficeVersion => {
                boot(&mut office).map(|o| o.0.version_info().unwrap_or_default())
            }
            Job::Convert {
                input,
                output,
                pages,
                notes,
                font_dirs,
            } => boot(&mut office).and_then(|(office, profile)| {
                for dir in font_dirs {
                    profile
                        .add_fonts(dir)
                        .map_err(|e| Failure::Input(e.to_string()))?;
                }
                convert(office, input, output, pages.as_deref(), *notes)
            }),
        };
        let _ = reply.send(result);
    }
}

/// Resolve the tree and start LibreOffice if not done yet.
fn boot(slot: &mut Option<(Office, Profile)>) -> Result<&mut (Office, Profile), Failure> {
    if slot.is_none() {
        let opts = BOOT
            .lock()
            .map(|mut b| std::mem::take(&mut *b))
            .unwrap_or_default();
        let runtime = locate(&opts).map_err(Failure::Runtime)?;
        let profile = Profile::create().map_err(|e| Failure::Runtime(e.to_string()))?;
        // Precondition of configure_fontconfig is "before LibreOffice loads"; other Python
        // threads may be running, which is a documented limitation of in-process use.
        runtime
            .configure_fontconfig(&profile)
            .map_err(|e| Failure::Runtime(e.to_string()))?;
        let office = Office::load(runtime.program_dir(), profile.url())
            .map_err(|e| Failure::Runtime(e.to_string()))?;
        *slot = Some((office, profile));
    }
    slot.as_mut()
        .ok_or_else(|| Failure::Runtime(String::from("LibreOffice failed to start")))
}

fn locate(opts: &BootOptions) -> Result<Runtime, String> {
    let locator = Locator::new().explicit(opts.lo_dir.as_deref());
    let locator = with_download(locator, opts.progress.as_ref());
    locator.locate().map_err(|e| e.to_string())
}

#[cfg(feature = "download")]
fn with_download<'a>(locator: Locator<'a>, progress: Option<&'a Py<PyAny>>) -> Locator<'a> {
    use lo_runtime::{Progress, TreeSource};
    let sha = std::env::var("PPTX_PDF_LO_TREE_SHA256")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| option_env!("PPTX_PDF_LO_TREE_SHA256").map(str::to_owned));
    let source = match std::env::var("PPTX_PDF_LO_TREE_URL")
        .ok()
        .filter(|s| !s.is_empty())
    {
        Some(url) => TreeSource::new(url, sha),
        None => {
            match TreeSource::release(env!("CARGO_PKG_REPOSITORY"), env!("CARGO_PKG_VERSION"), sha)
            {
                Some(s) => s,
                None => return locator,
            }
        }
    };
    let mut total = None;
    locator.download(source).on_progress(move |event| {
        let Some(cb) = progress else { return };
        let (received, done) = match event {
            Progress::Started { total: t } => {
                total = t;
                (0, false)
            }
            Progress::Received(n) => (n, false),
            Progress::Unpacking => (total.unwrap_or(0), true),
            _ => return,
        };
        Python::attach(|py| {
            if let Err(e) = cb.bind(py).call1((received, total, done)) {
                tracing::warn!(error = %e, "progress callback failed");
            }
        });
    })
}

#[cfg(not(feature = "download"))]
fn with_download<'a>(locator: Locator<'a>, _progress: Option<&'a Py<PyAny>>) -> Locator<'a> {
    locator
}

fn convert(
    office: &Office,
    input: &Path,
    output: &Path,
    pages: Option<&str>,
    notes: bool,
) -> Result<String, Failure> {
    if !input.is_file() {
        return Err(Failure::Input(format!(
            "input {} is not a file",
            input.display()
        )));
    }
    let range = pages
        .map(PageRange::new)
        .transpose()
        .map_err(|e| Failure::Input(e.to_string()))?;
    let input_url = file_url(input).map_err(|e| Failure::Input(e.to_string()))?;
    let output_url = file_url(output).map_err(|e| Failure::Input(e.to_string()))?;
    let document: Document<'_> = office
        .load_document(&input_url)
        .map_err(|e| Failure::Conversion(e.to_string()))?;
    let options = PdfExportOptions::new().notes_pages(notes).page_range(range);
    document
        .export_pdf(&output_url, &options)
        .map_err(|e| Failure::Conversion(e.to_string()))?;
    Ok(output.display().to_string())
}

/// Post a job and wait for it with the GIL released.
fn run(py: Python<'_>, job: Job) -> PyResult<String> {
    let (reply, rx) = mpsc::channel();
    let sent = worker()
        .ok_or_else(|| PyRuntimeError::new_err("cannot spawn the LibreOffice worker thread"))?
        .lock()
        .map_err(|_| PyRuntimeError::new_err("LibreOffice worker is poisoned"))?
        .send(Request { job, reply });
    sent.map_err(|_| PyRuntimeError::new_err("LibreOffice worker has stopped"))?;
    let result = py.detach(move || rx.recv());
    match result {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(Failure::Input(m))) if m.contains("is not a file") => {
            Err(PyFileNotFoundError::new_err(m))
        }
        Ok(Err(Failure::Input(m))) => Err(PyValueError::new_err(m)),
        Ok(Err(Failure::Runtime(m))) => Err(RuntimeUnavailable::new_err(m)),
        Ok(Err(Failure::Conversion(m))) => Err(ConversionError::new_err(m)),
        Err(_) => Err(PyRuntimeError::new_err(
            "LibreOffice worker exited unexpectedly",
        )),
    }
}

/// Record boot options; only the first LibreOffice start in this process uses them.
fn set_boot(lo_dir: Option<PathBuf>, progress: Option<Py<PyAny>>) {
    if let Ok(mut b) = BOOT.lock() {
        if lo_dir.is_some() {
            b.lo_dir = lo_dir;
        }
        if progress.is_some() {
            b.progress = progress;
        }
    }
}

/// Convert `input` to PDF at `output`; returns the output path.
#[pyfunction]
#[pyo3(signature = (input, output, *, pages=None, notes=false, font_dirs=vec![], lo_dir=None))]
fn convert_py(
    py: Python<'_>,
    input: PathBuf,
    output: PathBuf,
    pages: Option<String>,
    notes: bool,
    font_dirs: Vec<PathBuf>,
    lo_dir: Option<PathBuf>,
) -> PyResult<String> {
    set_boot(lo_dir, None);
    run(
        py,
        Job::Convert {
            input,
            output,
            pages,
            notes,
            font_dirs,
        },
    )
}

/// Make sure LibreOffice is available (download or unpack the tree) and started.
#[pyfunction]
#[pyo3(signature = (*, lo_dir=None, progress=None))]
fn fetch_py(py: Python<'_>, lo_dir: Option<PathBuf>, progress: Option<Py<PyAny>>) -> PyResult<()> {
    set_boot(lo_dir, progress);
    run(py, Job::Fetch).map(|_| ())
}

/// LibreOffice's own version string (JSON), starting it if needed.
#[pyfunction]
fn libreoffice_version(py: Python<'_>) -> PyResult<String> {
    run(py, Job::LibreOfficeVersion)
}

/// Where trees and caches live.
#[pyfunction]
fn cache_dir() -> PyResult<PathBuf> {
    lo_runtime::cache::cache_dir().map_err(|e| RuntimeUnavailable::new_err(e.to_string()))
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("ConversionError", m.py().get_type::<ConversionError>())?;
    m.add(
        "RuntimeUnavailable",
        m.py().get_type::<RuntimeUnavailable>(),
    )?;
    m.add_function(wrap_pyfunction!(convert_py, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_py, m)?)?;
    m.add_function(wrap_pyfunction!(libreoffice_version, m)?)?;
    m.add_function(wrap_pyfunction!(cache_dir, m)?)?;
    Ok(())
}
