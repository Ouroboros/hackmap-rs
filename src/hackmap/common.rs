pub use std::collections::HashMap;
pub use std::ptr::{addr_of, addr_of_mut};
pub use std::rc::Rc;
pub use std::cell::RefCell;
pub use crate::d2api::{*, d2113c::*};
pub use std::arch::global_asm;
pub use std::{iter::Once, marker::PhantomData, os::raw::c_void, ptr::{null, null_mut}, sync::OnceLock};
pub use ml::hooker::{err::HookError, x86::*};

pub use windows_sys::{
    core::{PCSTR, PCWSTR, PWSTR},
    Win32::{
        UI::WindowsAndMessaging::{WM_KEYDOWN, MB_OK},
        UI::Input::KeyboardAndMouse::{GetKeyState, VK_OEM_PLUS, VK_OEM_MINUS, VK_SHIFT, VK_CONTROL},

        Foundation::{HWND, BOOL, FALSE, TRUE, NTSTATUS, UNICODE_STRING},

        System::{
            WindowsProgramming::RtlInitUnicodeString,
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
            Diagnostics::Debug::IMAGE_NT_HEADERS32,
        },
    },
};

::windows_targets::link!("ntdll.dll" "system" fn RtlImageNtHeader(Base: PVOID) -> *const IMAGE_NT_HEADERS32);
