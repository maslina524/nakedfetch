use crate::{types::ptr::PVOID, win32::console::ConsoleHandle};

pub type SHORT = i16;
pub type USHORT = u16;

pub type LONG = i32;
pub type ULONG = u32;

pub type LONGLONG = i64;
pub type ULONGLONG = u64;

pub type LARGE_INTEGER = i64;

pub type VOID = core::ffi::c_void;
pub type SIZE_T = usize;
pub type ACCESS_MASK = ULONG;

pub type HANDLE = PVOID;
pub type CONSOLE_HANDLE = ConsoleHandle;
