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
        ReadFile, WriteFile,
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

fn create_with_dispose<A: Access>(
    path: &Path,
    dispose: FileDispose,
) -> Result<FileHandle<A>, FileCreateError> {
    CreateFile(
        path,
        FileShare::Read,
        dispose,
        FileAttributes::new(),
        CreateOptions::new().synchronize(),
    )
    .map_err(map_create_file)
}

pub fn create_file<A: Access>(path: &Path) -> Result<FileHandle<A>, FileCreateError> {
    create_with_dispose(path, FileDispose::CreateAlways)
}

pub fn open_file<A: Access>(path: &Path) -> Result<FileHandle<A>, FileCreateError> {
    create_with_dispose(path, FileDispose::OpenExisting)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileWriteError {
    InvalidHandle,
    NotEnoughMemory,
    WriteProtect,
    NotReady,
    Crc,
    WriteFault,
    AccessDenied,
    InvalidParam,
    NoData,
    DiskFull,
    Unknonw(NtStatus),
}

impl fmt::Display for FileWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileWriteError {}

const fn map_write_file(nt: NtStatus) -> FileWriteError {
    use super::fs::FileWriteError as FWE;

    match nt.into_inner() {
        0xC000_0008 => FWE::InvalidHandle,
        0xC000_009A | 0xC000_0017 => FWE::NotEnoughMemory,
        0xC000_00A2 => FWE::WriteProtect,
        0xC000_026E | 0xC000_00A3 => FWE::NotReady,
        0xC000_003F => FWE::Crc,
        0xC000_009C => FWE::WriteFault,
        0xC000_0022 => FWE::AccessDenied,
        0xC000_000D => FWE::InvalidParam,
        0xC000_00F2 | 0xC000_00AE => FWE::NoData,
        0xC000_007F => FWE::DiskFull,
        _ => FWE::Unknonw(nt),
    }
}

pub fn write_file<A: AccessCanWrite>(
    handle: &FileHandle<A>,
    buf: &[u8],
) -> Result<usize, FileWriteError> {
    WriteFile(handle, buf).map_err(map_write_file)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileReadError {
    BufTooSmall,
    InvalidHandle,
    EndOfFile,
    NotEnoughMemory,
    NotReady,
    Crc,
    ReadFault,
    AccessDenied,
    InvalidParam,
    NoData,
    Unknown(NtStatus),
}

impl fmt::Display for FileReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileReadError {}

const fn map_ntstatus(nt: NtStatus) -> FileReadError {
    use super::fs::FileReadError as FRE;

    match nt.into_inner() {
        0xC000_0008 => FRE::InvalidHandle,
        0xC000_0011 => FRE::EndOfFile,
        0xC000_009A | 0xC000_0017 => FRE::NotEnoughMemory,
        0xC000_026E | 0xC000_00A3 => FRE::NotReady,
        0xC000_003F => FRE::Crc,
        0xC000_009D => FRE::ReadFault,
        0xC000_0022 => FRE::AccessDenied,
        0xC000_000D => FRE::InvalidParam,
        0xC000_00F2 | 0xC000_00AE => FRE::NoData,
        _ => FRE::Unknown(nt),
    }
}

pub fn read_file<A: AccessCanRead>(
    handle: &FileHandle<A>,
    buf: &mut [u8],
    start: usize,
    count: usize,
) -> Result<usize, FileReadError> {
    if buf.len() < count {
        return Err(FileReadError::BufTooSmall);
    }
    ReadFile(handle, buf, count, Some(start)).map_err(map_ntstatus)
}
