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
    /// Alternate Data Stream
    #[clap(arg_required_else_help = true)]
    ADS {
        #[clap(short = 'f', long, required = false,default_value ="", help="Full path filename (regex)")]
        filename: String,
        #[clap(short = 'a', long, required = false,default_value ="", help="ADS to use")]
        adsname: String, 
        #[clap(short = 'l', long, required = false, default_value_t = false,help="Get all the possible ADS name and quit")]
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

    /// Create dummy file Artefact
    #[clap(arg_required_else_help = true)]
    FileCreateAuto {
        #[clap(short = 'n', long, required = false,default_value ="", help="Name of the malware to mimic")]
        name: String,
        #[clap(short = 'm', long, required = false, default_value_t = false,help="Get all the possible mimic name and quit")]
        mimic: bool,
    },
    /// Create dummy file Artefact manualy
    #[clap(arg_required_else_help = true)]
    FileCreateManual {
        #[clap(short = 'f', long, required = false,default_value ="", help="Full path filename (regex)")]
        filename: String,
        #[clap(short = 'm', long, required = false,default_value ="", help="MagicBytes name to use")]
        magicbyte: String,       
        #[clap(short = 'l', long, required = false, default_value_t = false,help="Get all the possible MagicBytes name and quit")]
        list: bool,
    },

    /// Generates Name Pipe Artefact
    #[clap(arg_required_else_help = true)]
    NamePipeAuto {
        #[clap(short = 'n', long, required = false,default_value ="", help="Name of the malware to mimic")]
        name: String,
        #[clap(short = 't', long, required = false, default_value_t = 0)]
        number: usize,
        #[clap(short = 'p', long, required = false, default_value_t = false,help="Get all the possible pipename for a mimic and quit")]
        pipe: bool,
        #[clap(short = 'm', long, required = false, default_value_t = false,help="Get all the possible mimic name and quit")]
        mimic: bool,
    },
    /// Generates Name Pipe Artefact manualy
    #[clap(arg_required_else_help = true)]
    NamePipeManual {
        #[clap(short = 'p', long, required = true,default_value ="", help="Regex of the PipeName to Create")]
        name: String,
    },
}