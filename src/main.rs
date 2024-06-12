// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod cli;
mod malware;

use cli::Arguments;
use clap::Parser;

fn banner() {
    let banner: &str = "

    ██     ██  █████   ██████  
    ██     ██ ██   ██ ██       
    ██  █  ██ ███████ ██   ███ 
    ██ ███ ██ ██   ██ ██    ██ 
     ███ ███  ██   ██  ██████ 
    
    ";
    println!("{}", banner);
}

fn main() -> () {
    banner();

    match Arguments::try_parse() {
        Ok(arguments) => std::process::exit(arguments.run()),
        Err(error) => {
            error.exit();
        }
    }
}
