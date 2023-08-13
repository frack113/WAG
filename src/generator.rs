//
// Artefact generator
//

// Windows API
use winapi::um::winbase::CreateNamedPipeA;
use winapi::um::winbase::{PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{HANDLE,LPCSTR};
use std::ptr::null_mut;

// Some others
use std::{thread, time};

// For regex to string
//extern crate regex_generate;
//extern crate rand;
use regex_generate::{DEFAULT_MAX_REPEAT, Generator};

pub fn create_name_pipe(name:&String,wait:u64) {
    let full_malware_pipe = format!("\\\\.\\pipe\\{}\0",name);
    let pipe_name : LPCSTR = full_malware_pipe.as_ptr() as *const i8;
    let server_pipe : HANDLE = unsafe {CreateNamedPipeA(pipe_name,PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE,1,2048,2048,0,null_mut())};
    let sleep_duration = time::Duration::from_millis(wait);
    thread::sleep(sleep_duration);
    let _res_server_pipe = unsafe { CloseHandle(server_pipe) };
}

pub fn regex_to_string(name:&String) -> String {
    let mut gen = Generator::new(name, rand::thread_rng(), DEFAULT_MAX_REPEAT).unwrap();
    let mut buffer = vec![];
    gen.generate(&mut buffer).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    return output;
}