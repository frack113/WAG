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

use crate::commands::tools::{
    hex_to_bytes, process_is_admin, regex_to_string, EXIST_ALL_GOOD, EXIST_TEST_ERROR,
};
use clap::Parser;
use std::env;
use std::ffi::OsString;
use std::io::Result as IOResult;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{thread, time};

#[derive(Parser)]
pub struct FileCreate {
    #[clap(
        short = 'f',
        long,
        required = false,
        default_value = "",
        help = "Full path filename (regex)"
    )]
    filename: String,
    #[clap(
        short = 'v',
        long,
        required = false,
        default_value = "",
        help = "Use the CMD variable"
    )]
    cmd_var: String,
    #[clap(
        short = 'p',
        long,
        required = false,
        default_value = "",
        help = "Use the CMD path"
    )]
    cmd_path: String,
    #[clap(
        short = 'm',
        long,
        required = true,
        default_value = "",
        help = "MagicBytes name to use with module manual "
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
        if self.admin && !process_is_admin() {
            println!("Need to have Administrator right to create the file");
            return EXIST_TEST_ERROR;
        }

        let header: Option<Vec<u8>> = hex_to_bytes(&self.magicbyte);
        let payload: Vec<u8> = match header {
            Some(data) => data,
            None => vec![70, 114, 97, 99, 107, 49, 49, 51],
        };

        let fullname: String;
        if self.filename.len() > 0 {
            fullname = regex_to_string(&self.filename);
        } else {
            let filename: String = regex_to_string(&self.cmd_path);
            let var_path: OsString = env::var_os(&self.cmd_var).unwrap();
            let full_path: PathBuf = Path::new(&var_path).join(filename);
            fullname = String::from(full_path.to_string_lossy());
        }

        println!("Create a file on disk");

        let ret: bool = create_file(fullname, payload);

        if ret == true {
            return EXIST_ALL_GOOD;
        } else {
            return EXIST_TEST_ERROR;
        }
    }
}
