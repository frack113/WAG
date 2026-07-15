// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    metadata::{AttackTechnique, SigmaIdentifier},
    registry::{REGISTRY, TraceFilters},
    traces::TraceDefinition,
};
use clap::Args;
use std::{fmt::Write, process::ExitCode};

const NO_MATCH_MESSAGE: &str = "No traces match the selected filters.\n";
const SEPARATOR: &str = " | ";
const IDENTIFIER_HEADER: &str = "Identifier";
const NAME_HEADER: &str = "Name";
const DESCRIPTION_HEADER: &str = "Description";

#[derive(Args)]
#[command(after_help = r#"Filter behavior:
  Repeat one option to match any value in that category (OR).
  Combine ATT&CK and Sigma options to require both categories (AND).
  Omit all filters to list every trace."#)]
pub struct List {
    #[clap(long = "attack", help = "Match an ATT&CK technique (repeatable)")]
    attack_techniques: Vec<AttackTechnique>,

    #[clap(long = "sigma", help = "Match a Sigma UUID (repeatable)")]
    sigma_identifiers: Vec<SigmaIdentifier>,
}

impl List {
    pub fn run(&self) -> ExitCode {
        let width = console::Term::stdout().size().1 as usize;
        let filters = TraceFilters {
            attack_techniques: &self.attack_techniques,
            sigma_identifiers: &self.sigma_identifiers,
        };
        let traces = REGISTRY.query(&filters);
        let output = Self::render_filtered(width.max(1), &traces, &filters);

        print!("{output}");

        ExitCode::SUCCESS
    }

    pub fn render_filtered(
        width: usize,
        traces: &[&TraceDefinition],
        filters: &TraceFilters,
    ) -> String {
        if traces.is_empty() && !Self::has_active_filters(filters) {
            return "No traces are available.\n".to_string();
        }

        if !Self::has_active_filters(filters) {
            return Self::render(width, traces);
        }

        let context_line = Self::render_filter_context(filters);
        if traces.is_empty() {
            return format!("{context_line}\n{NO_MATCH_MESSAGE}");
        }

        let table = Self::render(width, traces);

        let mut output = String::new();
        let _ = write!(output, "{context_line}\n{table}");
        output
    }

    pub fn render(width: usize, traces: &[&TraceDefinition]) -> String {
        if traces.is_empty() {
            return "No traces are available.\n".to_string();
        }

        let width = width.max(1);
        let identifier_width = traces
            .iter()
            .map(|definition| definition.metadata.identifier.as_str().len())
            .chain([IDENTIFIER_HEADER.len()])
            .max()
            .unwrap_or(1);
        let name_width = traces
            .iter()
            .map(|definition| definition.metadata.name.len())
            .chain([NAME_HEADER.len()])
            .max()
            .unwrap_or(1);
        let description_width = traces
            .iter()
            .map(|definition| definition.metadata.description.len())
            .chain([DESCRIPTION_HEADER.len()])
            .max()
            .unwrap_or(1);

        let three_column_width =
            identifier_width + SEPARATOR.len() + name_width + SEPARATOR.len() + description_width;
        if three_column_width <= width {
            return Self::render_three_columns(
                traces,
                identifier_width,
                name_width,
                description_width,
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
        traces: &[&TraceDefinition],
        identifier_width: usize,
        name_width: usize,
        description_width: usize,
    ) -> String {
        let mut output = String::new();
        Self::render_row(
            &mut output,
            [IDENTIFIER_HEADER, NAME_HEADER, DESCRIPTION_HEADER],
            [identifier_width, name_width, description_width],
        );

        for definition in traces {
            let identifier = definition.metadata.identifier.as_str();
            Self::render_row(
                &mut output,
                [
                    identifier.as_str(),
                    definition.metadata.name,
                    definition.metadata.description,
                ],
                [identifier_width, name_width, description_width],
            );
        }

        output
    }

    fn render_two_columns(
        traces: &[&TraceDefinition],
        identifier_width: usize,
        name_width: usize,
    ) -> String {
        let mut output = String::new();
        Self::render_row(
            &mut output,
            [IDENTIFIER_HEADER, NAME_HEADER],
            [identifier_width, name_width],
        );

        for definition in traces {
            let identifier = definition.metadata.identifier.as_str();
            Self::render_row(
                &mut output,
                [identifier.as_str(), definition.metadata.name],
                [identifier_width, name_width],
            );
        }

        output
    }

    fn render_identifier_column(traces: &[&TraceDefinition], width: usize) -> String {
        let mut output = String::new();
        let identifier_width = width.max(1);
        Self::render_row(&mut output, [IDENTIFIER_HEADER], [identifier_width]);

        for definition in traces {
            let identifier = definition.metadata.identifier.as_str();
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

    fn has_active_filters(filters: &TraceFilters) -> bool {
        !filters.attack_techniques.is_empty() || !filters.sigma_identifiers.is_empty()
    }

    fn render_filter_context(filters: &TraceFilters) -> String {
        let mut parts: Vec<String> = Vec::new();

        if !filters.attack_techniques.is_empty() {
            let joined = filters
                .attack_techniques
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("attack={joined}"));
        }

        if !filters.sigma_identifiers.is_empty() {
            let joined = filters
                .sigma_identifiers
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("sigma={joined}"));
        }

        format!("filters: {}", parts.join(" "))
    }
}
