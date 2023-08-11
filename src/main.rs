//
// Windows Artefact Generator POC
//

// Cli option 
use clap::Parser;

// Json
extern crate serde;
use serde::{Deserialize, Serialize};

// File
use std::fs::File;

// Windows API
use winapi::um::winbase::CreateNamedPipeA;
use winapi::um::winnt::{HANDLE,LPCSTR};
//use winapi::um::namedpipeapi::ConnectNamedPipe;
//use winapi::um::fileapi::WriteFile;
use winapi::um::winbase::{PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE};
use std::ptr::null_mut;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "Windows Artefact Generator")]
#[command(author = "S0me One in the Net")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    
    #[arg(short, long,default_value_t=String::from("validator"))]
    name: String,
}


#[derive(Serialize, Deserialize)]
struct DataJson {
    namepipe: String,
}

fn main() {

    let cli = Cli::parse();
    
    //
    // Let start with the test file
    //
    let json_name =  cli.name + &String::from(".json");
    let root_path = std::env::current_dir().expect("Error when read the current path");
    let myfile = root_path.join("data").join(json_name);
    println!("Open file : {:?}",myfile);
    let file = File::open(myfile).expect("Unable to open file");
    let malware:DataJson = serde_json::from_reader(file).expect("error while reading or parsing");

    //
    // Build the name pipe form regex 
    //
    let full_malware_pipe = format!("\\\\.\\pipe\\{}\0",malware.namepipe);

    //
    // Create the Name Pipe
    //
    let pipe_name : LPCSTR = full_malware_pipe.as_ptr() as *const i8;
    println!("Create Pipe : {full_malware_pipe}");
    let _server_pipe : HANDLE = unsafe {CreateNamedPipeA(pipe_name,PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE,1,2048,2048,0,null_mut())};
}