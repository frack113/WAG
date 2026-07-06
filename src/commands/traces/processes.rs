// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod spoofing;

use crate::commands::traces::{Trace, processes::spoofing::Spoofing};
use clap::{Args, Subcommand};
use serde::Deserialize;
use std::process::ExitCode;

#[derive(Args, Deserialize)]
pub struct Processes {
    #[clap(subcommand)]
    #[serde(flatten)]
    pub command: Commands,
}

#[derive(Subcommand, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Commands {
    Spoofing(Spoofing),
}

impl Processes {
    pub fn run(&self) -> ExitCode {
        match &self.command {
            Commands::Spoofing(spoofing) => Trace::run(spoofing),
        }
    }
}
