// SPDX-FileCopyrightText: 2023 The WAG team
//
// SPDX-License-Identifier: GPL-3.0-or-later

//
// Alternate Data Stream
//
// Last update 20240224

use crate::commands::tools::{
    hex_to_bytes, regex_to_string, EXIST_ALL_GOOD, EXIST_CLI_ERROR, EXIST_TEST_ERROR,
};

use clap::Parser;
use std::path::Path;

#[derive(Parser)]
pub struct ADS {
    #[clap(
        short = 'f',
        long,
        required = true,
        default_value = "",
        help = "Full path filename (regex)"
    )]
    filename: String,
    #[clap(
        short = 'a',
        long,
        required = true,
        default_value = "",
        help = "ADS to use"
    )]
    ads: String,
    #[clap(
        short = 'd',
        long,
        required = true,
        default_value = "",
        help = "Data to write in HEX"
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
    match ret_file {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

impl ADS {
    /* Version 20230908 */
    pub fn run(&self) -> i32 {
        println!("Alternate Data Stream");

        if self.filename.len() > 0 {
            let fullname: String = regex_to_string(&self.filename);

            let header: Option<Vec<u8>> = hex_to_bytes(&self.data);
            let payload: Vec<u8> = match header {
                Some(data) => data,
                None => vec![70, 114, 97, 99, 107, 49, 49, 51],
            };

            let barrow_ads: String = self.ads.to_string();
            let ret_ads: bool = create_ads(fullname, barrow_ads, payload);
            if ret_ads == true {
                return EXIST_ALL_GOOD;
            } else {
                return EXIST_TEST_ERROR;
            }
        }

        EXIST_CLI_ERROR
    }
}
