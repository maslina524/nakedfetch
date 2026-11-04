#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod imp;

#[macro_use]
mod macros;
pub mod allocator;
pub mod env;
pub mod fs;
pub mod io;
pub mod sync;

pub mod prelude;
