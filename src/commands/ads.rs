use super::file::FileArtefact;
use crate::commands::tools::{
    pretty_print_hashset, regex_to_string, EXIST_ALL_GOOD, EXIST_CLI_ERROR, EXIST_TEST_ERROR,
};
use clap::Parser;
use std::collections::HashSet;
use windows::core::Result as WindowsResult;
use windows::core::PCSTR;
use windows::Win32::Foundation::{CloseHandle, GENERIC_WRITE, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileA, WriteFile, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_WRITE,
};

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
    let file_path: String = format!("{}:{}\0", fullpath, adsname);
    println!("ads: {}", file_path);

    let handle: HANDLE = unsafe {
        CreateFileA(
            PCSTR::from_raw(file_path.as_ptr()),
            GENERIC_WRITE.0,
            FILE_SHARE_WRITE,
            None,
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE::default(),
        )
    }
    .unwrap();

    let result: WindowsResult<()> = unsafe {
        WriteFile(
            handle,
            Some(hex_data.as_slice()),
            Some(hex_data.len() as *mut u32),
            None,
        )
    };

    let _: WindowsResult<()> = unsafe { CloseHandle(handle) };

    match result {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
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
