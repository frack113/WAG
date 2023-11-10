/*
 the cli option

 Global
 --help -h
 --module -m
 --get -g
 --detail -d

*/

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
        #[clap(
            short = 'f',
            long,
            required = false,
            default_value = "",
            help = "Full path filename (regex)"
        )]
        filename: String,
        #[clap(
            short = 'm',
            long,
            required = false,
            default_value = "",
            help = "ADS to use"
        )]
        module: String,
        #[clap(
            short = 'g',
            long,
            required = false,
            default_value_t = false,
            help = "Get all the possible ADS name and quit"
        )]
        get: bool,
    },

    /// Bring Your Own Vulnerable Driver
    #[clap(arg_required_else_help = true)]
    BYOVD {
        #[clap(short = 'n', long, help = "Internal Name of the service")]
        internal: String,
        #[clap(short = 'd', long, help = "Displayed Name of the service")]
        display: String,
        #[clap(short = 'p', long, help = "Full path to the driver eg: c:\\temp...")]
        path: String,
    },

    /// Create dummy file Artefact
    #[clap(arg_required_else_help = true)]
    FileCreate {
        #[clap(
            short = 'm',
            long,
            required = false,
            default_value = "",
            help = "Name of the malware to mimic"
        )]
        module: String,
        #[clap(
            short = 'g',
            long,
            required = false,
            default_value_t = false,
            help = "Get all the possible mimic name and quit"
        )]
        get: bool,
        #[clap(
            short = 'f',
            long,
            required = false,
            default_value = "",
            help = "Full path filename (regex) with module manual"
        )]
        filename: String,
        #[clap(
            short = 'b',
            long,
            required = false,
            default_value = "",
            help = "MagicBytes name to use with module manual "
        )]
        magicbyte: String,
        #[clap(
            short = 'd',
            long,
            required = false,
            default_value_t = false,
            help = "Get all the possible MagicBytes name with module manual"
        )]
        details: bool,
    },

    /// Generates Name Pipe Artefact
    #[clap(arg_required_else_help = true)]
    NamePipe {
        #[clap(
            short = 'm',
            long,
            required = false,
            default_value = "",
            help = "Name of the malware to mimic"
        )]
        module: String,
        #[clap(short = 'n', long, required = false, default_value_t = 0)]
        number: usize,
        #[clap(
            short = 'g',
            long,
            required = false,
            default_value_t = false,
            help = "Get all the possible pipename for a mimic and quit"
        )]
        get: bool,
        #[clap(
            short = 'd',
            long,
            required = false,
            default_value_t = false,
            help = "Get all the possible mimic name"
        )]
        details: bool,
        #[clap(
            short = 'N',
            long,
            required = false,
            default_value = "",
            help = "Regex of the PipeName to Create"
        )]
        name: String,
    },
}
