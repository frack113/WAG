// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::actions::{
    ads::create::Create as ADSCreate, drivers::create::Create as DriversCreate,
    files::create::Create as FileCreate, mutexes::create::Create as MutexCreate,
    pipes::create::Create as PipesCreate, processes::spoofing::Spoofing as ProcessesSpoofing,
};
use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
#[clap(arg_required_else_help = true)]
pub struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
}

impl Arguments {
    pub fn run(self) -> i32 {
        match self.command {
            Some(Commands::FileCreate(file_create)) => file_create.run(),
            Some(Commands::ADS(ads)) => ads.run(),
            Some(Commands::NamePipe(name_pipe)) => name_pipe.run(),
            Some(Commands::Mutex(mutex)) => mutex.run(),
            Some(Commands::BYOVD(byovd)) => byovd.run(),
            Some(Commands::PPID(ppid)) => ppid.run(),
            None => {
                return 2;
            }
        }
    }
}

#[derive(Parser)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    FileCreate(FileCreate),
    #[clap(arg_required_else_help = true)]
    ADS(ADSCreate),
    #[clap(arg_required_else_help = true)]
    NamePipe(PipesCreate),
    #[clap(arg_required_else_help = true)]
    Mutex(MutexCreate),
    #[clap(arg_required_else_help = true)]
    BYOVD(DriversCreate),
    #[clap(arg_required_else_help = true)]
    PPID(ProcessesSpoofing),
}
