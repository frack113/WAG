use super::file::FileArtefact;
use crate::commands::tools::{
    pretty_print_hashset, regex_to_string, EXIST_ALL_GOOD, EXIST_CLI_ERROR, EXIST_TEST_ERROR,
};

use clap::Parser;
use std::collections::HashSet;
use std::path::Path;

#[derive(Parser)]
pub struct ADS {
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
        let mut artefact: FileArtefact = FileArtefact::new();
        artefact.load("data/files.json");

        if self.get == true {
            let all_name: HashSet<String> = artefact.file_ads_list();
            pretty_print_hashset("Name for the ADS File data".to_string(), all_name);
            return EXIST_ALL_GOOD;
        }

        if artefact.file_ads_exist(&self.module) == false {
            println!("Did not find \"{}\" name for ads", self.module);
            println!("You can use the help option --help");
            return EXIST_CLI_ERROR;
        }

        if self.filename.len() > 0 {
            println!("Get the regex : {}", self.filename);
            let fullname: String = regex_to_string(&self.filename);
            println!("Create the ADS");
            let name_ads: String = artefact.file_ads_get_name(&self.module);
            let payload: Vec<u8> = artefact.file_ads_get_data(&self.module);
            let ret_ads: bool = create_ads(fullname, name_ads, payload);
            if ret_ads == true {
                return EXIST_ALL_GOOD;
            } else {
                return EXIST_TEST_ERROR;
            }
        }

        EXIST_CLI_ERROR
    }
}
