// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Mutex
//
// Last update 20240224

use clap::Parser;
use regex_generate::{Generator, DEFAULT_MAX_REPEAT};
use std::{thread, time};
use windows::{
    core::{Result as WindowsResult, PCSTR},
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::Threading::CreateMutexA,
    },
};

#[derive(Parser)]
pub struct Mutex {
    #[clap(
        short = 'n',
        long,
        required = true,
        help = "Regex of the Mutex to Create"
    )]
    name: String,
}

fn create_mutex(name: &String, wait: u64) {
    let full_malware_mutex: String = format!("{}\0", name);
    let mutex_name: PCSTR = PCSTR::from_raw(full_malware_mutex.as_ptr());
    let mutex_handle: WindowsResult<HANDLE> = unsafe { CreateMutexA(None, true, mutex_name) };
    let sleep_duration: time::Duration = time::Duration::from_millis(wait);
    thread::sleep(sleep_duration);
    let _res_server_pipe: WindowsResult<()> = unsafe { CloseHandle(mutex_handle.unwrap()) };
}

impl Mutex {
    pub fn run(&self) -> i32 {
        println!("Create Mutex");

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

        println!("Create the Mutex : {}", payload);

        create_mutex(&payload, 2000);

        return 0;
    }
}
