#![allow(
    clippy::upper_case_acronyms,
    non_camel_case_types,
    non_snake_case,
    reason = "Windows style"
)]

pub mod chars;
pub use chars::*;

pub mod class;
pub use class::*;

pub mod prim;
pub use prim::*;

pub mod ptr;
pub use ptr::*;

mod bool;
pub use bool::*;

mod kernel;
pub use kernel::*;

mod bitfields;
pub use bitfields::*;
