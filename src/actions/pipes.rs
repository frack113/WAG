// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::actions::{pipes::create::Create, Runnable};
use clap::{Args, Subcommand};
use std::error::Error;

pub mod create;

#[derive(Debug, Args)]
pub struct Pipes {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Create(Create),
}

impl Runnable for Pipes {
    fn run(&self) -> Result<i32, Box<dyn Error>> {
        return match &self.command {
            Commands::Create(create) => create as &dyn Runnable,
        }
        .run();
    }
}
