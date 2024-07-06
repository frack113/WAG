// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{io::Result as IOResult, path::Path};

pub fn create_file(fullpath: &String, hex_data: Vec<u8>) -> bool {
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

        return true;
    }
    return false;
}

pub fn delete_file(fullpath: &String) -> bool {
    println!("Try to delete : {}", fullpath);
    let file_path: &Path = Path::new(&fullpath);
    if file_path.exists() {
        let ret_remove: IOResult<()> = std::fs::remove_file(file_path);
        match ret_remove {
            Ok(_) => println!("The file is removed"),
            Err(_) => return false,
        }
        return true;
    }
    return false;
}
