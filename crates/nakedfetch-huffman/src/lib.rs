#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod stream;
mod tree;

pub use stream::Stream;
pub use tree::{HuffmanTree, decode_symb};
