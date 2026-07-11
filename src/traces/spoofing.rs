// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier},
    traces::{Trace, TraceMetadata, TraceParameter, TraceParameterKind},
    windows::processes::get_process_identifier,
};
use clap::Parser;
use serde::Deserialize;
use std::{
    ffi::OsString, iter::once, mem::size_of, os::windows::ffi::OsStrExt, path::PathBuf,
    process::ExitCode, sync::LazyLock,
};
use uuid::Uuid;
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

#[derive(Parser, Deserialize)]
pub struct Spoofing {
    #[clap(required = true, help = "Path to the executable")]
    executable: PathBuf,
    #[clap(required = true, help = "Name of the parent executable")]
    parent_executable: String,
}

pub static METADATA: LazyLock<TraceMetadata> = LazyLock::new(|| TraceMetadata {
    identifier: TraceIdentifier::new("process", "spoofing", "create"),
    name: "Spoofing",
    requirements_summary: "parent process access, process creation",
    attack_techniques: vec![AttackTechnique::new("1134", Some("004"))],
    sigma_identifiers: vec![SigmaIdentifier::new(
        Uuid::parse_str("a05efb77-9c1d-4384-806e-080ca13946fe").unwrap(),
    )],
    use_cases: vec!["process spoofing".to_string()],
    parameters: &[
        TraceParameter {
            name: "executable",
            kind: TraceParameterKind::Path,
            required: true,
            allowed_values: &[],
        },
        TraceParameter {
            name: "parent_executable",
            kind: TraceParameterKind::String,
            required: true,
            allowed_values: &[],
        },
    ],
});

impl Trace for Spoofing {
    fn run(&self) -> ExitCode {
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

        if unsafe {
            InitializeProcThreadAttributeList(
                Some(startup_information.lpAttributeList),
                1,
                None,
                &mut required_size,
            )
        }
        .is_err()
        {
            return ExitCode::FAILURE;
        }

        let parent_process_identifier =
            match get_process_identifier(self.parent_executable.as_str()) {
                Ok(process_identifier) => process_identifier,
                Err(_) => return ExitCode::FAILURE,
            };

        let mut parent_process = match unsafe {
            OpenProcess(PROCESS_CREATE_PROCESS, false, parent_process_identifier)
        } {
            Ok(handle) => unsafe { Owned::new(handle) },
            Err(_) => return ExitCode::FAILURE,
        };

        if unsafe {
            UpdateProcThreadAttribute(
                startup_information.lpAttributeList,
                0,
                PROC_THREAD_ATTRIBUTE_PARENT_PROCESS as usize,
                Some(&mut *parent_process as *mut _ as *mut _),
                size_of::<HANDLE>(),
                None,
                None,
            )
        }
        .is_err()
        {
            return ExitCode::FAILURE;
        }

        if unsafe {
            CreateProcessW(
                None,
                Some(PWSTR(
                    OsString::from(self.executable.as_os_str())
                        .encode_wide()
                        .chain(once(0))
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                )),
                None,
                None,
                false,
                EXTENDED_STARTUPINFO_PRESENT,
                None,
                None,
                &startup_information.StartupInfo,
                &mut PROCESS_INFORMATION::default(),
            )
        }
        .is_err()
        {
            return ExitCode::FAILURE;
        }

        ExitCode::SUCCESS
    }
}
