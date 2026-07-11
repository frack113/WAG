// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    displayer::Displayer,
    metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier},
    traces::{Trace, TraceMetadata, TraceParameter, TraceParameterKind},
};
use clap::Parser;
use serde::Deserialize;
use std::{path::PathBuf, process::ExitCode, sync::LazyLock};
use uuid::Uuid;
use windows::{
    Win32::System::LibraryLoader,
    core::{HSTRING, Owned},
};

#[derive(Parser, Deserialize)]
pub struct DllLoader {
    #[clap(help = "Path to the DLL file")]
    path: PathBuf,
}

pub static METADATA: LazyLock<TraceMetadata> = LazyLock::new(|| TraceMetadata {
    identifier: TraceIdentifier::new("memory", "dll", "load"),
    name: "DLL Loader",
    requirements_summary: "local DLL file",
    attack_techniques: vec![AttackTechnique::new("1574", Some("001"))],
    sigma_identifiers: vec![SigmaIdentifier::new(
        Uuid::parse_str("2a4052f7-858e-412e-be8c-60138c8ce031").unwrap(),
    )],
    use_cases: vec!["dll sideloading".to_string()],
    parameters: &[TraceParameter {
        name: "path",
        kind: TraceParameterKind::Path,
        required: true,
        allowed_values: &[],
    }],
});

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
