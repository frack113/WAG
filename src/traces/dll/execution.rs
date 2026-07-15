// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::traces::{TraceError, dll::parameters::DllParameters};
use windows::{
    Win32::System::LibraryLoader,
    core::{HSTRING, Owned},
};

pub(super) fn execute(parameters: DllParameters) -> Result<(), TraceError> {
    let _dll_handle = unsafe {
        Owned::new(LibraryLoader::LoadLibraryW(&HSTRING::from(
            parameters.path.as_os_str(),
        ))?)
    };

    Ok(())
}
