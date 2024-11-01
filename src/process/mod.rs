use std::mem;

use memlib::{MemoryRead, MemoryWrite};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Threading::GetProcessId;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::PROCESS_ACCESS_RIGHTS;
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;

pub struct Process {
    pid: i32,
    handle: HANDLE,
}

impl Process {
    pub fn new(pid: i32) -> Self {
        let handle = unsafe {
            OpenProcess(
                PROCESS_ACCESS_RIGHTS(PROCESS_ALL_ACCESS.0),
                false,
                pid as u32,
            )
        };

        match handle {
            Err(error) => panic!("Failed to open process: {}", error),
            Ok(handle) => Self {
                handle,
                pid,
            },
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
        if let Err(error) = status.ok() {
            log::error!("ReadProcessMemory failed: {}", error);
            return None;
        }
        Some(())
    }
}