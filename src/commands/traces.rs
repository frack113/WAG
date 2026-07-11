// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod list;

use crate::commands::traces::list::List;
use clap::{Args, Subcommand};
use std::process::ExitCode;

#[derive(Args)]
pub struct Traces {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List(List),
}

impl Traces {
    pub fn run(&self) -> ExitCode {
        match &self.command {
            Commands::List(list) => list.run(),
        }
    }
}
