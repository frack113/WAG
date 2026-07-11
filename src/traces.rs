// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod browser;
pub mod byovd;
pub mod dll;
pub mod spoofing;

use crate::{
    metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier},
    traces::{browser, byovd, dll, spoofing},
};
use std::{process::ExitCode, sync::LazyLock};

pub trait Trace {
    fn run(&self) -> ExitCode;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceParameterKind {
    Path,
    String,
    Enum,
}

#[derive(Debug, Clone, Copy)]
pub struct TraceParameter {
    pub name: &'static str,
    pub kind: TraceParameterKind,
    pub required: bool,
    pub allowed_values: &'static [&'static str],
}

pub struct TraceMetadata {
    pub identifier: TraceIdentifier,
    pub name: &'static str,
    pub requirements_summary: &'static str,
    pub attack_techniques: Vec<AttackTechnique>,
    pub sigma_identifiers: Vec<SigmaIdentifier>,
    pub use_cases: Vec<String>,
    pub parameters: &'static [TraceParameter],
}

pub static TRACE_METADATA: LazyLock<Vec<&'static TraceMetadata>> = LazyLock::new(|| {
    vec![
        &byovd::METADATA,
        &dll::METADATA,
        &spoofing::METADATA,
        &browser::METADATA,
    ]
});

pub fn lookup(identifier: &str) -> Option<&'static TraceMetadata> {
    TRACE_METADATA
        .iter()
        .find(|metadata| metadata.identifier.as_str() == identifier)
        .copied()
}
