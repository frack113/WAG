// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FormatterResult},
};
use windows::{
    core::{Owned, HSTRING},
    Win32::{
        Foundation::HANDLE,
        System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
            TH32CS_SNAPPROCESS,
        },
    },
};

#[derive(Debug)]
pub struct ProcessNotFound;

impl Error for ProcessNotFound {}

impl Display for ProcessNotFound {
    fn fmt(&self, formatter: &mut Formatter) -> FormatterResult {
        write!(formatter, "Process not found")
    }
}

pub fn get_pid(name: &str) -> Result<u32, Box<dyn Error>> {
    let snapshot: Owned<HANDLE> =
        unsafe { Owned::new(CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?) };
    let mut process_entry: PROCESSENTRY32W = PROCESSENTRY32W {
        dwSize: size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    unsafe {
        Process32FirstW(*snapshot, &mut process_entry)?;
    }

    loop {
        if HSTRING::from_wide(
            process_entry
                .szExeFile
                .into_iter()
                .take_while(|&byte| byte != 0)
                .collect::<Vec<_>>()
                .as_slice(),
        )? == name
        {
            return Ok(process_entry.th32ProcessID);
        }

        if unsafe { Process32NextW(*snapshot, &mut process_entry) }.is_err() {
            break;
        }
    }

    Err(Box::new(ProcessNotFound))
}
