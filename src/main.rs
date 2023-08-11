// Json
extern crate serde;
//use serde_json::Value;
use serde::{Deserialize, Serialize};
//use serde_json::Result;

// File
use std::fs::File;
use std::path::Path;

// Windows API
use winapi::um::winbase::CreateNamedPipeA;
use winapi::um::winnt::{HANDLE,LPCSTR};
//use winapi::um::namedpipeapi::ConnectNamedPipe;
//use winapi::um::fileapi::WriteFile;
use winapi::um::winbase::{PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE};
use std::ptr::null_mut;

#[derive(Serialize, Deserialize)]
struct DataJson {
    namepipe: String,
}

fn main() {

    // path manipulation is a big try can be better 
    let root_path = std::env::current_dir().expect("Error when read the current path");
    let myfile = root_path.join("data").join("cobal_1.json");
    println!("Open file : {:?}",myfile);

    //let read the json file
    let json_file_path = Path::new(&myfile);
    let file = File::open(json_file_path).expect("Unable to open file");
    let malware:DataJson = serde_json::from_reader(file).expect("error while reading or parsing");

    let full_malware_pipe = format!("\\\\.\\pipe\\{}\0",malware.namepipe);

    let pipe_name : LPCSTR = full_malware_pipe.as_ptr() as *const i8;
    println!("Create Pipe : {full_malware_pipe}");
    let _server_pipe : HANDLE = unsafe {CreateNamedPipeA(pipe_name,PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE,1,2048,2048,0,null_mut())};
}