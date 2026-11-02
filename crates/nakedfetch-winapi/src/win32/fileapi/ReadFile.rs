use core::ptr;

use crate::{
    AsRawHandle, ntdll,
    ntstatus::NtStatus,
    types::IoStatusBlock,
    win32::fileapi::{CanRead, FileHandle},
};

/// # Errors
///
/// `Err` when the kernel fails to read the file correctly.
///
/// # Panics
///
/// ``assert!(read <= buf.len())``
pub fn ReadFile<A: CanRead>(
    handle: &FileHandle<A>,
    buf: &mut [u8],
    read: usize,
    offset: Option<usize>,
) -> Result<usize, NtStatus> {
    assert!(read <= buf.len());
    read_file_inner(handle.as_raw(), buf, read, offset)
}

fn read_file_inner(
    raw: *mut (),
    buf: &mut [u8],
    read: usize,
    offset: Option<usize>,
) -> Result<usize, NtStatus> {
    let mut offset_val = 0;
    let offset_ptr = offset.map_or(ptr::null_mut(), |o| {
        offset_val = o;
        &raw mut offset_val
    });

    let mut info = IoStatusBlock::default();

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
        let readed = info.information as usize;
        Ok(readed)
    } else {
        Err(ret)
    }
}
