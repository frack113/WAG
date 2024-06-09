// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

//
// Mimic stealer action on file
//
// Last update 20240609

use std::env;
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;

// Some others
use crate::commands::tools::EXIST_ALL_GOOD;
use clap::Parser;

#[derive(Parser)]
pub struct BrowserStealer {
    #[clap(short = 'b', long, help = "Browser to steal")]
    browser: String,
}

/// read the file like a stealer but do not process the data
fn steal_file(name: walkdir::DirEntry) {
    let infile: String = name.path().display().to_string();
    let mut file: File = File::open(infile).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
}

impl BrowserStealer {
    /* Version 20240609 */
    pub fn run(&self) -> i32 {
        let sensitive_files: Vec<&str> = match self.browser.as_str() {
            "chrome" => vec!["Login Data", "Cookies", "History"],
            "edge" => vec!["Login Data", "Cookies", "History"],
            "firefox" => vec![
                "key3.db",
                "key4.db",
                "logins.json",
                "cert9.db",
                "compatibility.ini",
            ],
            _ => vec!["password.txt"],
        };

        let brower_data = match self.browser.as_str() {
            "chrome" => "\\AppData\\Local\\Google\\Chrome\\User Data",
            "edge" => "\\AppData\\Local\\Microsoft\\Edge",
            "firefox" => "\\AppData\\Roaming\\Mozilla\\Firefox",
            _ => "\\AppData",
        };

        println!("Mimic stealer file access ");

        let userprofile: String = env::var("USERPROFILE").unwrap();
        println!("😈 looking in the user folder : {}", userprofile);

        let data_folder: String = userprofile + brower_data;
        for entry in WalkDir::new(data_folder).into_iter().filter_map(|e| e.ok()) {
            let filename: &str = entry.file_name().to_str().unwrap();
            if sensitive_files.contains(&&filename) {
                println!("🥷 stealing the data from {}", entry.path().display());
                steal_file(entry);
            }
        }

        return EXIST_ALL_GOOD;
    }
}
