#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg(target_family = "unix")]

extern crate alloc;

mod api;
pub use api::{fcntl, stdio, sys, unistd};

pub mod errno;
pub mod raw;

pub trait AsRawHandle {
    fn from_raw(raw: i32) -> Self;
    fn as_raw(&self) -> i32;
}
