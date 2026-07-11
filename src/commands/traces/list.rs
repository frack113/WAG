// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::traces::{TRACE_METADATA, TraceMetadata};
use clap::Args;
use serde::Deserialize;
use std::{fmt::Write, process::ExitCode};

const NO_MATCH_MESSAGE: &str = "No traces match the selected filters.\n";
const SEPARATOR: &str = " | ";
const IDENTIFIER_HEADER: &str = "Identifier";
const NAME_HEADER: &str = "Name";
const REQUIREMENTS_SUMMARY_HEADER: &str = "Requirements summary";

#[derive(Args, Deserialize)]
pub struct List {
    #[clap(long = "attack")]
    attack_techniques: Vec<String>,

    #[clap(long = "sigma")]
    sigma_identifiers: Vec<String>,

    #[clap(long = "use-case")]
    use_cases: Vec<String>,
}

pub struct TraceListFilters<'a> {
    pub attack_techniques: &'a [&'a str],
    pub sigma_identifiers: &'a [&'a str],
    pub use_cases: &'a [&'a str],
}

impl List {
    pub fn run(&self) -> ExitCode {
        let width = console::Term::stdout().size().1 as usize;

        let attack_techniques: Vec<&str> =
            self.attack_techniques.iter().map(String::as_str).collect();
        let sigma_identifiers: Vec<&str> =
            self.sigma_identifiers.iter().map(String::as_str).collect();
        let use_cases: Vec<&str> = self.use_cases.iter().map(String::as_str).collect();

        let filters = TraceListFilters {
            attack_techniques: &attack_techniques,
            sigma_identifiers: &sigma_identifiers,
            use_cases: &use_cases,
        };

        let output = Self::render_filtered(width.max(1), &TRACE_METADATA, &filters);

        print!("{output}");

        ExitCode::SUCCESS
    }

    pub fn render_filtered(
        width: usize,
        traces: &[&TraceMetadata],
        filters: &TraceListFilters,
    ) -> String {
        if traces.is_empty() {
            return "No traces are available.\n".to_string();
        }

        if !Self::has_active_filters(filters) {
            return Self::render(width, traces);
        }

        let filtered = Self::filter(traces, filters);
        let context_line = Self::render_filter_context(filters);
        if filtered.is_empty() {
            return format!("{context_line}\n{NO_MATCH_MESSAGE}");
        }

        let table = Self::render(width, &filtered);

        let mut output = String::new();
        let _ = write!(output, "{context_line}\n{table}");
        output
    }

    pub fn filter<'a>(
        traces: &'a [&'a TraceMetadata],
        filters: &TraceListFilters,
    ) -> Vec<&'a TraceMetadata> {
        traces
            .iter()
            .copied()
            .filter(|entry| Self::matches_all_filters(entry, filters))
            .collect()
    }

    pub fn render(width: usize, traces: &[&TraceMetadata]) -> String {
        if traces.is_empty() {
            return "No traces are available.\n".to_string();
        }

        let width = width.max(1);
        let identifier_width = traces
            .iter()
            .map(|entry| entry.identifier.as_str().len())
            .chain([IDENTIFIER_HEADER.len()])
            .max()
            .unwrap_or(1);
        let name_width = traces
            .iter()
            .map(|entry| entry.name.len())
            .chain([NAME_HEADER.len()])
            .max()
            .unwrap_or(1);
        let requirements_width = traces
            .iter()
            .map(|entry| entry.requirements_summary.len())
            .chain([REQUIREMENTS_SUMMARY_HEADER.len()])
            .max()
            .unwrap_or(1);

        let three_column_width =
            identifier_width + SEPARATOR.len() + name_width + SEPARATOR.len() + requirements_width;
        if three_column_width <= width {
            return Self::render_three_columns(
                traces,
                identifier_width,
                name_width,
                requirements_width,
            );
        }

        let minimum_two_column_width = 1 + SEPARATOR.len() + NAME_HEADER.len();
        if minimum_two_column_width <= width {
            let available = width.saturating_sub(SEPARATOR.len());
            let identifier_width = identifier_width
                .min(available.saturating_sub(NAME_HEADER.len()))
                .max(1);
            let name_width = available.saturating_sub(identifier_width).max(1);

            return Self::render_two_columns(traces, identifier_width, name_width);
        }

        Self::render_identifier_column(traces, width)
    }

    fn render_three_columns(
        traces: &[&TraceMetadata],
        identifier_width: usize,
        name_width: usize,
        requirements_width: usize,
    ) -> String {
        let mut output = String::new();
        Self::render_row(
            &mut output,
            [IDENTIFIER_HEADER, NAME_HEADER, REQUIREMENTS_SUMMARY_HEADER],
            [identifier_width, name_width, requirements_width],
        );

        for entry in traces {
            let identifier = entry.identifier.as_str();
            Self::render_row(
                &mut output,
                [identifier.as_str(), entry.name, entry.requirements_summary],
                [identifier_width, name_width, requirements_width],
            );
        }

        output
    }

    fn render_two_columns(
        traces: &[&TraceMetadata],
        identifier_width: usize,
        name_width: usize,
    ) -> String {
        let mut output = String::new();
        Self::render_row(
            &mut output,
            [IDENTIFIER_HEADER, NAME_HEADER],
            [identifier_width, name_width],
        );

        for entry in traces {
            let identifier = entry.identifier.as_str();
            Self::render_row(
                &mut output,
                [identifier.as_str(), entry.name],
                [identifier_width, name_width],
            );
        }

        output
    }

    fn render_identifier_column(traces: &[&TraceMetadata], width: usize) -> String {
        let mut output = String::new();
        let identifier_width = width.max(1);
        Self::render_row(&mut output, [IDENTIFIER_HEADER], [identifier_width]);

        for entry in traces {
            let identifier = entry.identifier.as_str();
            Self::render_row(&mut output, [identifier.as_str()], [identifier_width]);
        }

        output
    }

    fn render_row<const COLUMNS: usize>(
        output: &mut String,
        values: [&str; COLUMNS],
        widths: [usize; COLUMNS],
    ) {
        let wrapped_columns: Vec<Vec<String>> = values
            .into_iter()
            .zip(widths)
            .map(|(value, width)| Self::wrap_cell(value, width))
            .collect();

        let line_count = wrapped_columns.iter().map(Vec::len).max().unwrap_or(0);
        for line_index in 0..line_count {
            for (column_index, column_lines) in wrapped_columns.iter().enumerate() {
                if column_index > 0 {
                    let _ = output.write_str(SEPARATOR);
                }

                let cell = column_lines
                    .get(line_index)
                    .map(String::as_str)
                    .unwrap_or("");
                let width = widths[column_index];
                let _ = write!(output, "{cell:<width$}");
            }

            output.push('\n');
        }
    }

    fn wrap_cell(value: &str, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![String::new()];
        }

        let mut lines = Vec::new();
        let mut chars = value.chars();

        loop {
            let chunk: String = chars.by_ref().take(width).collect();
            if chunk.is_empty() {
                break;
            }

            lines.push(chunk);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    fn matches_all_filters(entry: &TraceMetadata, filters: &TraceListFilters) -> bool {
        if !filters.attack_techniques.is_empty()
            && !filters.attack_techniques.iter().any(|filter| {
                entry
                    .attack_techniques
                    .iter()
                    .any(|target| target.as_str() == *filter)
            })
        {
            return false;
        }

        if !filters.sigma_identifiers.is_empty()
            && !filters.sigma_identifiers.iter().any(|filter| {
                entry
                    .sigma_identifiers
                    .iter()
                    .any(|target| target.as_str() == *filter)
            })
        {
            return false;
        }

        if !filters.use_cases.is_empty()
            && !filters.use_cases.iter().any(|filter| {
                entry
                    .use_cases
                    .iter()
                    .any(|target| target.as_str() == *filter)
            })
        {
            return false;
        }

        true
    }

    fn has_active_filters(filters: &TraceListFilters) -> bool {
        !filters.attack_techniques.is_empty()
            || !filters.sigma_identifiers.is_empty()
            || !filters.use_cases.is_empty()
    }

    fn render_filter_context(filters: &TraceListFilters) -> String {
        let mut parts: Vec<String> = Vec::new();

        if !filters.attack_techniques.is_empty() {
            let joined = filters.attack_techniques.join(", ");
            parts.push(format!("attack={joined}"));
        }

        if !filters.sigma_identifiers.is_empty() {
            let joined = filters.sigma_identifiers.join(", ");
            parts.push(format!("sigma={joined}"));
        }

        if !filters.use_cases.is_empty() {
            let joined = filters.use_cases.join(", ");
            parts.push(format!("use-case={joined}"));
        }

        format!("filters: {}", parts.join(" "))
    }
}
