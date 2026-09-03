//! PDF export options and their LOK filter-options encoding.
//!
//! LOK's `saveAs` takes filter data as a JSON object of typed values, e.g.
//! `{"PageRange":{"type":"string","value":"2-"}}`. The keys are the `FilterData` names
//! consumed by `filter/source/pdf/pdfexport.cxx`.

use std::fmt::Write as _;
use std::str::FromStr;

use crate::error::Error;

/// A page-range expression in the form LibreOffice's PDF export accepts: comma-separated
/// single pages or `a-b` spans, with open-ended `a-` allowed. Pages are 1-based.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageRange(String);

impl PageRange {
    /// Validate and wrap a range expression.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidPageRange`] for anything that is not `\d+`, `\d+-\d+` or `\d+-`
    /// joined by commas, or for a span whose end precedes its start.
    pub fn new(range: &str) -> Result<Self, Error> {
        let invalid = || Error::InvalidPageRange(range.to_owned());
        if range.is_empty() {
            return Err(invalid());
        }
        for part in range.split(',') {
            let (start, end) = part
                .split_once('-')
                .map_or((part, None), |(a, b)| (a, Some(b)));
            let start: u32 = parse_page(start).ok_or_else(invalid)?;
            match end {
                None | Some("") => {}
                Some(e) => {
                    let end = parse_page(e).ok_or_else(invalid)?;
                    if end < start {
                        return Err(invalid());
                    }
                }
            }
        }
        Ok(Self(range.to_owned()))
    }

    /// The validated expression.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether 1-based `page` falls inside the range.
    #[must_use]
    pub fn contains(&self, page: u32) -> bool {
        self.0.split(',').any(|part| match part.split_once('-') {
            None => parse_page(part) == Some(page),
            Some((start, "")) => parse_page(start).is_some_and(|s| page >= s),
            Some((start, end)) => matches!(
                (parse_page(start), parse_page(end)),
                (Some(s), Some(e)) if (s..=e).contains(&page)
            ),
        })
    }
}

fn parse_page(s: &str) -> Option<u32> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok().filter(|&n| n >= 1)
}

impl FromStr for PageRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl std::fmt::Display for PageRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Options for [`crate::Document::export_pdf`]. Defaults match LibreOffice's own defaults.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct PdfExportOptions {
    notes_pages: bool,
    page_range: Option<PageRange>,
    tagged: Option<bool>,
}

impl PdfExportOptions {
    /// Start from LibreOffice's defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Also emit the notes pages (slide plus speaker notes) after the slides.
    #[must_use]
    pub fn notes_pages(mut self, on: bool) -> Self {
        self.notes_pages = on;
        self
    }

    /// Restrict export to a page range.
    #[must_use]
    pub fn page_range(mut self, range: Option<PageRange>) -> Self {
        self.page_range = range;
        self
    }

    /// Force tagged (accessible) PDF on or off; `None` leaves LibreOffice's default.
    #[must_use]
    pub fn tagged(mut self, tagged: Option<bool>) -> Self {
        self.tagged = tagged;
        self
    }

    /// Encode as the JSON filter-options string LOK expects. Empty when nothing is set.
    #[must_use]
    pub fn to_filter_options(&self) -> String {
        let mut entries: Vec<String> = Vec::new();
        if self.notes_pages {
            entries.push(typed_entry("ExportNotesPages", "boolean", "true"));
        }
        if let Some(range) = &self.page_range {
            entries.push(typed_entry("PageRange", "string", range.as_str()));
        }
        if let Some(tagged) = self.tagged {
            entries.push(typed_entry(
                "UseTaggedPDF",
                "boolean",
                if tagged { "true" } else { "false" },
            ));
        }
        if entries.is_empty() {
            String::new()
        } else {
            format!("{{{}}}", entries.join(","))
        }
    }
}

fn typed_entry(key: &str, ty: &str, value: &str) -> String {
    let mut out = String::new();
    // `key` and `ty` are literals we control; only `value` needs escaping.
    let _ = write!(
        out,
        "\"{key}\":{{\"type\":\"{ty}\",\"value\":\"{}\"}}",
        json_escape(value)
    );
    out
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_ranges() {
        for r in ["1", "1-3", "2-", "1,3,5-7", "10-10"] {
            assert!(PageRange::new(r).is_ok(), "{r}");
        }
    }

    #[test]
    fn rejects_malformed_ranges() {
        for r in [
            "", "0", "a", "1-a", "-3", "3-1", "1,", ",1", "1--2", "1 - 2",
        ] {
            assert!(
                matches!(PageRange::new(r), Err(Error::InvalidPageRange(_))),
                "{r}"
            );
        }
    }

    #[test]
    fn contains_checks_every_clause() {
        let range = PageRange::new("1,3-4,7-").unwrap();
        let hits: Vec<u32> = (1..=9).filter(|&p| range.contains(p)).collect();
        assert_eq!(hits, [1, 3, 4, 7, 8, 9]);
        assert!(!PageRange::new("2").unwrap().contains(1));
    }

    #[test]
    fn default_options_encode_to_empty() {
        assert_eq!(PdfExportOptions::new().to_filter_options(), "");
    }

    #[test]
    fn encodes_all_options_as_typed_json() {
        let opts = PdfExportOptions::new()
            .notes_pages(true)
            .page_range(Some(PageRange::new("2-").unwrap()))
            .tagged(Some(false));
        assert_eq!(
            opts.to_filter_options(),
            r#"{"ExportNotesPages":{"type":"boolean","value":"true"},"PageRange":{"type":"string","value":"2-"},"UseTaggedPDF":{"type":"boolean","value":"false"}}"#
        );
    }

    #[test]
    fn escapes_json_control_characters() {
        assert_eq!(json_escape(r#"a"b\c"#), r#"a\"b\\c"#);
        assert_eq!(json_escape("x\ny\u{1}"), "x\\ny\\u0001");
    }
}
