#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod lexer;
mod parser;

pub mod value;

pub use value::Value;

pub mod map;
pub use map::Map;
