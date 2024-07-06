// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::malware::{
    ads::ADS, dropper::Dropper, mutex::Mutex, namepipe::NamePipe, ppid::PPID, service::BYOVD,
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
            Some(Commands::Dropper(dropper)) => dropper.run(),
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
    Dropper(Dropper),
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
