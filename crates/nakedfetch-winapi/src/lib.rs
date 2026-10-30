#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[macro_use]
mod macros;

mod native;
mod ntdll;
mod types;