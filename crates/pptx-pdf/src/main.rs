//! `pptx-pdf`: convert a PowerPoint deck to PDF through an in-process LibreOffice.

mod cli;
mod tree;

use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use anyhow::{Context as _, bail};
use clap::Parser as _;
use lo_runtime::{FontPolicy, Locator, Profile, file_url};
use lok::{DocumentType, Office, PdfExportOptions};

use crate::cli::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();
    init_tracing(cli.verbose, cli.quiet);
    match run(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            // Errors are reported once, here; lower layers only attach context.
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: &Cli) -> anyhow::Result<()> {
    let started = Instant::now();
    let locator = tree::with_download(
        Locator::new().explicit(cli.lo_dir.as_deref()),
        cli.lo_tree_url.clone(),
        cli.lo_tree_sha256.clone(),
        cli.quiet,
    );
    if cli.fetch {
        let runtime = locator.locate().context("fetching LibreOffice runtime")?;
        tracing::info!(elapsed_ms = started.elapsed().as_millis(), root = %runtime.install_root().display(), "ready");
        if !cli.quiet {
            println!("{}", runtime.install_root().display());
        }
        return Ok(());
    }
    let input = cli.input.as_deref().context("an input file is required")?;
    if !input.is_file() {
        bail!("input {} is not a file", input.display());
    }
    let output = cli.output.clone().unwrap_or_else(|| default_output(input));
    if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty())
        && !parent.is_dir()
    {
        bail!("output directory {} does not exist", parent.display());
    }

    let runtime = locator.locate().context("locating LibreOffice runtime")?;
    // Extra fonts are per run, so they get a throw-away profile; otherwise reuse the
    // persistent one and its font cache.
    let profile = if cli.font_dir.is_empty() {
        Profile::acquire().context("acquiring user profile")?
    } else {
        Profile::create().context("creating user profile")?
    };
    for dir in &cli.font_dir {
        let n = profile
            .add_fonts(dir)
            .with_context(|| format!("adding fonts from {}", dir.display()))?;
        tracing::info!(dir = %dir.display(), fonts = n, "added fonts");
    }

    // Single-threaded here, before LibreOffice loads: the precondition `configure_fontconfig` documents.
    let policy = font_policy(cli, input);
    let fonts = runtime
        .configure_fontconfig(&profile, &policy)
        .context("configuring fontconfig")?;
    tracing::debug!(?fonts, "fontconfig");
    // A persistent profile that LibreOffice cannot start from (left inconsistent by an earlier
    // abort) is not worth failing over: retry once with a fresh temporary one.
    let (office, _profile) = match Office::load(runtime.program_dir(), profile.url()) {
        Ok(office) => (office, profile),
        Err(e) if profile.is_persistent() => {
            tracing::warn!(error = %e, "persistent profile unusable; retrying with a temporary one");
            let fresh = Profile::create().context("creating user profile")?;
            runtime
                .configure_fontconfig(&fresh, &policy)
                .context("configuring fontconfig")?;
            let office =
                Office::load(runtime.program_dir(), fresh.url()).context("starting LibreOffice")?;
            (office, fresh)
        }
        Err(e) => return Err(e).context("starting LibreOffice"),
    };
    tracing::info!(
        elapsed_ms = started.elapsed().as_millis(),
        "LibreOffice ready"
    );

    let input_url = file_url(input).context("building input URL")?;
    let output_url = file_url(&output).context("building output URL")?;
    let document = office
        .load_document(&input_url)
        .with_context(|| format!("opening {}", input.display()))?;
    let kind = document.document_type();
    if !matches!(kind, DocumentType::Presentation | DocumentType::Drawing) {
        tracing::warn!(%kind, "input is not a presentation; exporting anyway");
    }

    let options = PdfExportOptions::new()
        .notes_pages(cli.notes)
        .page_range(cli.pages.clone());
    document
        .export_pdf(&output_url, &options)
        .with_context(|| format!("writing {}", output.display()))?;
    tracing::info!(elapsed_ms = started.elapsed().as_millis(), "done");

    if !cli.quiet {
        // Piping into `head` may close stdout early; that is not an error for a converter.
        let mut stdout = io::stdout().lock();
        if let Err(e) = writeln!(stdout, "{}", output.display())
            && e.kind() != io::ErrorKind::BrokenPipe
        {
            return Err(e).context("writing to stdout");
        }
    }
    Ok(())
}

/// Only the deck's families unless `--all-fonts`; a deck we cannot read the families of (a
/// binary `.ppt`, say) gets every font.
fn font_policy(cli: &Cli, input: &Path) -> FontPolicy {
    if cli.all_fonts {
        return FontPolicy::All;
    }
    match lo_runtime::deckfonts::scan(input) {
        Ok(fonts) if !fonts.families.is_empty() => {
            tracing::debug!(families = ?fonts.families, langs = ?fonts.langs, "deck fonts");
            FontPolicy::from(fonts)
        }
        Ok(_) => FontPolicy::All,
        Err(e) => {
            tracing::debug!(error = %e, "could not read deck fonts; showing all");
            FontPolicy::All
        }
    }
}

fn default_output(input: &Path) -> PathBuf {
    input.with_extension("pdf")
}

fn init_tracing(verbose: u8, quiet: bool) {
    use tracing_subscriber::EnvFilter;
    let level = match (quiet, verbose) {
        (true, _) => "error",
        (false, 0) => "warn",
        (false, 1) => "info",
        (false, 2) => "debug",
        (false, _) => "trace",
    };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(io::stderr)
        .with_target(false)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_output_swaps_extension() {
        assert_eq!(
            default_output(Path::new("/a/deck.pptx")),
            PathBuf::from("/a/deck.pdf")
        );
        assert_eq!(default_output(Path::new("deck")), PathBuf::from("deck.pdf"));
        assert_eq!(
            default_output(Path::new("a.b.PPTX")),
            PathBuf::from("a.b.pdf")
        );
    }
}
