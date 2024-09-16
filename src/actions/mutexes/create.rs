// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Mutex
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
        System::Threading::CreateMutexA,
    },
};

#[derive(Debug, Parser)]
pub struct Create {
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

impl Runnable for Create {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Create Mutex");

        let mut generator: Generator<rand::rngs::ThreadRng> =
            Generator::new(&self.name, rand::thread_rng(), DEFAULT_MAX_REPEAT)?;
        let mut buffer: Vec<u8> = vec![];
        generator.generate(&mut buffer).unwrap();
        let payload: String = String::from_utf8(buffer)?;

        println!("Create the Mutex : {}", payload);

        create_mutex(&payload, 2000);

        Ok(())
    }
}
