use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "WAG",
    about = "WAG is a CLI Application to generate Windows Artefacts",
    version = "0.0.1"
)]
pub struct WagCli {
    #[clap(subcommand)]
    pub command: Clioptions,
}

#[derive(Subcommand)]
pub enum Clioptions {
    /// Generates Name Pipe Artefact
    #[clap(arg_required_else_help = true)]
    NamePipe {
        #[clap(short = 'n', long, required = false,default_value ="help", help="Name of the malware to mimic")]
        name: String,
        #[clap(short = 't', long, required = false, default_value_t = 0)]
        number: usize,
        #[clap(short = 'p', long, required = false, default_value_t = false,help="Get all the possible pipename for a mimic and quit")]
        pipe: bool,
        #[clap(short = 'm', long, required = false, default_value_t = false,help="Get all the possible mimic name and quit")]
        mimic: bool,
    },
    /// Bring Your Own Vulnerable Driver
    #[clap(arg_required_else_help = true)]
    BYOVD {
        #[clap(short = 'n', long, help="Internal Name of the service")]
        name: String,
        #[clap(short = 'd', long, help="Displayed Name of the service")]
        details: String,
        #[clap(short = 'p', long, help="Full path to the driver eg: c:\\temp...")]
        path: String,
    },
    /// Create dummy file
    #[clap(arg_required_else_help = true)]
    Filecreate {
        #[clap(short = 'n', long, required = false,default_value ="help", help="Name of the malware to mimic")]
        name: String,
        #[clap(short = 'm', long, required = false, default_value_t = false,help="Get all the possible mimic name and quit")]
        mimic: bool,
    },
}