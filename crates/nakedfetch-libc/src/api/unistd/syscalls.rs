use crate::{raw::Table, syscall};

#[must_use]
pub fn write(fd: isize, buf: &[u8]) -> isize {
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    unsafe { syscall!(Table::Write, fd, buf.as_ptr(), buf.len()) }
}
