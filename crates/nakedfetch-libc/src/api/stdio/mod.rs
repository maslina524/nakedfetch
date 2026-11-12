use core::ffi::CStr;

use crate::{errno::Errno, fcntl, sys::types::c_mode, unistd};

pub const O_RDONLY: i32 = 0o0;
pub const O_WRONLY: i32 = 0o1;
pub const O_RDWR: i32 = 0o2;
pub const O_CREAT: i32 = 0o100;
pub const O_EXCL: i32 = 0o200;
pub const O_NOCTTY: i32 = 0o400;
pub const O_TRUNC: i32 = 0o1000;
pub const O_APPEND: i32 = 0o2000;
pub const O_CLOEXEC: i32 = 0o200_0000;

pub const DEFAULT_FILE_MODE: c_mode = 0o666;

#[repr(transparent)]
#[derive(Debug)]
pub struct File {
    fd: i32,
}

impl File {
    #[must_use]
    pub const fn as_raw_fd(&self) -> i32 {
        self.fd
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = unistd::close(self.as_raw_fd());
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAccess {
    Read = O_RDONLY,
    Write = O_WRONLY,
    ReadWrite = O_RDWR,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileDispose {
    CreateNew = O_CREAT | O_EXCL,
    CreateAlways = O_CREAT | O_TRUNC,
    OpenExisting = 0,
    OpenAlways = O_CREAT,
    TruncateExisting = O_TRUNC,
}

impl FileDispose {
    #[must_use]
    pub const fn creates_file(self) -> bool {
        matches!(
            self,
            Self::CreateNew | Self::CreateAlways | Self::OpenAlways
        )
    }

    #[must_use]
    pub const fn requires_existing(self) -> bool {
        matches!(self, Self::OpenExisting | Self::TruncateExisting)
    }
}

/// # Errors
///
/// If open(2) returned an error.
pub fn fopen(path: &CStr, access: FileAccess, dispose: FileDispose) -> Result<File, Errno> {
    let flags = access as i32 | dispose as i32 | O_CLOEXEC;
    let perm = if dispose.creates_file() {
        DEFAULT_FILE_MODE
    } else {
        0
    };
    fcntl::open(path, flags, perm).map(|fd| File { fd })
}

/// # Errors
///
/// If read(2) returned an error.
pub fn fgets(buf: &mut [u8], size: usize, stream: &File) -> Result<usize, Errno> {
    unistd::read(stream.fd, buf, size)
}

#[cfg(test)]
mod tests {
    use crate::stdio::{FileAccess, FileDispose};

    #[test]
    fn open_file() {
        super::fopen(c"Cargo.toml", FileAccess::Read, FileDispose::OpenExisting)
            .expect("Failed to open file");
    }

    #[test]
    fn read_file() {
        let mut buf = [0u8; 256];
        let file = super::fopen(c"Cargo.toml", FileAccess::Read, FileDispose::OpenExisting)
            .expect("Failed to open file");
        let len = super::fgets(&mut buf, 32, &file).expect("Failed to read file");
        let slice = &buf[..len];

        let Ok(s) = str::from_utf8(slice) else {
            return;
        };
        assert!(s.starts_with("[package]"));
    }
}
