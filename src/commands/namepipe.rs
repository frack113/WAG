//
// Name Pipe
//
// Last update 20240224

use windows::core::{Result as WindowsResult, PCSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::PIPE_ACCESS_DUPLEX;
use windows::Win32::System::Pipes::{CreateNamedPipeA, PIPE_TYPE_MESSAGE};

use crate::commands::tools::{regex_to_string, EXIST_ALL_GOOD};
use clap::Parser;
use std::{thread, time};

#[derive(Parser)]
pub struct NamePipe {
    #[clap(
        short = 'n',
        long,
        required = true,
        default_value = "wag",
        help = "Regex of the PipeName to Create"
    )]
    name: String,
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

impl NamePipe {
    pub fn run(&self) -> i32 {
        println!("Create NamePipe");
        let full_payload: String;
        full_payload = regex_to_string(&self.name);
        println!("Create the namepipe : {}", full_payload);
        create_name_pipe(&full_payload, 2000);
        return EXIST_ALL_GOOD;
    }
}
