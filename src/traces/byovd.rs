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
    Win32::System::Services::{
        CreateServiceW, OpenSCManagerW, SC_MANAGER_ALL_ACCESS, SC_MANAGER_CREATE_SERVICE,
        SERVICE_AUTO_START, SERVICE_ERROR_IGNORE, SERVICE_KERNEL_DRIVER, StartServiceW,
    },
    core::{HSTRING, Owned, PCWSTR},
};

#[derive(Parser, Deserialize)]
pub struct Byovd {
    #[clap(required = true, help = "Path to the driver")]
    driver: PathBuf,
    #[clap(required = true, help = "Name of the driver")]
    name: String,
    #[clap(required = true, help = "Description of the driver")]
    description: String,
}

pub static METADATA: LazyLock<TraceMetadata> = LazyLock::new(|| TraceMetadata {
    identifier: TraceIdentifier::new("driver", "byovd", "load"),
    name: "Bring Your Own Vulnerable Driver",
    requirements_summary: "administrator, kernel driver loading",
    attack_techniques: vec![AttackTechnique::new("1068", None::<&str>)],
    sigma_identifiers: vec![
        "b5d44a2e-31c9-4e4f-8a4f-cc18633f2146"
            .parse::<SigmaIdentifier>()
            .unwrap(),
    ],
    use_cases: vec!["driver loading".to_string()],
    decode_parameters: decode_parameters::<Byovd>,
});

impl Trace for Byovd {
    fn run(&self) -> ExitCode {
        let service_manager = match unsafe {
            OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CREATE_SERVICE)
        } {
            Ok(handle) => unsafe { Owned::new(handle) },
            Err(_) => return ExitCode::FAILURE,
        };

        let service = match unsafe {
            CreateServiceW(
                *service_manager,
                &HSTRING::from(self.name.as_str()),
                &HSTRING::from(self.description.as_str()),
                SC_MANAGER_ALL_ACCESS,
                SERVICE_KERNEL_DRIVER,
                SERVICE_AUTO_START,
                SERVICE_ERROR_IGNORE,
                &HSTRING::from(self.driver.to_str().unwrap()),
                PCWSTR::null(),
                None,
                PCWSTR::null(),
                PCWSTR::null(),
                PCWSTR::null(),
            )
        } {
            Ok(handle) => unsafe { Owned::new(handle) },
            Err(_) => return ExitCode::FAILURE,
        };

        if unsafe { StartServiceW(*service, None) }.is_err() {
            return ExitCode::FAILURE;
        }

        ExitCode::SUCCESS
    }
}
