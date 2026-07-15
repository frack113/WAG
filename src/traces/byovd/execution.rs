// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::traces::{TraceError, byovd::parameters::ByovdParameters};
use windows::{
    Win32::System::Services::{
        CreateServiceW, OpenSCManagerW, SC_MANAGER_CREATE_SERVICE, SERVICE_AUTO_START,
        SERVICE_ERROR_IGNORE, SERVICE_KERNEL_DRIVER, SERVICE_START, StartServiceW,
    },
    core::{HSTRING, Owned, PCWSTR},
};

pub(super) fn execute(parameters: ByovdParameters) -> Result<(), TraceError> {
    let service_manager = unsafe {
        Owned::new(OpenSCManagerW(
            PCWSTR::null(),
            PCWSTR::null(),
            SC_MANAGER_CREATE_SERVICE,
        )?)
    };

    let service = unsafe {
        Owned::new(CreateServiceW(
            *service_manager,
            &HSTRING::from(parameters.name),
            &HSTRING::from(parameters.display_name),
            SERVICE_START,
            SERVICE_KERNEL_DRIVER,
            SERVICE_AUTO_START,
            SERVICE_ERROR_IGNORE,
            &HSTRING::from(parameters.driver.as_os_str()),
            PCWSTR::null(),
            None,
            PCWSTR::null(),
            PCWSTR::null(),
            PCWSTR::null(),
        )?)
    };

    unsafe { StartServiceW(*service, None)? };

    Ok(())
}
