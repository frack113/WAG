// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Load Vulnerable Driver
//
// Last update 20240224

use crate::{actions::Runnable, windows::users::is_administrator};
use clap::Parser;
use std::{error::Error, thread, time};
use windows::{
    core::{Result as WindowsResult, PCWSTR},
    Win32::System::Services::{
        ControlService, CreateServiceW, DeleteService, OpenSCManagerW, StartServiceW,
        ENUM_SERVICE_TYPE, SC_HANDLE, SC_MANAGER_ALL_ACCESS, SERVICE_CONTROL_STOP, SERVICE_ERROR,
        SERVICE_START_TYPE, SERVICE_STATUS,
    },
};

#[derive(Debug, Parser)]
pub struct Create {
    #[clap(
        short = 'n',
        long,
        required = true,
        help = "Internal Name of the service"
    )]
    internal: String,
    #[clap(
        short = 'd',
        long,
        required = true,
        help = "Displayed Name of the service"
    )]
    display: String,
    #[clap(
        short = 'p',
        long,
        required = true,
        help = "Full path to the driver eg: c:\\temp..."
    )]
    path: String,
}

fn create_driver_service(name: &str, details: &str, path: &str) -> bool {
    println!("Open the service manager");
    let scmanager: SC_HANDLE =
        unsafe { OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_ALL_ACCESS) }
            .expect("Sc Manager open failure");

    let mut service_name: Vec<u16> = name.encode_utf16().collect();
    service_name.push(0);
    let mut service_display: Vec<u16> = details.encode_utf16().collect();
    service_display.push(0);
    let mut service_path: Vec<u16> = path.encode_utf16().collect();
    service_path.push(0);

    println!("Create the service manager");

    let service_handle: SC_HANDLE = match unsafe {
        CreateServiceW(
            scmanager,
            PCWSTR::from_raw(service_name.as_ptr()),
            PCWSTR::from_raw(service_display.as_ptr()),
            0xF003F,
            ENUM_SERVICE_TYPE(1),
            SERVICE_START_TYPE(2),
            SERVICE_ERROR(0),
            PCWSTR::from_raw(service_path.as_ptr()),
            PCWSTR::null(),
            None,
            PCWSTR::null(),
            PCWSTR::null(),
            PCWSTR::null(),
        )
    } {
        Ok(value) => value,
        Err(_) => {
            println!("Service creation failure");
            return false;
        }
    };

    println!("Start Service ");

    match unsafe { StartServiceW(service_handle, None) } {
        Ok(_) => {
            println!("Wait a little");
            let sleep_duration: time::Duration = time::Duration::from_millis(2000);
            thread::sleep(sleep_duration);
            let mut service_status: SERVICE_STATUS = unsafe { std::mem::zeroed() };
            println!("Stop Service");
            let _result_stop: WindowsResult<()> = unsafe {
                ControlService(service_handle, SERVICE_CONTROL_STOP, &mut service_status)
            };
        }
        Err(value) => {
            println!("Service Start failure with code : {:#06x}", value.code().0);
        }
    };

    match unsafe { DeleteService(service_handle) } {
        Ok(_) => {
            println!("Service remove succeed");
            true
        }
        Err(value) => {
            println!("Service remove failure with code : {:#06x}", value.code().0);
            false
        }
    }
}

impl Runnable for Create {
    /* Version 20230908 */
    fn run(&self) -> Result<i32, Box<dyn Error>> {
        println!("Bring Your Own Vulnerable Driver");

        if !is_administrator()? {
            println!("Need to have Administrator right to create the service");
            return Ok(1);
        }

        // Todo check path is valid or not :)

        let result: bool = create_driver_service(&self.internal, &self.display, &self.path);

        Ok(!result as i32)
    }
}
