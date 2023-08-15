use super::*;

#[derive(Debug, Parser)]
#[clap(
    name = "WAG",
    about = "WAG is a CLI Application to genereate Windows Artefacts",
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
        #[clap(short = 'n', long,help="Name of the malware to mimic")]
        name: String,
        #[clap(short = 't', long, required = false, default_value_t = 0)]
        number: usize,
        #[clap(short = 'l', long, required= false,help="Get all the possible number")]
        list: bool,
    },
    /// List all the Value
    #[clap(arg_required_else_help = true)]
    List {
        #[clap(short = 'm', long)]
        module: String,
        #[clap(short = 'd', long, required= false)]
        details: bool,
    },

}