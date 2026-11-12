use crate::{
    ntdll,
    ntstatus::NtStatus,
    types::{
        IO_STATUS_BLOCK,
        class::{FILE_INFORMATION_CLASS, FILE_STANDARD_INFORMATION},
    },
    win32::fileapi::FileHandle,
};

/// # Errors
///
/// Err when the kernel fails to correctly retrieve information about the file.
pub fn GetFileSizeEx(handle: &FileHandle) -> Result<usize, NtStatus> {
    let mut info = FILE_STANDARD_INFORMATION::default();
    let mut io = IO_STATUS_BLOCK::default();

    // SAFETY: All fields have been verified against ntdoc, safe
    let ret = unsafe {
        ntdll::NtQueryInformationFile(
            handle.0,
            &raw mut io,
            (&raw mut info).cast(),
            size_of::<FILE_STANDARD_INFORMATION>() as u32,
            FILE_INFORMATION_CLASS::FileStandardInformation,
        )
    };

    if ret.is_success() {
        Ok(info.EndOfFile as usize)
    } else {
        Err(ret)
    }
}
