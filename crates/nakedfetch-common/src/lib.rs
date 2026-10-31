#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// extern crate alloc;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as imp;

pub mod io;