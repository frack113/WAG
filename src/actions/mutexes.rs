// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::actions::{mutexes::create::Create, Runnable};
use clap::{Args, Subcommand};
use std::error::Error;

pub mod create;

#[derive(Debug, Args)]
pub struct Mutexes {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Create(Create),
}

impl Runnable for Mutexes {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            Commands::Create(create) => create as &dyn Runnable,
        }
        .run()
    }
}
