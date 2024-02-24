//
// PPID Artefact generator
//

use crate::commands::tools::{EXIST_ALL_GOOD, EXIST_TEST_ERROR};
use clap::Parser;

use rand::prelude::SliceRandom;
use sysinfo::System;

use core::ffi::c_void;
use std::mem::size_of;
use windows::core::PSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Memory::{GetProcessHeap, HeapAlloc, HEAP_FLAGS};
use windows::Win32::System::Threading::{
    CreateProcessA, InitializeProcThreadAttributeList, OpenProcess, TerminateProcess,
    UpdateProcThreadAttribute, WaitForSingleObject, LPPROC_THREAD_ATTRIBUTE_LIST,
    PROCESS_ACCESS_RIGHTS, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION,
    PROC_THREAD_ATTRIBUTE_PARENT_PROCESS, STARTF_USESHOWWINDOW, STARTUPINFOEXA,
};

use std::thread;
use std::time::Duration;

#[derive(Parser)]
pub struct PPID {
    #[clap(short = 'p', long, help = "Full path to the driver eg: c:\\temp...")]
    path: String,
}

/* Use internal rust command */
fn get_user_pid() -> u32 {
    let s: System = System::new_all();
    let mut ppid_list: Vec<u32> = Vec::<u32>::new();
    for (pid, process) in s.processes() {
        if process.user_id().is_some() {
            ppid_list.push(pid.as_u32());
        }
    }
    let new_ppid: u32 = ppid_list
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned();
    return new_ppid;
}

fn create_ppid(name: &String) -> bool {
    let new_ppid: u32 = get_user_pid();
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
            &mut sinfo.StartupInfo,
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
            return true;
        }
        Err(_) => return false,
    }
}

impl PPID {
    /* Version 20240209 */
    pub fn run(&self) -> i32 {
        println!("PPID spoofing");
        let result: bool = create_ppid(&self.path);
        if result {
            println!("All good ");
            return EXIST_ALL_GOOD;
        } else {
            println!("Sorry get a error");
            return EXIST_TEST_ERROR;
        }
    }
}
