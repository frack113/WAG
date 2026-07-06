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

pub const METADATA: TraceMetadata = TraceMetadata {
    identifier: "driver.byovd.load",
    name: "Bring Your Own Vulnerable Driver",
    requirements_summary: "administrator, kernel driver loading",
    attack_techniques: &["T1068"],
    sigma_targets: &["loldrivers"],
    use_cases: &["driver loading"],
};

impl Trace for Byovd {
    fn run(&self) -> ExitCode {
        let mut displayer: Displayer = Displayer::new();
        displayer.loading("Creating the service manager");

        let service_manager = match unsafe {
            OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CREATE_SERVICE)
        } {
            Ok(handle) => unsafe { Owned::new(handle) },
            Err(error) => {
                displayer.failure(
                    format!("Failed to create the service manager: {}", error.message()).as_str(),
                );

                return ExitCode::FAILURE;
            }
        };

        displayer.success("The service manager is created");
        displayer.loading("Creating the driver service");

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
            Err(error) => {
                displayer.failure(
                    format!("Failed to create the driver service: {}", error.message()).as_str(),
                );

                return ExitCode::FAILURE;
            }
        };

        displayer.success("The driver service is created");
        displayer.loading("Starting the driver service");

        if let Err(error) = unsafe { StartServiceW(*service, None) } {
            displayer.failure(
                format!("Failed to start the driver service: {}", error.message()).as_str(),
            );

            return ExitCode::FAILURE;
        }

        displayer.success("The driver service is started");

        ExitCode::SUCCESS
    }
}
