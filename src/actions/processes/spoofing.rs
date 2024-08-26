// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

// PPID Spoofing
//
// Last update 20240224

use crate::actions::Runnable;
use clap::Parser;
use core::ffi::c_void;
use std::{
    error::Error,
    ffi::OsString,
    fmt::{Display, Formatter, Result as FormatterResult},
    mem::size_of,
    os::windows::ffi::OsStringExt,
    thread,
    time::Duration,
};
use windows::{
    core::{Owned, PSTR},
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
                TH32CS_SNAPPROCESS,
            },
            Memory::{GetProcessHeap, HeapAlloc, HEAP_FLAGS},
            Threading::{
                CreateProcessA, InitializeProcThreadAttributeList, OpenProcess, TerminateProcess,
                UpdateProcThreadAttribute, WaitForSingleObject, LPPROC_THREAD_ATTRIBUTE_LIST,
                PROCESS_ACCESS_RIGHTS, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION,
                PROC_THREAD_ATTRIBUTE_PARENT_PROCESS, STARTF_USESHOWWINDOW, STARTUPINFOEXA,
            },
        },
    },
};

#[derive(Debug, Parser)]
pub struct Spoofing {
    #[clap(
        short = 'e',
        long,
        required = true,
        help = "Full path to the executable eg: c:\\temp..."
    )]
    executable: String,
    #[clap(
        short = 'p',
        long,
        required = true,
        help = "Full path to the parent executable eg: c:\\temp..."
    )]
    parent_executable: String,
}

#[derive(Debug)]
struct ProcessNotFound;

impl Error for ProcessNotFound {}

impl Display for ProcessNotFound {
    fn fmt(&self, formatter: &mut Formatter) -> FormatterResult {
        write!(formatter, "Process not found")
    }
}

fn get_pid_from_name(name: &str) -> Result<u32, Box<dyn Error>> {
    let snapshot: Owned<HANDLE> =
        unsafe { Owned::new(CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?) };
    let mut process_entry: PROCESSENTRY32W = PROCESSENTRY32W::default();
    process_entry.dwSize = size_of::<PROCESSENTRY32W>() as u32;

    unsafe {
        Process32FirstW(*snapshot, &mut process_entry)?;
    }

    loop {
        if OsString::from_wide(
            process_entry
                .szExeFile
                .into_iter()
                .take_while(|&byte| byte != 0)
                .collect::<Vec<_>>()
                .as_slice(),
        ) == name
        {
            return Ok(process_entry.th32ProcessID);
        }

        if unsafe { Process32NextW(*snapshot, &mut process_entry) }.is_err() {
            break;
        }
    }

    Err(Box::new(ProcessNotFound))
}

fn create_ppid(name: &String, new_ppid: u32) -> bool {
    println!("Use the PPID {}", new_ppid);
    println!("Open the Parent Process");
    let mut parent_process_handle: HANDLE =
        unsafe { OpenProcess(PROCESS_ACCESS_RIGHTS(0x02000000), false, new_ppid).unwrap() };

    let mut pi: PROCESS_INFORMATION = PROCESS_INFORMATION::default();
    let mut sinfo: STARTUPINFOEXA = STARTUPINFOEXA::default();
    let mut cb_attribute_list_size: usize = size_of::<STARTUPINFOEXA>();
    sinfo.StartupInfo.cb = cb_attribute_list_size as u32;
    sinfo.StartupInfo.dwFlags = STARTF_USESHOWWINDOW;

    println!("allocate memory for PROC_THREAD_ATTRIBUTE_LIST");
    sinfo.lpAttributeList = LPPROC_THREAD_ATTRIBUTE_LIST(unsafe {
        HeapAlloc(
            GetProcessHeap().unwrap(),
            HEAP_FLAGS(0),
            cb_attribute_list_size,
        )
    });

    println!("InitializeProcThreadAttributeList");
    unsafe {
        InitializeProcThreadAttributeList(sinfo.lpAttributeList, 1, 0, &mut cb_attribute_list_size)
            .unwrap()
    };

    println!("UpdateProcThreadAttribute");
    let _ = unsafe {
        UpdateProcThreadAttribute(
            sinfo.lpAttributeList,
            0,
            PROC_THREAD_ATTRIBUTE_PARENT_PROCESS as usize,
            Some(&mut parent_process_handle as *mut _ as *mut c_void),
            size_of::<HANDLE>(),
            None,
            None,
        )
    };

    println!("CreateProcessA");
    let process_name = format!("{}\0", name);
    let new_process = unsafe {
        CreateProcessA(
            None,
            PSTR::from_raw(process_name.to_owned().as_mut_ptr()),
            None,
            None,
            false,
            PROCESS_CREATION_FLAGS(0x00080000),
            None,
            None,
            &sinfo.StartupInfo,
            &mut pi,
        )
    };
    match new_process {
        Ok(_) => {
            println!("New process is created with pid {:}", pi.dwProcessId);
            let wait_duration: Duration = Duration::from_millis(2000);
            thread::sleep(wait_duration);
            let _ = unsafe { TerminateProcess(pi.hProcess, 0) };
            let _ = unsafe { WaitForSingleObject(pi.hProcess, 5000) };
            let _ = unsafe { CloseHandle(pi.hProcess) };
            let _ = unsafe { CloseHandle(pi.hThread) };
            true
        }
        Err(_) => false,
    }
}

impl Runnable for Spoofing {
    /* Version 20240209 */
    fn run(&self) -> Result<i32, Box<dyn Error>> {
        println!("PPID spoofing");
        let result: bool = create_ppid(
            &self.executable,
            get_pid_from_name(&self.parent_executable)?,
        );

        Ok(!result as i32)
    }
}
