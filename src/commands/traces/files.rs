// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod browser;

use crate::commands::traces::{Trace, files::browser::Browser};
use clap::{Args, Subcommand};
use serde::Deserialize;
use std::process::ExitCode;

#[derive(Args, Deserialize)]
pub struct Files {
    #[clap(subcommand)]
    #[serde(flatten)]
    pub command: Commands,
}

#[derive(Subcommand, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Commands {
    Browser(Browser),
}

impl Files {
    pub fn run(&self) -> ExitCode {
        match &self.command {
            Commands::Browser(browser) => Trace::run(browser),
        }
    }
}
