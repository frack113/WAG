// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod byovd;

use crate::commands::traces::{Trace, drivers::byovd::Byovd};
use clap::{Args, Subcommand};
use serde::Deserialize;
use std::process::ExitCode;

#[derive(Args, Deserialize)]
pub struct Drivers {
    #[clap(subcommand)]
    #[serde(flatten)]
    pub command: Commands,
}

#[derive(Subcommand, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Commands {
    Byovd(Byovd),
}

impl Drivers {
    pub fn run(&self) -> ExitCode {
        match &self.command {
            Commands::Byovd(byovd) => Trace::run(byovd),
        }
    }
}
