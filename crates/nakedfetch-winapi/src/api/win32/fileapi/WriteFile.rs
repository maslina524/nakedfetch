use core::ptr;

use crate::{
    AsRawHandle as _, ntdll, ntstatus::NtStatus, types::IO_STATUS_BLOCK, win32::fileapi::FileHandle,
};

/// # Errors
///
/// Mostly it always returns ``Ok()`` with the number of bytes written
/// in it; situations where ``Err()`` is returned are kernel failures,
/// for example the disk is full.
pub fn WriteFile(handle: &FileHandle, buf: &[u8]) -> Result<usize, NtStatus> {
    let mut info = IO_STATUS_BLOCK::default();

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

    if status.is_success() {
        let written = info.Information as usize;
        Ok(written)
    } else {
        Err(status)
    }
}
