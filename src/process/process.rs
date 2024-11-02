use std::mem;

use sysinfo::System;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::PROCESS_ACCESS_RIGHTS;
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;

use crate::types::*;

/// Gets pid of process by name (case-insensitive!)
pub(super) fn get_pid_by_name(process_name: &str) -> Option<u32> {
    let s = System::new_all();
    let lower_name = process_name.to_ascii_lowercase();

    for (pid, process) in s.processes() {
        let process_name = process.name().to_string_lossy().to_ascii_lowercase();
        if process_name == lower_name {
            return Some(pid.as_u32());
        }
    }

    None
}

pub(super) unsafe fn open_process_handle(pid: u32) -> Result<HANDLE, DWORD> {
    let handle = OpenProcess(
        PROCESS_ACCESS_RIGHTS(PROCESS_ALL_ACCESS.0),
        false,
        pid,
    );

    match handle {
        Ok(handle) => Ok(handle),
        Err(error) => {
            // Get the error code from Windows
            let error_code = error.code().0 as u32;
            Err(error_code)
        }
    }
}

