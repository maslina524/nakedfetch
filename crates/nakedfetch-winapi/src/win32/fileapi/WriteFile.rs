use core::ptr;

use crate::{
    AsRawHandle as _, ntdll,
    types::IoStatusBlock,
    win32::fileapi::{CanWrite, FileHandle},
};

/// # Errors
///
/// Mostly it always returns ``Ok()`` with the number of bytes written
/// in it; situations where ``Err()`` is returned are kernel failures,
/// for example the disk is full.
pub fn WriteFile<T: CanWrite>(handle: &FileHandle<T>, buf: &[u8]) -> Result<usize, i32> {
    let mut info = IoStatusBlock::default();

    // SAFETY: The descriptor, buffer, and its length are always valid
    let status = unsafe {
        ntdll::NtWriteFile(
            handle.as_raw(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &raw mut info,
            buf.as_ptr().cast_mut().cast(),
            buf.len() as u32,
            ptr::null_mut(),
            ptr::null_mut(),
        )
    };

    if status >= 0 {
        let written = info.union.pointer;
        Ok(written as usize)
    } else {
        Err(status)
    }
}
