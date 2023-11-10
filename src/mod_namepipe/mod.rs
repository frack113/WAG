//
// Name Pipe Artefact
//
// Windows API
use windows::core::{Result, PCSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::PIPE_ACCESS_DUPLEX;
use windows::Win32::System::Pipes::{CreateNamedPipeA, PIPE_TYPE_MESSAGE};

use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::PathBuf;
use std::{thread, time};

//Structure for the Json
#[derive(Deserialize)]
struct JsonNamepipeInfo {
    name: String,
    namepipe: Vec<String>,
}

#[derive(Deserialize)]
struct JsonGlobalInfo {
    dictionary: Vec<JsonNamepipeInfo>,
}

//Structure form the name pipe
// Very simple struct
#[derive(Clone)]
pub struct NamePipeArtefact {
    namepipe: HashMap<String, Vec<String>>,
}

impl NamePipeArtefact {
    pub fn new() -> Self {
        Self {
            namepipe: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &str) {
        let file_path: PathBuf = std::env::current_dir()
            .expect("Failed to get current folder")
            .join(path);
        let json_open: File = File::open(file_path).expect("Unable to open json file");
        let json_data: JsonGlobalInfo =
            serde_json::from_reader(json_open).expect("error while reading or parsing the json");

        for data in json_data.dictionary {
            self.namepipe.insert(data.name, data.namepipe);
        }
    }

    pub fn namepipe_list(&self) -> HashSet<String> {
        self.namepipe.keys().cloned().collect()
    }

    pub fn namepipe_value_list(&self, name: &str) -> Vec<String> {
        self.namepipe.get(name).unwrap().to_vec()
    }

    pub fn namepipe_exist(&self, name: &str) -> bool {
        self.namepipe.contains_key(name)
    }

    pub fn namepipe_get_value_at_index(&self, name: &str, number: usize) -> String {
        let mut index_payload: usize = number;
        let namepipe_value_list: Vec<String> = self.namepipe_value_list(name);
        //Don't trust humain
        if index_payload > namepipe_value_list.len() - 1 {
            index_payload = 0;
        }
        let barrow: &String = &namepipe_value_list[index_payload];
        barrow.to_owned()
    }
}

pub fn create_name_pipe(name: &String, wait: u64) {
    let full_malware_pipe: String = format!("\\\\.\\pipe\\{}\0", name);
    let pipe_name: PCSTR = PCSTR::from_raw(full_malware_pipe.as_ptr());
    let server_pipe: Result<HANDLE> = unsafe {
        CreateNamedPipeA(
            pipe_name,
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_MESSAGE,
            1,
            2048,
            2048,
            0,
            None,
        )
    };
    let sleep_duration: time::Duration = time::Duration::from_millis(wait);
    thread::sleep(sleep_duration);
    let _res_server_pipe = unsafe { CloseHandle(server_pipe.unwrap()) };
}
