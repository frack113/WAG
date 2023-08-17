use super::*;

#[derive(Debug, Parser)]
#[clap(
    name = "WAG",
    about = "WAG is a CLI Application to generate Windows Artefacts",
    version = "0.0.1"
)]
pub struct WagCli {
    #[clap(subcommand)]
    pub command: Clioptions,
}

#[derive(Debug, Subcommand)]
pub enum Clioptions {
    /// Generates Name Pipe Artefact
    #[clap(arg_required_else_help = true)]
    NamePipe {
        #[clap(short = 'n', long, help="Name of the malware to mimic")]
        name: String,
        #[clap(short = 't', long, required = false, default_value_t = 0)]
        number: usize,
        #[clap(short = 'l', long, required= false, default_value_t = false,help="Get all the possible number")]
        list: bool,
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

}