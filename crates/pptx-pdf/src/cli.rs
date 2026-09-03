//! Command-line surface.

use std::path::PathBuf;

use clap::Parser;
use lok::PageRange;

/// Convert a PowerPoint deck to PDF.
#[derive(Debug, Parser)]
#[command(name = "pptx-pdf", version, about, long_about = None)]
pub(crate) struct Cli {
    /// Presentation to convert (.pptx, .ppt, .odp, ...).
    #[arg(required_unless_present = "fetch")]
    pub(crate) input: Option<PathBuf>,

    /// Where to write the PDF. Defaults to the input path with a .pdf extension.
    #[arg(short, long, value_name = "FILE", conflicts_with = "png")]
    pub(crate) output: Option<PathBuf>,

    /// Render slides as PNG files into this directory (slide-01.png, slide-02.png, ...)
    /// instead of writing a PDF.
    #[arg(long, value_name = "DIR", conflicts_with = "notes")]
    pub(crate) png: Option<PathBuf>,

    /// Resolution of the PNG files in pixels per inch of slide (a 16:9 slide at 150 dpi is
    /// 2000x1125 pixels).
    #[arg(long, value_name = "N", default_value_t = 150, requires = "png")]
    pub(crate) dpi: u32,

    /// Also render slides marked hidden in the deck (PNG only; the PDF never has them).
    #[arg(long, requires = "png")]
    pub(crate) hidden: bool,

    /// LibreOffice installation to use instead of the embedded or downloaded one.
    #[arg(long, value_name = "DIR", env = lo_runtime::ENV_LO_DIR)]
    pub(crate) lo_dir: Option<PathBuf>,

    /// Only make sure the LibreOffice tree is available (download or unpack it), then exit.
    #[arg(long)]
    pub(crate) fetch: bool,

    /// Where to download the LibreOffice tree from (slim builds). Defaults to this release's
    /// asset on GitHub; a SHA256SUMS next to it is used when no hash is pinned.
    #[arg(long, value_name = "URL", env = "PPTX_PDF_LO_TREE_URL", hide = true)]
    pub(crate) lo_tree_url: Option<String>,

    /// Expected SHA-256 of the downloaded tree, overriding the pinned one.
    #[arg(long, value_name = "HEX", env = "PPTX_PDF_LO_TREE_SHA256", hide = true)]
    pub(crate) lo_tree_sha256: Option<String>,

    /// Append notes pages (slide plus speaker notes) after the slides.
    #[arg(long)]
    pub(crate) notes: bool,

    /// Export only these slides, e.g. "1-3,5" or "4-". Numbers count slides in the deck.
    #[arg(long, value_name = "RANGE")]
    pub(crate) pages: Option<PageRange>,

    /// Let LibreOffice see every installed font instead of only the families the deck uses
    /// (slower start; try it if a fallback glyph for an unusual script looks wrong).
    #[arg(long)]
    pub(crate) all_fonts: bool,

    /// Directory of extra .ttf/.otf fonts to make available for this conversion. Repeatable.
    #[arg(long, value_name = "DIR")]
    pub(crate) font_dir: Vec<PathBuf>,

    /// Print nothing but errors.
    #[arg(short, long, conflicts_with = "verbose")]
    pub(crate) quiet: bool,

    /// Increase log detail on stderr (-v info, -vv debug, -vvv trace).
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory as _;

    #[test]
    fn command_definition_is_consistent() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_page_range_and_flags() {
        let cli = Cli::parse_from(["pptx-pdf", "deck.pptx", "--pages", "2-4", "--notes", "-vv"]);
        assert_eq!(cli.pages.as_ref().map(PageRange::as_str), Some("2-4"));
        assert!(cli.notes);
        assert_eq!(cli.verbose, 2);
    }

    #[test]
    fn png_mode_flags() {
        let cli = Cli::parse_from([
            "pptx-pdf",
            "deck.pptx",
            "--png",
            "out",
            "--dpi",
            "96",
            "--hidden",
        ]);
        assert_eq!(cli.png.as_deref(), Some(std::path::Path::new("out")));
        assert_eq!(cli.dpi, 96);
        assert!(cli.hidden);
        assert!(
            Cli::try_parse_from(["pptx-pdf", "deck.pptx", "--dpi", "96"]).is_err(),
            "--dpi needs --png"
        );
        assert!(Cli::try_parse_from(["pptx-pdf", "deck.pptx", "--png", "out", "--notes"]).is_err());
        assert!(
            Cli::try_parse_from(["pptx-pdf", "deck.pptx", "--png", "out", "-o", "x.pdf"]).is_err()
        );
    }

    #[test]
    fn rejects_bad_page_range() {
        assert!(Cli::try_parse_from(["pptx-pdf", "deck.pptx", "--pages", "3-1"]).is_err());
    }

    #[test]
    fn quiet_and_verbose_conflict() {
        assert!(Cli::try_parse_from(["pptx-pdf", "deck.pptx", "-q", "-v"]).is_err());
    }

    #[test]
    fn fetch_needs_no_input_but_input_is_otherwise_required() {
        assert!(Cli::try_parse_from(["pptx-pdf", "--fetch"]).is_ok());
        assert!(Cli::try_parse_from(["pptx-pdf"]).is_err());
    }
}
