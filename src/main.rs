/*
 ____ ____ ____ ____ ____ ____ ____ _________ ____ ____ ____ ____ ____ ____ ____ ____ _________ ____ ____ ____ ____ ____ ____ ____ ____ ____
||W |||i |||n |||d |||o |||w |||s |||       |||A |||r |||t |||e |||f |||a |||c |||t |||       |||G |||e |||n |||e |||r |||a |||t |||o |||r ||
||__|||__|||__|||__|||__|||__|||__|||_______|||__|||__|||__|||__|||__|||__|||__|||__|||_______|||__|||__|||__|||__|||__|||__|||__|||__|||__||
|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/_______\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/_______\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|

Working date:  2023-08
*/

use wag::artefact::file::{run_ads, run_createfile};
use wag::artefact::namepipe::run_pipecreate;
use wag::artefact::service::run_byovd;

use wag::cli;

use clap::Parser;


fn banner() {
    let banner = "

    ██     ██  █████   ██████  
    ██     ██ ██   ██ ██       
    ██  █  ██ ███████ ██   ███ 
    ██ ███ ██ ██   ██ ██    ██ 
     ███ ███  ██   ██  ██████ 
    
    ";
    println!("{}", banner);
}


fn main() -> ! {
    banner();
    let my_cli: cli::WagCli = cli::WagCli::parse();

    match my_cli.command {
        cli::Clioptions::ADS {
            module,
            get,
            filename,
        } => {
            let ret: i32 = run_ads(module, get, filename);
            std::process::exit(ret);
        }

        cli::Clioptions::BYOVD {
            internal,
            display,
            path,
        } => {
            let ret: i32 = run_byovd(internal, display, path);
            std::process::exit(ret);
        }

        cli::Clioptions::FileCreate {
            module,
            get,
            filename,
            magicbyte,
            details,
        } => {
            let ret: i32 = run_createfile(module, get, filename, magicbyte, details);
            std::process::exit(ret);
        }

        cli::Clioptions::NamePipe {
            module,
            number,
            get,
            details,
            name,
        } => {
            let ret: i32 = run_pipecreate(module, number, get, details, name);
            std::process::exit(ret);
        }
    }
}
