#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg(target_os = "windows")]

extern crate alloc;

use crate::types::HANDLE;

#[macro_use]
mod macros;

mod api;
pub use api::win32;

pub mod kernel32;
pub mod ntdll;
pub mod ntstatus;
pub mod types;
pub mod unicode;

pub trait AsRawHandle {
    fn from_raw(raw: HANDLE) -> Self;
    fn as_raw(&self) -> HANDLE;
}
