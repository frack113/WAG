// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod traces;

use crate::commands::traces::Traces;
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[clap(author, version)]
#[clap(arg_required_else_help = true)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Traces(Traces),
}

impl Arguments {
    pub fn run(&self) -> ExitCode {
        self.command.run()
    }
}

impl Commands {
    fn run(&self) -> ExitCode {
        match self {
            Commands::Traces(traces) => traces.run(),
        }
    }
}
