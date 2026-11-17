#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod imp;

#[macro_use]
mod macros;
pub mod allocator;
pub mod env;
pub mod fs;
pub mod io;
pub mod sync;

#[cfg(target_os = "windows")]
pub mod windows;

pub mod prelude;
