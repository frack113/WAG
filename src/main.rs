// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod actions;
mod cli;
mod windows;

use actions::Runnable;
use clap::Parser;
use cli::{Arguments, Commands};
use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {
    banner();

    Ok(match Arguments::parse().command {
        Commands::Actions(action) => action.run()?,
    })
}
