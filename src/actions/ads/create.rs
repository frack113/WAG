// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Alternate Data Stream
//
// Last update 20240224

use crate::actions::Runnable;
use base64::engine::{general_purpose, Engine};
use clap::Parser;
use regex_generate::{Generator, DEFAULT_MAX_REPEAT};
use std::{error::Error, path::Path};

#[derive(Debug, Parser)]
pub struct Create {
    #[clap(
        short = 'f',
        long,
        required = true,
        help = "Full path filename (regex)"
    )]
    filename: String,
    #[clap(short = 'a', long, required = true, help = "ADS to use")]
    ads: String,
    #[clap(
        short = 'd',
        long,
        required = false,
        default_value = "V2VsY29tZSB0byB0aGUgV0FH",
        help = "Data to write in base64"
    )]
    data: String,
}

fn create_ads(fullpath: String, adsname: String, hex_data: Vec<u8>) -> bool {
    let file_base: &Path = Path::new(&fullpath);
    if !file_base.exists() {
        println!("Missing base file for ADS, try to create it");
        let folder: &Path = file_base.parent().unwrap();

        let ret_folder: Result<(), std::io::Error> = std::fs::create_dir_all(folder);
        match ret_folder {
            Ok(_) => println!("The folder is valid"),
            Err(_) => return false,
        }
        let ret_file: Result<(), std::io::Error> = std::fs::write(
            file_base,
            vec![
                87, 105, 110, 100, 111, 119, 115, 32, 65, 114, 116, 101, 102, 97, 99, 116, 32, 71,
                101, 110, 101, 114, 97, 116, 111, 114,
            ],
        );
        match ret_file {
            Ok(_) => println!("The base file is created"),
            Err(_) => return false,
        }
    }
    let full_ads_name: String = format!("{}:{}", fullpath, adsname);
    let file_ads: &Path = Path::new(&full_ads_name);
    let ret_file: Result<(), std::io::Error> = std::fs::write(file_ads, hex_data);
    ret_file.is_ok()
}

impl Runnable for Create {
    /* Version 20230908 */
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Alternate Data Stream");

        if !self.filename.is_empty() {
            let mut generator: Generator<rand::rngs::ThreadRng> =
                Generator::new(&self.filename, rand::thread_rng(), DEFAULT_MAX_REPEAT)?;
            let mut buffer: Vec<u8> = vec![];
            generator.generate(&mut buffer).unwrap();
            let fullname: String = String::from_utf8(buffer)?;
            let barrow_ads: String = self.ads.to_string();
            let payload: Vec<u8> = general_purpose::STANDARD.decode(self.data.as_str())?;
            let ret_ads: bool = create_ads(fullname, barrow_ads, payload);

            return Ok(());
        }

        Ok(())
    }
}
