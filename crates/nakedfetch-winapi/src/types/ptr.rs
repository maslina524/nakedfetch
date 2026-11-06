use core::ffi;

#[allow(
    clippy::wildcard_imports,
    reason = "There's no point in importing them separately"
)]
use super::*;

pub type PVOID = *mut ffi::c_void;

pub type PULONG = *mut ULONG;
pub type ULONG_PTR = *mut ULONG;
pub type PLARGE_INTEGER = *mut LARGE_INTEGER;

pub type PIO_APC_ROUTINE = PVOID;
pub type PHEAP = PVOID;
pub type PPEB_LDR_DATA = &'static ffi::c_void;
pub type PSECURITY_DESCRIPTOR = PVOID;
pub type PSECURITY_QUALITY_OF_SERVICE = PVOID;
pub type PHANDLE = *mut HANDLE;

pub type PCOBJECT_ATTRIBUTES = *const OBJECT_ATTRIBUTES;
pub type PCUNICODE_STRING = *const UNICODE_STRING;

// Strings
pub type PWSTR = *mut u16;
pub type PCWSTR = *const u16;

pub type PWCH = *const u16;
