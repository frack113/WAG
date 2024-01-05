/*
 the cli option

 Global
 --help -h
 --module -m
 --get -g
 --detail -d

*/

use crate::artefact::file::FileCreate;
use crate::artefact::file::ADS;
use crate::artefact::namepipe::NamePipe;
use crate::artefact::service::BYOVD;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
#[clap(disable_version_flag = true)]
pub struct Arguments {
    #[clap(short = 'v', long)]
    version: bool,
    #[clap(subcommand)]
    command: Option<Commands>,
}

impl Arguments {
    pub fn run(self) -> i32 {
        match self.command {
            Some(Commands::FileCreate(file_create)) => file_create.run(),
            Some(Commands::ADS(ads)) => ads.run(),
            Some(Commands::NamePipe(name_pipe)) => name_pipe.run(),
            Some(Commands::BYOVD(byovd)) => byovd.run(),
            None => {
                return 2;
            }
        }
    }
}

#[derive(Parser)]
pub enum Commands {
    #[clap(arg_required_else_help = true)]
    FileCreate(FileCreate),
    #[clap(arg_required_else_help = true)]
    ADS(ADS),
    #[clap(arg_required_else_help = true)]
    NamePipe(NamePipe),
    #[clap(arg_required_else_help = true)]
    BYOVD(BYOVD),
}
