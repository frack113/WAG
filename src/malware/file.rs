// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

/*
        File creation Artefact

the magic bytes and NTFS ADS are store a string HEX value.
like 504B0304 or 4D5A.

Path are regex in json so need `\\.` to get a `.`.

if fullpath is empty , it build the path from
env variable and the cmd_path.
"SystemRoot" "Temp\\debug\\.bin" will give "c:\Windows\Temp\debug.bin"

You can use `SET | more` or `Get-ChildItem Env:` to get the list

*/

use crate::windows::users::is_administrator;
use base64::engine::{general_purpose, Engine};
use clap::Parser;
use regex_generate::{Generator, DEFAULT_MAX_REPEAT};
use std::{io::Result as IOResult, path::Path, thread, time, time::Duration};

#[derive(Parser)]
pub struct FileCreate {
    #[clap(
        short = 'f',
        long,
        required = true,
        help = "Full path filename (regex)"
    )]
    filename: String,
    #[clap(
        short = 'm',
        long,
        required = false,
        default_value = "V2VsY29tZSB0byB0aGUgV0FH",
        help = "MagicBytes name to use with module manual in base64"
    )]
    magicbyte: String,
    #[clap(
        short = 'a',
        long,
        required = false,
        default_value_t = false,
        help = "Need to be admin"
    )]
    admin: bool,
}

fn create_file(fullpath: String, hex_data: Vec<u8>) -> bool {
    println!("Try to create : {}", fullpath);
    let file_path: &Path = Path::new(&fullpath);
    if !file_path.exists() {
        let folder: &Path = file_path.parent().unwrap();

        let ret_folder: IOResult<()> = std::fs::create_dir_all(folder);
        match ret_folder {
            Ok(_) => println!("The folder is valid"),
            Err(_) => return false,
        }

        let ret_file: IOResult<()> = std::fs::write(file_path, hex_data);
        match ret_file {
            Ok(_) => println!("The file is created"),
            Err(_) => return false,
        }

        let sleep_duration: Duration = time::Duration::from_millis(2000);
        thread::sleep(sleep_duration);

        let ret_remove: IOResult<()> = std::fs::remove_file(file_path);
        match ret_remove {
            Ok(_) => println!("The file is removed"),
            Err(_) => return false,
        }

        return true;
    }
    return false;
}

impl FileCreate {
    pub fn run(&self) -> i32 {
        if self.admin
            && !match is_administrator() {
                Ok(is_admin) => is_admin,
                Err(error) => {
                    println!(
                        "Could not check if the user is an administrator or not.\nError: {}",
                        error
                    );

                    return 1;
                }
            }
        {
            println!("Need to have Administrator right to create the file");
            return 1;
        }

        let mut generator: Generator<rand::rngs::ThreadRng> =
            match Generator::new(&self.filename, rand::thread_rng(), DEFAULT_MAX_REPEAT) {
                Ok(generator) => generator,
                Err(_) => {
                    println!("Regex expressions are malformed.");

                    return 1;
                }
            };
        let mut buffer: Vec<u8> = vec![];
        generator.generate(&mut buffer).unwrap();
        let fullname: String = match String::from_utf8(buffer) {
            Ok(string) => string,
            Err(_) => {
                println!("Filename contains non-utf8 characters.");

                return 1;
            }
        };

        println!("Create a file on disk");

        let payload: Vec<u8> = match general_purpose::STANDARD.decode(self.magicbyte.as_str()) {
            Ok(decoded) => decoded,
            Err(_) => {
                println!("Could not decode the data.");

                return 1;
            }
        };
        let ret: bool = create_file(fullname, payload);

        if ret == true {
            return 0;
        } else {
            return 1;
        }
    }
}
