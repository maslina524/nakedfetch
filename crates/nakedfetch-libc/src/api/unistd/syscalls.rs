use core::hint;

use crate::{errno::Errno, raw::Table, syscall};

/// # Errors
///
/// If system failed to write
pub fn write(fd: i32, buf: &[u8]) -> Result<usize, Errno> {
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let ret = unsafe { syscall!(Table::Write, fd, buf.as_ptr(), buf.len()) };
    ret.map(|v| v as usize)
}

/// # Errors
///
/// If system failed to read
pub fn read(fd: i32, buf: &mut [u8], count: usize) -> Result<usize, Errno> {
    if count > buf.len() {
        return Err(Errno::ERANGE);
    }
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let ret = unsafe { syscall!(Table::Read, fd, buf.as_ptr(), buf.len()) };
    ret.map(|v| v as usize)
}

/// # Errors
///
/// If system failed to pread
pub fn pread(fd: i32, buf: &mut [u8], count: usize, offset: usize) -> Result<usize, Errno> {
    if count > buf.len() {
        return Err(Errno::ERANGE);
    }
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let ret = unsafe { syscall!(Table::PRead64, fd, buf.as_ptr(), buf.len(), offset) };
    ret.map(|v| v as usize)
}

/// # Errors
///
/// If system failed to close fd
pub fn close(fd: i32) -> Result<(), Errno> {
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let ret = unsafe { syscall!(Table::Close, fd) };
    ret.map(|_| ())
}

pub fn exit(error_code: i32) -> ! {
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let _ = unsafe { syscall!(Table::Exit, error_code) };

    // Here we have already exited the program, if this is not the case, it
    // means exit failed with an error
    let _ = write(2, b"Failed to exit");
    loop {
        hint::spin_loop();
    }
}
