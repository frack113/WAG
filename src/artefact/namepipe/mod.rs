//
// Name Pipe Artefact
//
// Windows API
use windows::core::{Result as WindowsResult, PCSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::PIPE_ACCESS_DUPLEX;
use windows::Win32::System::Pipes::{CreateNamedPipeA, PIPE_TYPE_MESSAGE};

use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::PathBuf;
use std::{thread, time};

use super::tools::{pretty_print_hashset, regex_to_string, EXIST_ALL_GOOD, EXIST_CLI_ERROR};

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
struct NamePipeArtefact {
    namepipe: HashMap<String, Vec<String>>,
}

impl NamePipeArtefact {
    fn new() -> Self {
        Self {
            namepipe: HashMap::new(),
        }
    }

    fn load(&mut self, path: &str) {
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

    fn namepipe_list(&self) -> HashSet<String> {
        self.namepipe.keys().cloned().collect()
    }

    fn namepipe_value_list(&self, name: &str) -> Vec<String> {
        self.namepipe.get(name).unwrap().to_vec()
    }

    fn namepipe_exist(&self, name: &str) -> bool {
        self.namepipe.contains_key(name)
    }

    fn namepipe_get_value_at_index(&self, name: &str, number: usize) -> String {
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

fn create_name_pipe(name: &String, wait: u64) {
    let full_malware_pipe: String = format!("\\\\.\\pipe\\{}\0", name);
    let pipe_name: PCSTR = PCSTR::from_raw(full_malware_pipe.as_ptr());
    let server_pipe: WindowsResult<HANDLE> = unsafe {
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
    let _res_server_pipe: WindowsResult<()> = unsafe { CloseHandle(server_pipe.unwrap()) };
}

/* Version 20230908 */
pub fn run_pipecreate(
    module: String,
    number: usize,
    get: bool,
    details: bool,
    name: String,
) -> i32 {
    println!("Create NamePipe");
    let mut artefact: NamePipeArtefact = NamePipeArtefact::new();
    artefact.load("data/namepipe.json");

    let full_payload: String;

    if get == true {
        let all_name: HashSet<String> = artefact.namepipe_list();
        pretty_print_hashset("Name for the mimic Name Pipe".to_string(), all_name);
        return EXIST_ALL_GOOD;
    }
    if module == "manual" {
        if name.len() > 0 {
            full_payload = regex_to_string(&name);
        } else {
            return EXIST_CLI_ERROR;
        }
    } else {
        if artefact.namepipe_exist(&module) == false {
            println!("Did not find \"{}\" name for namepipe", module);
            println!("You can use the help option --help");
            return EXIST_CLI_ERROR;
        }

        if details == true {
            println!("Name Pipe number for \"{}\" :", module);
            println!("----------------");
            let list_name_pipe: Vec<String> = artefact.namepipe_value_list(&module);
            for i in 0..list_name_pipe.len() {
                println!(" {} - {}", i, list_name_pipe[i])
            }
            println!("----------------");
            println!("bye");
            return EXIST_ALL_GOOD;
        }

        let payload: String = artefact.namepipe_get_value_at_index(&module, number);
        full_payload = regex_to_string(&payload);
    }

    println!("Create the namepipe : {}", full_payload);
    create_name_pipe(&full_payload, 2000);
    return EXIST_ALL_GOOD;
}
