pub use libc::stdio::{File, FileAccess};
use libc::{
    errno::Errno,
    stdio::{self, FileDispose},
    sys::stat,
    unistd,
};
use path::Path;

use crate::fs::{FileCreateError, FileExistsError, FileReadError, FileSizeError, FileWriteError};

pub type FileHandle = File;

fn create_with_dispose(
    path: &Path,
    access: FileAccess,
    dispose: FileDispose,
) -> Result<FileHandle, FileCreateError> {
    path.with_c_str(|c_str| {
        stdio::fopen(c_str, access, dispose).map_err(|e| FileCreateError::Unknown(e))
    })
}

pub fn create_or_rewrite_file(
    path: &Path,
    access: FileAccess,
) -> Result<FileHandle, FileCreateError> {
    create_with_dispose(path, access, FileDispose::CreateAlways)
}

pub fn create_file(path: &Path, access: FileAccess) -> Result<FileHandle, FileCreateError> {
    create_with_dispose(path, access, FileDispose::CreateNew)
}

pub fn open_file(path: &Path, access: FileAccess) -> Result<FileHandle, FileCreateError> {
    create_with_dispose(path, access, FileDispose::OpenExisting)
}

pub fn write_file(handle: &FileHandle, buf: &[u8]) -> Result<usize, FileWriteError> {
    unistd::write(handle.as_raw_fd(), buf).map_err(|e| FileWriteError::Unknown(e))
}

pub fn read_file(
    handle: &FileHandle,
    buf: &mut [u8],
    start: usize,
    count: usize,
) -> Result<usize, FileReadError> {
    if buf.len() < count {
        return Err(FileReadError::BufTooSmall);
    }
    unistd::pread(handle.as_raw_fd(), buf, count, start).map_err(|e| FileReadError::Unknown(e))
}

pub fn size_of_file(handle: &FileHandle) -> Result<usize, FileSizeError> {
    stat::fstat(handle.as_raw_fd())
        .map(|s| s.st_size as usize)
        .map_err(|e| FileSizeError::Unknown(e))
}

pub fn create_dir(path: &Path) -> Result<(), FileCreateError> {
    path.with_c_str(|c_str| stat::mkdir(c_str, 0o755).map_err(|e| FileCreateError::Unknown(e)))
}

pub fn exists(path: &Path) -> Result<bool, FileExistsError> {
    path.with_c_str(|c_str| match stat::stat(c_str) {
        Ok(_) => Ok(true),
        Err(Errno::ENOENT) => Ok(false),
        Err(e) => Err(FileExistsError::Unknown(e)),
    })
}
