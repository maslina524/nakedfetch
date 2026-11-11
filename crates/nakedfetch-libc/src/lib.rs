#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod api;
pub use api::{fcntl, stdio, sys, unistd};

pub mod errno;
pub mod raw;
