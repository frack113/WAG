// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod actions;
mod cli;
mod windows;

use actions::Runnable;
use clap::Parser;
use cli::{Arguments, Commands};

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

fn main() {
    banner();

    match Arguments::parse().command {
        Commands::Actions(action) => match action.run() {
            Ok(code) => std::process::exit(code),
            Err(error) => {
                println!("Error: {}", error);

                std::process::exit(1);
            }
        },
    };
}
