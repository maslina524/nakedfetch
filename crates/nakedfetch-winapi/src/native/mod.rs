#![allow(
    clippy::upper_case_acronyms,
    non_camel_case_types,
    reason = "Windows style"
)]

use crate::{
    native::rtl::RtlUserProcessParameters,
    types::{Boolean, CurDir, Handle, RawUnicodeString, primitives as prim, ptr},
    win32::console::ConsoleHandle,
};

pub mod peb;
pub mod rtl;
pub mod shared; // kuser_shared_data

pub type BOOLEAN = Boolean;
pub type HANDLE = Handle;
pub type PVOID = ptr::VoidPtr;
pub type ULONG = prim::ULong;
pub type PPEB_LDR_DATA = &'static core::ffi::c_void;
pub type PHEAP = PVOID;
pub type PRTL_USER_PROCESS_PARAMETERS = &'static RtlUserProcessParameters;
pub type UNICODE_STRING = RawUnicodeString;
pub type CURDIR = CurDir;
pub type CONSOLE_HANDLE = ConsoleHandle;
