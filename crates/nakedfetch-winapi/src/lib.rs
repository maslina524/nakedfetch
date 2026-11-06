#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

use crate::types::HANDLE;

#[macro_use]
mod macros;
mod native;

pub mod kernel32;
pub mod ntdll;
pub mod ntstatus;
pub mod types;
pub mod unicode;
pub mod win32;

pub trait AsRawHandle {
    fn from_raw(raw: HANDLE) -> Self;
    fn as_raw(&self) -> HANDLE;
}
