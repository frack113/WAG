// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 the cli option

 Global
 --help -h
 --module -m
 --get -g
 --detail -d

*/

use crate::malware::ads::ADS;
use crate::malware::file::FileCreate;
use crate::malware::mutex::Mutex;
use crate::malware::namepipe::NamePipe;
use crate::malware::ppid::PPID;
use crate::malware::service::BYOVD;
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
    ADS(ADS),
    #[clap(arg_required_else_help = true)]
    NamePipe(NamePipe),
    #[clap(arg_required_else_help = true)]
    Mutex(Mutex),
    #[clap(arg_required_else_help = true)]
    BYOVD(BYOVD),
    #[clap(arg_required_else_help = true)]
    PPID(PPID),
}
