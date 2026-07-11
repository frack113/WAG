// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
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
    sigma_identifiers: vec![
        "2a4052f7-858e-412e-be8c-60138c8ce031"
            .parse::<SigmaIdentifier>()
            .unwrap(),
    ],
    use_cases: vec!["dll sideloading".to_string()],
    decode_parameters: decode_parameters::<DllLoader>,
});

impl Trace for DllLoader {
    fn run(&self) -> ExitCode {
        if !self.path.exists() || !self.path.is_file() {
            return ExitCode::FAILURE;
        }

        let dll_handle = match unsafe {
            LibraryLoader::LoadLibraryW(&HSTRING::from(self.path.to_str().unwrap()))
        } {
            Ok(handle) => unsafe { Owned::new(handle) },
            Err(_) => return ExitCode::FAILURE,
        };

        if dll_handle.0.is_null() {
            return ExitCode::FAILURE;
        }

        ExitCode::SUCCESS
    }
}
