use core::ffi::CStr;

use crate::{errno::Errno, raw::Table, sys::types::c_mode, syscall};

const AT_FDCWD: i32 = -100;

/// # Errors
///
/// `Err` if system failed to openat file
pub fn openat(dirfd: i32, path: &CStr, flags: i32, mode: c_mode) -> Result<i32, Errno> {
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let ret = unsafe { syscall!(Table::OpenAt, dirfd, path.as_ptr(), flags, mode) };
    ret.map(|v| v as i32)
}

/// # Errors
///
/// `Err` if system failed to open file
pub fn open(path: &CStr, flags: i32, mode: c_mode) -> Result<i32, Errno> {
    openat(AT_FDCWD, path, flags, mode)
}
