mod process;
use process::*;

use crate::utils;

use std::mem;

use memlib::{MemoryRead, MemoryWrite};
use sysinfo::System;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Threading::GetProcessId;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::PROCESS_ACCESS_RIGHTS;
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;

#[derive(Debug)]
pub struct Process {
    name: String,
    pid: u32,
    handle: HANDLE,
    base_address: u64,
}

impl Process {
    pub fn new(process_name: impl ToString + std::fmt::Display) -> Self {
        let name = process_name.to_string();

        log::debug!("Finding pid of process name '{}'", &name);

        let pid = get_pid_by_name(&name)
            .unwrap_or_else(|| panic!("Could not get pid!")) as u32;

        log::debug!("Got pid! - {}", &pid);



        let handle = unsafe {
            open_process_handle(pid)
        };

        match handle {
            Err(error) => panic!("Failed to open process - Error code: {}", error),
            Ok(handle) => {
                log::debug!("Got handle! - {:?}", &handle);

                Self {
                    name: name.to_string(),
                    handle,
                    pid,
                    base_address: 123,
                }
            }
        }
    }

    // Generic wrapper that uses try_read_bytes_into under the hood
    pub fn read<T>(&self, address: u64) -> Option<T>
    where T: Default + Copy {
        // Create a default value of type T
        let mut buffer = T::default();  // Example: if T is u32, this is 0u32

        // Convert the T into a mutable byte slice
        let buffer_slice = unsafe {
            std::slice::from_raw_parts_mut(
                // Convert &mut T to *mut T (raw pointer) then to *mut u8 (byte pointer)
                &mut buffer as *mut T as *mut u8,
                // Get the size of T in bytes (e.g., u32 is 4 bytes)
                std::mem::size_of::<T>()
            )
        };

        self.try_read_bytes_into(address, buffer_slice)?;
        Some(buffer)
    }

    // Original function that does the actual reading
    fn try_read_bytes_into(&self, address: u64, buffer: &mut [u8]) -> Option<()> {
        if buffer.len() == 0 {
            return Some(());
        }
        let status = unsafe {
            ReadProcessMemory(
                self.handle,
                address as _,
                buffer.as_mut_ptr() as _,
                mem::size_of_val(buffer) as _,
                None,
            )
        };

        match status {
            Ok(_) => Some(()),
            Err(error) => {
                log::error!("ReadProcessMemory failed: {}", error);
                None
            }
        }
    }


}