#![allow(clippy::similar_names, reason = "C names")]

mod syscalls;
use core::{
    ffi::c_char,
    mem::ManuallyDrop,
    ptr,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};

pub use syscalls::*;

use crate::{AsRawHandle as _, stdio::File};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ConsoleHandle(i32);

impl ConsoleHandle {
    #[must_use]
    pub fn into_file(self) -> ManuallyDrop<File> {
        ManuallyDrop::new(File::from_raw(self.0))
    }
}

pub const STDIN: ConsoleHandle = ConsoleHandle(0);
pub const STDOUT: ConsoleHandle = ConsoleHandle(1);
pub const STDERR: ConsoleHandle = ConsoleHandle(2);

static ARGC: AtomicUsize = AtomicUsize::new(0);
static ARGV: AtomicPtr<*const c_char> = AtomicPtr::new(ptr::null_mut());
static ENVP: AtomicPtr<*const c_char> = AtomicPtr::new(ptr::null_mut());

fn really_init(argc: usize, argv: *const *const u8, envp: *const *const u8) {
    ARGC.store(argc, Ordering::Relaxed);
    ARGV.store(argv as *mut _, Ordering::Relaxed);
    ENVP.store(envp as *mut _, Ordering::Relaxed);
}

/// glibc passes argc, argv, and envp to functions in ``.init_array``, as a
/// non-standard extension
#[cfg(any(all(target_os = "linux", target_env = "gnu"), target_os = "android"))]
#[used]
#[unsafe(link_section = ".init_array.00099")]
static ARGV_INIT_ARRAY: extern "C" fn(core::ffi::c_int, *const *const u8, *const *const u8) = {
    extern "C" fn init_wrapper(
        argc: core::ffi::c_int,
        argv: *const *const u8,
        envp: *const *const u8,
    ) {
        really_init(argc as usize, argv, envp);
    }
    init_wrapper
};

pub fn argc() -> usize {
    ARGC.load(Ordering::Relaxed)
}

pub fn argv() -> *const *const c_char {
    ARGV.load(Ordering::Relaxed).cast()
}

pub fn envp() -> *const *const c_char {
    ENVP.load(Ordering::Relaxed).cast()
}
