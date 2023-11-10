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

// Windows API
use windows::core::PCSTR;
use windows::Win32::Foundation::{CloseHandle, GENERIC_WRITE, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileA, WriteFile, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_WRITE,
};

// Some others
use std::{thread, time};

use serde::Deserialize;

use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::path::{Path, PathBuf};

use super::tools;

/*
    Json Data structure
*/
#[derive(Deserialize)]
struct JsonHeaderInfo {
    name: String,
    magicbyte: String,
}

#[derive(Deserialize)]
struct JsonPayloadInfo {
    name: String,
    needroot: bool,
    file_type: String,
    fullpath: String,
    cmd_var: String,
    cmd_path: String,
}

#[derive(Deserialize)]
struct JsonAdsInfo {
    name: String,
    adsname: String,
    hexvalue: String,
}

#[derive(Deserialize)]
struct JsonGlobalInfo {
    magicbytes: Vec<JsonHeaderInfo>,
    payloads: Vec<JsonPayloadInfo>,
    ads: Vec<JsonAdsInfo>,
}

/*
    Rust internal Structure
    Get error with "copy"

    split ads struct into 2 Hashmap
*/
#[derive(Clone)]
pub struct PayloadPathInfo {
    needroot: bool,
    file_type: String,
    fullpath: String,
    cmd_var: String,
    cmd_path: String,
}

#[derive(Clone)]
pub struct FileArtefact {
    magicbyte: HashMap<String, String>,
    payload: HashMap<String, PayloadPathInfo>,
    ads_adsname: HashMap<String, String>,
    ads_hexvalue: HashMap<String, String>,
}

impl FileArtefact {
    pub fn new() -> Self {
        Self {
            magicbyte: HashMap::new(),
            payload: HashMap::new(),
            ads_adsname: HashMap::new(),
            ads_hexvalue: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &str) {
        let file_path: PathBuf = std::env::current_dir()
            .expect("Failed to get current folder")
            .join(path);
        let json_open: File = File::open(file_path).expect("Unable to open json file");
        let json_data: JsonGlobalInfo =
            serde_json::from_reader(json_open).expect("error while reading or parsing the json");

        for data in json_data.magicbytes {
            self.magicbyte.insert(data.name, data.magicbyte);
        }

        for data in json_data.payloads {
            let tmp_data: PayloadPathInfo = {
                PayloadPathInfo {
                    needroot: data.needroot,
                    file_type: data.file_type,
                    fullpath: data.fullpath,
                    cmd_var: data.cmd_var,
                    cmd_path: data.cmd_path,
                }
            };
            self.payload.insert(data.name, tmp_data);
        }

        for data in json_data.ads {
            self.ads_adsname.insert(data.name.clone(), data.adsname);
            self.ads_hexvalue.insert(data.name, data.hexvalue);
        }
    }

    pub fn file_magicbyte_list(&self) -> HashSet<String> {
        self.magicbyte.keys().cloned().collect()
    }

    pub fn file_magicbyte_exist(&self, name: &str) -> bool {
        self.magicbyte.contains_key(name)
    }

    pub fn file_magicbyte_get(&self, name: &str) -> Vec<u8> {
        let mut payload: &str = "467261636b313133";
        if self.file_magicbyte_exist(name) {
            payload = self.magicbyte.get(name).clone().unwrap(); //Can not faild as the key exist
        }
        let header: Option<Vec<u8>> = tools::hex_to_bytes(payload); // User input 😅
        match header {
            Some(data) => data,
            None => vec![70, 114, 97, 99, 107, 49, 49, 51],
        }
    }

    pub fn file_payload_list(&self) -> HashSet<String> {
        self.payload.keys().cloned().collect()
    }

    pub fn file_payload_exist(&self, name: &str) -> bool {
        self.payload.contains_key(name)
    }

    pub fn file_payload_needroot(&self, name: &str) -> bool {
        let data: &PayloadPathInfo = self.payload.get(name).unwrap(); // exit fail if name is invalid
        data.needroot
    }

    pub fn file_payload_getfilename(&self, name: &str) -> String {
        if self.file_payload_exist(name) {
            let data: &PayloadPathInfo = self.payload.get(name).unwrap(); // name is a valid key

            if data.fullpath.len() > 0 {
                tools::regex_to_string(&data.fullpath)
            } else {
                let filename: String = tools::regex_to_string(&data.cmd_path);
                let var_path: OsString = env::var_os(&data.cmd_var).unwrap();
                let full_path: PathBuf = Path::new(&var_path).join(filename);
                String::from(full_path.to_string_lossy())
            }
        } else {
            "c:/wag.file".to_string()
        }
    }

    pub fn file_payload_getfiletype(&self, name: &str) -> String {
        if self.file_payload_exist(name) {
            let data: &PayloadPathInfo = self.payload.get(name).unwrap(); // name is a valid key
            data.file_type.clone()
        } else {
            "Wag".to_string()
        }
    }

    pub fn file_ads_list(&self) -> HashSet<String> {
        self.ads_adsname.keys().cloned().collect()
    }

    pub fn file_ads_exist(&self, name: &str) -> bool {
        self.ads_adsname.contains_key(name)
    }

    pub fn file_ads_get_data(&self, name: &str) -> Vec<u8> {
        println!("Ask for {}", name);
        let payload: &String = self.ads_hexvalue.get(name).clone().unwrap(); //Can not faild as the key exist
        println!("{}", payload);
        let header: Option<Vec<u8>> = tools::hex_to_bytes(payload); // User input 😅
        match header {
            Some(data) => data,
            None => vec![70, 114, 97, 99, 107, 49, 49, 51],
        }
    }

    pub fn file_ads_get_name(&self, name: &str) -> String {
        let data: &String = self.ads_adsname.get(name).clone().unwrap(); //Can not faild as the key exist
        data.to_string()
    }
}

pub fn create_file(fullpath: String, hex_data: Vec<u8>) -> bool {
    println!("Try to create : {}", fullpath);
    let file_path: &Path = Path::new(&fullpath);
    if !file_path.exists() {
        let folder: &Path = file_path.parent().unwrap();

        let ret_folder = std::fs::create_dir_all(folder);
        match ret_folder {
            Ok(_) => println!("The folder is valid"),
            Err(_) => return false,
        }

        let ret_file = std::fs::write(file_path, hex_data);
        match ret_file {
            Ok(_) => println!("The file is created"),
            Err(_) => return false,
        }

        let sleep_duration = time::Duration::from_millis(2000);
        thread::sleep(sleep_duration);

        let ret_remove = std::fs::remove_file(file_path);
        match ret_remove {
            Ok(_) => println!("The file is removed"),
            Err(_) => return false,
        }

        return true;
    }
    return false;
}

pub fn create_ads(fullpath: String, adsname: String, hex_data: Vec<u8>) -> bool {
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

    let result = unsafe {
        WriteFile(
            handle,
            Some(hex_data.as_slice()),
            Some(hex_data.len() as *mut u32),
            None,
        )
    };

    let _ = unsafe { CloseHandle(handle) };

    match result {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}
