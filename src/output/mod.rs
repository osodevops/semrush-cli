pub mod csv;
pub mod json;
pub mod jsonl;
pub mod table;

use std::io::IsTerminal;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Json,
    Table,
    Csv,
    Jsonl,
}

impl OutputFormat {
    pub fn from_str_or_detect(s: Option<&str>) -> Self {
        match s {
            Some("json") => OutputFormat::Json,
            Some("table") => OutputFormat::Table,
            Some("csv") => OutputFormat::Csv,
            Some("jsonl") => OutputFormat::Jsonl,
            Some(_) => OutputFormat::Json,
            None => {
                // Auto-detect: table for TTY, JSON for pipes
                if std::io::stdout().is_terminal() {
                    OutputFormat::Table
                } else {
                    OutputFormat::Json
                }
            }
        }
    }
}

/// Render data rows with the given format and metadata.
pub fn render(
    format: OutputFormat,
    data: &[serde_json::Value],
    meta: &serde_json::Value,
) -> String {
    match format {
        OutputFormat::Json => json::render(data, meta),
        OutputFormat::Table => table::render(data),
        OutputFormat::Csv => csv::render(data),
        OutputFormat::Jsonl => jsonl::render(data),
    }
}
