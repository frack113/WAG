//
// Artefact generator
//

// Windows API
use windows::core::PCWSTR;
use windows::Win32::Security::SC_HANDLE;
use windows::Win32::System::Services::{
    ControlService, CreateServiceW, DeleteService, OpenSCManagerW, StartServiceW,
    ENUM_SERVICE_TYPE, SC_MANAGER_ALL_ACCESS, SERVICE_CONTROL_STOP, SERVICE_ERROR,
    SERVICE_START_TYPE, SERVICE_STATUS,
};

// Some others
use std::{thread, time};

pub fn create_driver_service(name: String, details: String, path: String) -> bool {
    println!("Open the service manager");
    let scmanager: SC_HANDLE =
        unsafe { OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_ALL_ACCESS) }
            .expect("Sc Manager open failure");

    let mut service_name: Vec<u16> = name.encode_utf16().collect();
    service_name.push(0);
    let mut service_display: Vec<u16> = details.encode_utf16().collect();
    service_display.push(0);
    let mut service_path: Vec<u16> = path.encode_utf16().collect();
    service_path.push(0);

    println!("Create the service manager");

    let service_handle: SC_HANDLE = match unsafe {
        CreateServiceW(
            scmanager,
            PCWSTR::from_raw(service_name.as_ptr()),
            PCWSTR::from_raw(service_display.as_ptr()),
            0xF003F,
            ENUM_SERVICE_TYPE(1),
            SERVICE_START_TYPE(2),
            SERVICE_ERROR(0),
            PCWSTR::from_raw(service_path.as_ptr()),
            PCWSTR::null(),
            None,
            PCWSTR::null(),
            PCWSTR::null(),
            PCWSTR::null(),
        )
    } {
        Ok(value) => value,
        Err(_) => {
            println!("Service creation failure");
            return false;
        }
    };

    println!("Start Service ");

    match unsafe { StartServiceW(service_handle, None) } {
        Ok(_) => {
            println!("Wait a little");
            let sleep_duration: time::Duration = time::Duration::from_millis(2000);
            thread::sleep(sleep_duration);
            let mut service_status: SERVICE_STATUS = unsafe { std::mem::zeroed() };
            println!("Stop Service");
            let _result_stop = unsafe {
                ControlService(service_handle, SERVICE_CONTROL_STOP, &mut service_status)
            };
        }
        Err(value) => {
            // let error_code = unsafe { GetLastError() };
            println!("Service Start failure with code : {:#06x}", value.code().0);
        }
    };

    match unsafe { DeleteService(service_handle) } {
        Ok(_) => {
            println!("Service remove succeed");
            return true;
        }
        Err(value) => {
            println!("Service remove failure with code : {:#06x}", value.code().0);
            return false;
        }
    }
}
