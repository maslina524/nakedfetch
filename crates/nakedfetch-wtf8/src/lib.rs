#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod buf;
pub use buf::Wtf8Buf;

mod codepoint;
pub use codepoint::CodePointsIter;

mod utf16;
pub use utf16::Utf16Iter;
