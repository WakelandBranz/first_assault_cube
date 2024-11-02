// Because I am a big lazy BUM!
// This file stores all types that I might use in multiple files to centralize everything

#![allow(
    clippy::pedantic,
    clippy::grammar,
    clippy::wrong_self_convention
)]

// src/prelude.rs
pub use windows::Win32::Foundation::HANDLE;
pub use windows::Win32::System::Diagnostics::ToolHelp::{
    MODULEENTRY32,
    CREATE_TOOLHELP_SNAPSHOT_FLAGS,
};

pub type DWORD = u32;