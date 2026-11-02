use crate::{
    AsRawHandle, ntdll,
    ntstatus::Ntstatus,
    types::{
        Handle, IoStatusBlock,
        class::{FileInformation, FileStandartInformation},
    },
    win32::fileapi::{FileAccess, FileHandle},
};

/// # Errors
///
/// Err when the kernel fails to correctly retrieve information about the file.
pub fn GetFileSizeEx<A: FileAccess>(handle: &FileHandle<A>) -> Result<usize, Ntstatus> {
    get_file_size_ex(handle.as_raw())
}

fn get_file_size_ex(raw: Handle) -> Result<usize, Ntstatus> {
    let mut info = FileStandartInformation::default();
    let mut io = IoStatusBlock::default();

    // SAFETY: All fields have been verified against ntdoc, safe
    let ret = unsafe {
        ntdll::NtQueryInformationFile(
            raw,
            &raw mut io,
            (&raw mut info).cast(),
            size_of::<FileStandartInformation>() as u32,
            FileInformation::FileStandardInformation,
        )
    };

    if ret.is_success() {
        Ok(info.end_of_file as usize)
    } else {
        Err(ret)
    }
}
