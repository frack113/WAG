// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    traces::{TraceError, spoofing::parameters::SpoofingParameters},
    windows::processes,
};
use std::{ffi::OsString, iter, mem, os::windows::ffi::OsStrExt};
use windows::{
    Win32::{
        Foundation::HANDLE,
        System::Threading::{
            CreateProcessW, EXTENDED_STARTUPINFO_PRESENT, InitializeProcThreadAttributeList,
            LPPROC_THREAD_ATTRIBUTE_LIST, OpenProcess, PROC_THREAD_ATTRIBUTE_PARENT_PROCESS,
            PROCESS_CREATE_PROCESS, PROCESS_INFORMATION, STARTUPINFOEXW, STARTUPINFOW,
            UpdateProcThreadAttribute,
        },
    },
    core::{Owned, PWSTR},
};

pub(super) fn execute(parameters: SpoofingParameters) -> Result<(), TraceError> {
    let mut required_size = 0;
    let _ = unsafe { InitializeProcThreadAttributeList(None, 1, None, &mut required_size) };

    let mut attributes: Box<[u8]> = vec![0; required_size].into_boxed_slice();
    let attributes_list = unsafe {
        Owned::new(LPPROC_THREAD_ATTRIBUTE_LIST(
            attributes.as_mut_ptr() as *mut _
        ))
    };
    let startup_information = STARTUPINFOEXW {
        StartupInfo: STARTUPINFOW {
            cb: size_of::<STARTUPINFOEXW>() as u32,
            ..Default::default()
        },
        lpAttributeList: *attributes_list,
    };

    unsafe {
        InitializeProcThreadAttributeList(
            Some(startup_information.lpAttributeList),
            1,
            None,
            &mut required_size,
        )?
    };

    let parent_process_identifier =
        processes::get_process_identifier(parameters.parent_executable.as_str())?;
    let mut parent_process = unsafe {
        Owned::new(OpenProcess(
            PROCESS_CREATE_PROCESS,
            false,
            parent_process_identifier,
        )?)
    };

    unsafe {
        UpdateProcThreadAttribute(
            startup_information.lpAttributeList,
            0,
            PROC_THREAD_ATTRIBUTE_PARENT_PROCESS as usize,
            Some(&mut *parent_process as *mut _ as *mut _),
            mem::size_of::<HANDLE>(),
            None,
            None,
        )?
    };

    let mut command_line: Vec<u16> = OsString::from(parameters.executable.as_os_str())
        .encode_wide()
        .chain(iter::once(0))
        .collect();
    let mut process_information = PROCESS_INFORMATION::default();

    unsafe {
        CreateProcessW(
            None,
            Some(PWSTR(command_line.as_mut_ptr())),
            None,
            None,
            false,
            EXTENDED_STARTUPINFO_PRESENT,
            None,
            None,
            &startup_information.StartupInfo,
            &mut process_information,
        )?
    };

    let _process = unsafe { Owned::new(process_information.hProcess) };
    let _thread = unsafe { Owned::new(process_information.hThread) };

    Ok(())
}
