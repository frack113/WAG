// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod list;

mod drivers;
mod files;
mod memory;
mod processes;

use crate::commands::traces::{
    drivers::Drivers, files::Files, list::List, memory::Memory, processes::Processes,
};
use clap::{Args, Subcommand};
use serde::Deserialize;
use std::process::ExitCode;

#[derive(Args, Deserialize)]
pub struct Traces {
    #[clap(subcommand)]
    #[serde(flatten)]
    pub command: Commands,
}

#[derive(Subcommand, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Commands {
    Drivers(Drivers),
    Memory(Memory),
    Processes(Processes),
    Files(Files),
}

pub trait Trace {
    fn run(&self) -> ExitCode;
}

pub struct TraceMetadata {
    pub identifier: &'static str,
    pub name: &'static str,
    pub requirements_summary: &'static str,
    pub attack_techniques: &'static [&'static str],
    pub sigma_targets: &'static [&'static str],
    pub use_cases: &'static [&'static str],
}

pub const TRACE_METADATA: &[&TraceMetadata] = &[
    &drivers::byovd::METADATA,
    &memory::dll::METADATA,
    &processes::spoofing::METADATA,
    &files::browser::METADATA,
];

impl Traces {
    pub fn run(&self) -> ExitCode {
        match &self.command {
            Commands::Drivers(drivers) => drivers.run(),
            Commands::Memory(memory) => memory.run(),
            Commands::Processes(processes) => processes.run(),
            Commands::Files(files) => files.run(),
        }
    }
}
