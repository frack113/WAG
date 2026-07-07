// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    commands::traces::{Trace, TraceMetadata},
    displayer::Displayer,
};
use clap::Parser;
use serde::Deserialize;
use std::{path::PathBuf, process::ExitCode};
use windows::{
    Win32::System::LibraryLoader,
    core::{HSTRING, Owned},
};

#[derive(Parser, Deserialize)]
pub struct DllLoader {
    #[clap(help = "Path to the DLL file")]
    path: PathBuf,
}

pub const METADATA: TraceMetadata = TraceMetadata {
    identifier: "memory.dll.load",
    name: "DLL Loader",
    requirements_summary: "local DLL file",
    attack_techniques: &["T1574.001"],
    sigma_targets: &["dll_sideloading"],
    use_cases: &["dll sideloading"],
};

impl Trace for DllLoader {
    fn run(&self) -> ExitCode {
        let mut displayer = Displayer::new();
        displayer.loading("Verifying the DLL path");

        if !self.path.exists() {
            displayer.failure(&format!("DLL file does not exist: {}", self.path.display()));

            return ExitCode::FAILURE;
        }

        if !self.path.is_file() {
            displayer.failure(&format!("DLL path is not a file: {}", self.path.display()));

            return ExitCode::FAILURE;
        }

        displayer.success("The DLL path is verified");
        displayer.loading("Loading DLL in memory");

        let dll_handle = match unsafe {
            LibraryLoader::LoadLibraryW(&HSTRING::from(self.path.to_str().unwrap()))
        } {
            Ok(handle) => unsafe { Owned::new(handle) },
            Err(error) => {
                displayer.failure(&format!("Failed to load DLL: {}", error.message()));

                return ExitCode::FAILURE;
            }
        };

        if dll_handle.0.is_null() {
            displayer.failure("Failed to get a valid DLL handle");

            return ExitCode::FAILURE;
        }

        displayer.success("DLL loaded successfully in memory");

        ExitCode::SUCCESS
    }
}
