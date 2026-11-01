#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

use crate::types::Handle;

#[macro_use]
mod macros;
mod native;
mod ntstatus;

pub mod kernel32;
pub mod ntdll;
pub mod types;
pub mod unicode;
pub mod win32;

pub trait AsRawHandle {
    fn from_raw(raw: Handle) -> Self;
    fn as_raw(&self) -> Handle;
}
