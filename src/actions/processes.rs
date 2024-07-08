// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::actions::{processes::spoofing::Spoofing, Runnable};
use clap::{Args, Subcommand};

pub mod spoofing;

#[derive(Debug, Args)]
pub struct Processes {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Spoofing(Spoofing),
}

impl Runnable for Processes {
    fn run(&self) -> i32 {
        return match &self.command {
            Commands::Spoofing(spoofing) => spoofing as &dyn Runnable,
        }
        .run();
    }
}
