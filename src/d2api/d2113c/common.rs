pub use std::ptr::null_mut;
pub use super::types::*;
pub use super::D2RVA;
pub use std::arch::asm;
pub use super::super::d2consts::*;

pub use windows_sys::{
    core::{PCSTR, PCWSTR},
    Win32::Foundation::{BOOL, HWND},
    Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS32,
};


::windows_targets::link!("ntdll.dll" "system" fn RtlImageNtHeader(Base: PVOID) -> *const IMAGE_NT_HEADERS32);
