// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Name Pipe
//
// Last update 20240224

use crate::actions::Runnable;
use clap::Parser;
use regex_generate::{Generator, DEFAULT_MAX_REPEAT};
use std::{error::Error, thread, time};
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
    fn run(&self) -> Result<i32, Box<dyn Error>> {
        println!("Create NamePipe");

        let mut generator: Generator<rand::rngs::ThreadRng> =
            Generator::new(&self.name, rand::thread_rng(), DEFAULT_MAX_REPEAT)?;
        let mut buffer: Vec<u8> = vec![];
        generator.generate(&mut buffer).unwrap();
        let payload: String = String::from_utf8(buffer)?;

        println!("Create the namepipe : {}", payload);

        create_name_pipe(&payload, 2000);

        Ok(0)
    }
}
