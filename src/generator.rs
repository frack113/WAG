//
// Artefact generator
//

// Windows API
use winapi::um::winbase::CreateNamedPipeA;
use winapi::um::winbase::{PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::*;
use std::ptr::null_mut;

use winapi::um::winsvc::{ControlService, DeleteService, OpenSCManagerW, StartServiceW, CreateServiceW,SC_MANAGER_ALL_ACCESS,SERVICE_CONTROL_STOP};
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::minwindef::DWORD;
use widestring::U16CString;




use winapi::um::winsvc::*;



// Some others
use std::{thread, time};

// For regex to string
use regex_generate::{DEFAULT_MAX_REPEAT, Generator};

pub fn create_name_pipe(name:&String,wait:u64) {
    let full_malware_pipe = format!("\\\\.\\pipe\\{}\0",name);
    let pipe_name : LPCSTR = full_malware_pipe.as_ptr() as *const i8;
    let server_pipe : HANDLE = unsafe {CreateNamedPipeA(pipe_name,PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE,1,2048,2048,0,null_mut())};
    let sleep_duration = time::Duration::from_millis(wait);
    thread::sleep(sleep_duration);
    let _res_server_pipe = unsafe { CloseHandle(server_pipe) };
}

pub fn regex_to_string(name:&String) -> String {
    let mut gen = Generator::new(name, rand::thread_rng(), DEFAULT_MAX_REPEAT).unwrap();
    let mut buffer = vec![];
    gen.generate(&mut buffer).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    return output;
}

fn open_sc_manager(desired_access: DWORD) -> Result<*mut SC_HANDLE__, DWORD> {
    let sc_manager_handle = unsafe { OpenSCManagerW(null_mut(), null_mut(), desired_access) };
    if sc_manager_handle.is_null() {
        Err(unsafe { GetLastError() })
    } else {
        Ok(sc_manager_handle)
    }
}

pub fn create_driver_service(name:String,details:String,path:String) -> bool {
    println!("Open the service manager");
    let scmanager = open_sc_manager(SC_MANAGER_ALL_ACCESS).expect("Sc Manager open failure");


    let service_name = U16CString::from_str(name).unwrap();
    let service_display = U16CString::from_str(details).unwrap();
    let service_path = U16CString::from_str(path).unwrap();

    println!("Create the service manager");
    let service_handle= unsafe { CreateServiceW(scmanager,
        service_name.as_ptr(),
        service_display.as_ptr(),
        0xF003F,
        1,
        2,
        0,
        service_path.as_ptr(),
        std::ptr::null(),
        std::ptr::null_mut(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null()
    ) };

    if service_handle.is_null() {
        println!("Service creation failure");
        return false
    }

    println!("Start Service ");
    let result = unsafe { StartServiceW(service_handle, 0, null_mut()) };
    if result == 0 {
        let error_code = unsafe { GetLastError() };
        println!("Service Start failure with code : {:#06x}",error_code);
    } else {
        println!("Wait a little");
        let sleep_duration = time::Duration::from_millis(2000);//2s
        thread::sleep(sleep_duration); 
        let mut service_status = unsafe { std::mem::zeroed() };
        println!("Stop Service");
        let _result_stop = unsafe { ControlService(service_handle,SERVICE_CONTROL_STOP,&mut service_status,)};
    }

    let result_delete = unsafe { DeleteService(service_handle) };

    if result_delete == 0 {
        let error_code = unsafe { GetLastError() };
        println!("Service remove failure with code : {:#06x}",error_code);
        return false
    } else {
        println!("Service remove succeed");
        return true
    }

}
