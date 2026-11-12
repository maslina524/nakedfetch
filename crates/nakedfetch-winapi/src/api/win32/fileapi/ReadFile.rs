use core::ptr;

use crate::{
    AsRawHandle, ntdll,
    ntstatus::NtStatus,
    types::{IO_STATUS_BLOCK, PVOID},
    win32::fileapi::FileHandle,
};

/// # Errors
///
/// `Err` when the kernel fails to read the file correctly.
///
/// # Panics
///
/// ``assert!(read <= buf.len())``
pub fn ReadFile(
    handle: &FileHandle,
    buf: &mut [u8],
    read: usize,
    offset: Option<usize>,
) -> Result<usize, NtStatus> {
    assert!(read <= buf.len());
    read_file_inner(handle.as_raw(), buf, read, offset)
}

fn read_file_inner(
    raw: PVOID,
    buf: &mut [u8],
    read: usize,
    offset: Option<usize>,
) -> Result<usize, NtStatus> {
    let mut offset_val = 0;
    let offset_ptr = offset.map_or(ptr::null_mut(), |o| {
        offset_val = o;
        &raw mut offset_val
    });

    let mut info = IO_STATUS_BLOCK::default();

    // SAFETY: The parameters fully comply with docs
    let ret = unsafe {
        ntdll::NtReadFile(
            raw,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &raw mut info,
            buf.as_mut_ptr().cast(),
            read as u32,
            offset_ptr.cast(),
            ptr::null_mut(),
        )
    };
    if ret.is_success() {
        let readed = info.Information as usize;
        Ok(readed)
    } else {
        Err(ret)
    }
}
