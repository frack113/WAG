// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::actions::{drivers::create::Create, Runnable};
use clap::{Args, Subcommand};

pub mod create;

#[derive(Debug, Args)]
pub struct Drivers {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Create(Create),
}

impl Runnable for Drivers {
    fn run(&self) -> i32 {
        return match &self.command {
            Commands::Create(create) => create as &dyn Runnable,
        }
        .run();
    }
}
