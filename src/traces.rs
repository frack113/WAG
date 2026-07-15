// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod browser;
pub mod byovd;
pub mod dll;
pub mod spoofing;

use crate::metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier};
use clap::Parser;
use std::{error::Error, ffi::OsString};
use toml_span::{DeserError, DeserializeOwned, Value};

pub struct TraceDefinition {
    pub metadata: TraceMetadata,
    pub(crate) from_toml: TomlTraceConstructor,
    #[allow(dead_code)]
    pub(crate) from_cli: CliTraceConstructor,
}

impl TraceDefinition {
    pub fn new<Input>(metadata: TraceMetadata, execute: fn(Input) -> Result<(), TraceError>) -> Self
    where
        Input: DeserializeOwned + Parser + 'static,
    {
        Self {
            metadata,
            from_toml: Box::new(move |value| {
                let input = Input::deserialize(value)?;

                Ok(Box::new(move || execute(input)))
            }),
            from_cli: Box::new(move |arguments| {
                let input = Input::try_parse_from(arguments)?;

                Ok(Box::new(move || execute(input)))
            }),
        }
    }
}

pub struct TraceMetadata {
    pub identifier: TraceIdentifier,
    pub name: &'static str,
    pub description: &'static str,
    pub attack_techniques: Vec<AttackTechnique>,
    pub sigma_identifiers: Vec<SigmaIdentifier>,
}

pub type Trace = Box<dyn FnOnce() -> Result<(), TraceError>>;
pub type TraceError = Box<dyn Error>;
pub(crate) type TomlTraceConstructor =
    Box<dyn Fn(&mut Value<'static>) -> Result<Trace, DeserError> + Send + Sync>;
pub(crate) type CliTraceConstructor =
    Box<dyn Fn(&[OsString]) -> Result<Trace, clap::Error> + Send + Sync>;
