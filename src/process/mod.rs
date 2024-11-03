mod process;
use process::*;

use crate::utils;

use std::mem;
use std::ptr::null_mut;
use sysinfo::System;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Threading::GetProcessId;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::PROCESS_ACCESS_RIGHTS;
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;

#[derive(Clone, Debug)]
pub struct Process {
    name: String,
    pid: u32,
    handle: HANDLE,
    pub(crate) base_address: *mut core::ffi::c_void,
}

impl Process {
    pub fn new(process_name: impl ToString + std::fmt::Display) -> Self {
        let name = process_name.to_string();

        let pid = get_pid_by_name(&name)
            .unwrap_or_else(|| panic!("Could not get pid!")) as u32;

        log::debug!("Got pid! - {}", &pid);

        let handle = unsafe {
            match open_process_handle(pid) {
                Ok(handle) => handle,
                Err(error) => {
                    panic!("Failed to open handle for {}. Error: {}", name, error)
                }
            }
        };

        log::debug!("Got handle! - {:?}", &handle);

        let base_address = unsafe {
            match get_mod_base(pid, &name) {
                Ok(mod_base) => {
                    if mod_base.is_null() {
                        panic!("Could not find module base for {}", name);
                    }
                    mod_base  // Return the base address if not null
                },
                Err(error) => {
                    panic!("Failed to get module base for {}. Error: {}", name, error)
                },
            }
        };

        log::debug!("Got base address! - {:?}", base_address);

        Self {
            name,
            pid,
            handle,
            base_address
        }
    }

    /// Generic wrapper that uses try_read_bytes_into under the hood
    pub fn read<T>(&self, address: u32) -> Option<T>
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

    pub fn write<T>(&self, address: u32, value: T) -> Option<()>
    where T: Copy {
        // Convert the value into a byte slice
        let buffer = unsafe {
            std::slice::from_raw_parts(
                // Convert &T to *const T (raw pointer) then to *const u8 (byte pointer)
                &value as *const T as *const u8,
                // Get the size of T in bytes
                std::mem::size_of::<T>()
            )
        };

        self.try_write_bytes(address, buffer)
    }

    // Original function that does the actual reading
    fn try_read_bytes_into(&self, address: u32, buffer: &mut [u8]) -> Option<()> {
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

    fn try_write_bytes(&self, address: u32, buffer: &[u8]) -> Option<()> {
        if buffer.len() == 0 {
            return Some(());
        }

        let status = unsafe {
            WriteProcessMemory(
                self.handle,
                address as _,
                buffer.as_ptr() as _,
                mem::size_of_val(buffer) as _,
                None,
            )
        };

        match status {
            Ok(_) => Some(()),
            Err(error) => {
                log::error!("WriteProcessMemory failed: {}", error);
                None
            }
        }
    }
}