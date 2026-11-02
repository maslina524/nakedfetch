use core::{error::Error, fmt};

use path::Path;
pub use winapi::win32::fileapi::{
    All as AccessAll, CanRead as AccessCanRead, CanWrite as AccessCanWrite, FileAccess as Access,
    FileHandle, Read as AccessRead, Write as AccessWrite,
};
use winapi::{
    ntstatus::NtStatus,
    win32::fileapi::{
        CreateFile, CreateFileError, CreateOptions, FileAttributes, FileDispose, FileShare,
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileCreateError {
    BadUnicode,
    PathNotFound,
    NameNotFound,
    AccessDenied,
    AccessPending,
    InvalidParam,
    InvalidName,
    DiskFull,
    AlreadyExists,
    NameTooLong,
    NotADir,
    QuotaExceeded,
    NoSystemResource,
    BadPrivilege,
    Unknown(NtStatus),
}

impl fmt::Display for FileCreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileCreateError {}

const fn map_create_file(err: CreateFileError) -> FileCreateError {
    use super::fs::FileCreateError as FCE;

    let nt = match err {
        CreateFileError::UnicodePathAllocate => return FCE::BadUnicode,
        CreateFileError::FailedToCreate(nt) => nt,
    };

    match nt.into_inner() {
        0xC000_003A => FCE::PathNotFound,
        0xC000_0034 => FCE::NameNotFound,
        0xC000_0022 => FCE::AccessDenied,
        0xC000_0056 => FCE::AccessPending,
        0xC000_000D => FCE::InvalidParam,
        0xC000_0033 => FCE::InvalidName,
        0xC000_007F => FCE::DiskFull,
        0xC000_0035 => FCE::AlreadyExists,
        0xC000_0106 => FCE::NameTooLong,
        0xC000_0103 | 0xC000_00BA => FCE::NotADir,
        0xC000_0044 => FCE::QuotaExceeded,
        0xC000_009A | 0xC000_0017 => FCE::NoSystemResource,
        0xC000_0061 => FCE::BadPrivilege,
        _ => FCE::Unknown(nt),
    }
}

pub fn create_file<A: Access>(path: &Path) -> Result<FileHandle<A>, FileCreateError> {
    CreateFile(
        path,
        FileShare::Read,
        FileDispose::CreateAlways,
        FileAttributes::new(),
        CreateOptions::new().synchronize(),
    )
    .map_err(map_create_file)
}
