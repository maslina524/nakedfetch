use crate::{errno::Errno, raw::Table, syscall};

/// # Errors
///
/// `Err` if system failed to write
pub fn write(fd: isize, buf: &[u8]) -> Result<usize, Errno> {
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let ret = unsafe { syscall!(Table::Write, fd, buf.as_ptr(), buf.len()) };
    ret.map(|v| v as usize)
}
