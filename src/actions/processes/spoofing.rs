// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{actions::Runnable, windows::processes::get_pid};
use clap::Parser;
use std::{error::Error, ffi::OsString, iter::once, mem::size_of, os::windows::ffi::OsStrExt};
use windows::{
    core::{Owned, PWSTR},
    Win32::{
        Foundation::HANDLE,
        System::Threading::{
            CreateProcessW, InitializeProcThreadAttributeList, OpenProcess,
            UpdateProcThreadAttribute, EXTENDED_STARTUPINFO_PRESENT, LPPROC_THREAD_ATTRIBUTE_LIST,
            PROCESS_INFORMATION, PROCESS_SET_INFORMATION, PROC_THREAD_ATTRIBUTE_PARENT_PROCESS,
            STARTUPINFOEXW, STARTUPINFOW,
        },
    },
};

#[derive(Debug, Parser)]
pub struct Spoofing {
    #[clap(short = 'e', long, required = true, help = "Path to the executable")]
    executable: String,
    #[clap(
        short = 'p',
        long,
        required = true,
        help = "Name of the parent executable"
    )]
    parent_executable: String,
}

fn spoof(executable: &str, parent_pid: u32) -> Result<(), Box<dyn Error>> {
    let mut required_size: usize = 0;

    unsafe {
        let _ = InitializeProcThreadAttributeList(
            LPPROC_THREAD_ATTRIBUTE_LIST::default(),
            1,
            0,
            &mut required_size,
        );
    };

    let mut attributes: Box<[u8]> = vec![0; required_size].into_boxed_slice();
    let startup_informations: STARTUPINFOEXW = STARTUPINFOEXW {
        StartupInfo: STARTUPINFOW {
            cb: size_of::<STARTUPINFOEXW>() as u32,
            ..Default::default()
        },
        lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST(attributes.as_mut_ptr() as *mut _),
    };

    unsafe {
        InitializeProcThreadAttributeList(
            startup_informations.lpAttributeList,
            1,
            0,
            &mut required_size,
        )?;

        UpdateProcThreadAttribute(
            startup_informations.lpAttributeList,
            0,
            PROC_THREAD_ATTRIBUTE_PARENT_PROCESS as usize,
            Some(
                &mut *Owned::new(OpenProcess(PROCESS_SET_INFORMATION, false, parent_pid)?) as *mut _
                    as *mut _,
            ),
            size_of::<HANDLE>(),
            None,
            None,
        )?;

        CreateProcessW(
            None,
            PWSTR(
                OsString::from(executable)
                    .encode_wide()
                    .chain(once(0))
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
            ),
            None,
            None,
            false,
            EXTENDED_STARTUPINFO_PRESENT,
            None,
            None,
            &startup_informations.StartupInfo,
            &mut PROCESS_INFORMATION::default(),
        )?
    };

    Ok(())
}

impl Runnable for Spoofing {
    fn run(&self) -> Result<i32, Box<dyn Error>> {
        spoof(&self.executable, get_pid(&self.parent_executable)?)?;

        Ok(0)
    }
}
