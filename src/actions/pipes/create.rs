// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Name Pipe
//
// Last update 20240224

use crate::actions::Runnable;
use clap::Parser;
use regex_generate::{Generator, DEFAULT_MAX_REPEAT};
use std::{thread, time};
use windows::{
    core::{Result as WindowsResult, PCSTR},
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        Storage::FileSystem::PIPE_ACCESS_DUPLEX,
        System::Pipes::{CreateNamedPipeA, PIPE_TYPE_MESSAGE},
    },
};

#[derive(Debug, Parser)]
pub struct Create {
    #[clap(
        short = 'n',
        long,
        required = true,
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

impl Runnable for Create {
    fn run(&self) -> i32 {
        println!("Create NamePipe");

        let mut generator: Generator<rand::rngs::ThreadRng> =
            match Generator::new(&self.name, rand::thread_rng(), DEFAULT_MAX_REPEAT) {
                Ok(generator) => generator,
                Err(_) => {
                    println!("Regex expressions are malformed.");

                    return 1;
                }
            };
        let mut buffer: Vec<u8> = vec![];
        generator.generate(&mut buffer).unwrap();
        let payload: String = match String::from_utf8(buffer) {
            Ok(string) => string,
            Err(_) => {
                println!("Filename contains non-utf8 characters.");

                return 1;
            }
        };

        println!("Create the namepipe : {}", payload);

        create_name_pipe(&payload, 2000);

        return 0;
    }
}
