// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod dll;

use crate::commands::traces::{Trace, memory::dll::DllLoader};
use clap::{Args, Subcommand};
use serde::Deserialize;
use std::process::ExitCode;

#[derive(Args, Deserialize)]
pub struct Memory {
    #[clap(subcommand)]
    #[serde(flatten)]
    pub command: Commands,
}

#[derive(Subcommand, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Commands {
    DllLoader(DllLoader),
}

impl Memory {
    pub fn run(&self) -> ExitCode {
        match &self.command {
            Commands::DllLoader(dll_loader) => Trace::run(dll_loader),
        }
    }
}
