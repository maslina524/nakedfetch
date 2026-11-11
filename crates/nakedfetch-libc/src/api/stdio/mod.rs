use core::ffi::CStr;

use crate::{errno::Errno, fcntl, sys::types::c_mode};

pub const O_RDONLY: i32 = 0o0;
pub const O_WRONLY: i32 = 0o1;
pub const O_RDWR: i32 = 0o2;
pub const O_CREAT: i32 = 0o100;
pub const O_EXCL: i32 = 0o200;
pub const O_NOCTTY: i32 = 0o400;
pub const O_TRUNC: i32 = 0o1000;
pub const O_APPEND: i32 = 0o2000;
pub const O_CLOEXEC: i32 = 0o200_0000;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct File {
    fd: i32,
}

impl File {
    #[must_use]
    pub const fn as_raw_fd(self) -> i32 {
        self.fd
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    Read,
    Write,
    Append,
    ReadWrite,
    WriteRead,
    AppendRead,
}

impl FileMode {
    #[must_use]
    pub const fn to_open_flags(self) -> c_mode {
        match self {
            Self::Read => O_RDONLY,
            Self::Write => O_WRONLY | O_CREAT | O_TRUNC,
            Self::Append => O_WRONLY | O_CREAT | O_APPEND,
            Self::ReadWrite => O_RDWR,
            Self::WriteRead => O_RDWR | O_CREAT | O_TRUNC,
            Self::AppendRead => O_RDWR | O_CREAT | O_APPEND,
        }
    }
}

/// # Errors
/// `Err` if the system failed to open the file.
pub fn fopen(path: &CStr, mode: FileMode) -> Result<File, Errno> {
    fcntl::open(path, mode.to_open_flags() | O_CLOEXEC, 0o666).map(|fd| File { fd })
}

#[cfg(test)]
mod tests {
    use crate::stdio::FileMode;

    #[test]
    fn open_file() {
        super::fopen(c"Cargo.toml", FileMode::Read).expect("Failed to open file");
    }
}
