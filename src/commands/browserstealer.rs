// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

//
// Mimic stealer action on file
//
// Last update 20240609

use std::env;
use std::fs;
use walkdir::WalkDir;

// Some others
use crate::commands::tools::EXIST_ALL_GOOD;
use clap::ArgAction;
use clap::Parser;

#[derive(Parser)]
pub struct BrowserStealer {
    #[clap(short = 'c', long, help = "Compress file into the default temp", action=ArgAction::SetFalse,required = false)]
    compress: bool,
}

fn steal_file(name: walkdir::DirEntry, temp: &str) {
    let infile: String = name.path().display().to_string();
    let outfile: String =
        temp.to_owned() + &String::from('\\') + name.file_name().to_str().unwrap();
    fs::copy(infile, outfile).unwrap();
}

impl BrowserStealer {
    /* Version 202406xx */
    pub fn run(&self) -> i32 {
        let sensitive_file = ["key4.db", "cookies.sqlite"];
        println!("Mimic stealer file access ");
        if self.compress {
            println!("No compress for now :)");
        }

        let userprofile = env::var("USERPROFILE").unwrap();
        println!("😈 looking in the folder {}", userprofile);

        let tempfolder = env::var("TEMP").unwrap();

        for entry in WalkDir::new(userprofile).into_iter().filter_map(|e| e.ok()) {
            let filename: &str = entry.file_name().to_str().unwrap();
            if sensitive_file.contains(&&filename) {
                println!("😈 stealing the file {}", filename);
                steal_file(entry, &tempfolder);
            }
        }

        return EXIST_ALL_GOOD;
    }
}
