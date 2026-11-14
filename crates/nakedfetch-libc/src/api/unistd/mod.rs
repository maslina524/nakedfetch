mod syscalls;
use core::{
    ffi::c_char,
    ptr,
    sync::atomic::{AtomicPtr, AtomicUsize},
};

pub use syscalls::*;

use crate::{AsRawHandle as _, stdio::File};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ConsoleHandle(i32);

impl ConsoleHandle {
    #[must_use]
    pub fn into_file(self) -> File {
        File::from_raw(self.0)
    }
}

pub const STDIN: ConsoleHandle = ConsoleHandle(0);
pub const STDOUT: ConsoleHandle = ConsoleHandle(1);
pub const STDERR: ConsoleHandle = ConsoleHandle(2);

pub static ARGC: AtomicUsize = AtomicUsize::new(0);
pub static ARGV: AtomicPtr<*const c_char> = AtomicPtr::new(ptr::null_mut());
pub static ENVIRON: AtomicPtr<*const c_char> = AtomicPtr::new(ptr::null_mut());
