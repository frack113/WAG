// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::actions::{
    ads::ADS, drivers::Drivers, files::Files, mutexes::Mutexes, pipes::Pipes, processes::Processes,
};
use clap::{Args, Subcommand};
use std::error::Error;

pub mod ads;
pub mod drivers;
pub mod files;
pub mod mutexes;
pub mod pipes;
pub mod processes;

#[derive(Debug, Args)]
pub struct Actions {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    ADS(ADS),
    Drivers(Drivers),
    Files(Files),
    Mutexes(Mutexes),
    Pipes(Pipes),
    Processes(Processes),
}

pub trait Runnable {
    fn run(&self) -> Result<i32, Box<dyn Error>>;
}

impl Runnable for Actions {
    fn run(&self) -> Result<i32, Box<dyn Error>> {
        match &self.command {
            Commands::ADS(ads) => ads as &dyn Runnable,
            Commands::Drivers(drivers) => drivers,
            Commands::Files(files) => files,
            Commands::Mutexes(mutexes) => mutexes,
            Commands::Pipes(pipes) => pipes,
            Commands::Processes(processes) => processes,
        }
        .run()
    }
}
