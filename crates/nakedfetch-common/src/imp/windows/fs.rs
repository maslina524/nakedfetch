use core::{error::Error, fmt};

use path::Path;
pub use winapi::win32::fileapi::{
    All as AccessAll, CanRead as AccessCanRead, CanWrite as AccessCanWrite, CreateFileError,
    FileAccess as Access, FileAttributesError, FileHandle, Read as AccessRead,
    Write as AccessWrite,
};
use winapi::{
    ntstatus::NtStatus,
    win32::{
        fileapi::{
            CreateFile, CreateOptions, FileAttributes, FileDispose, FileShare, GetFileAttributes,
            GetFileSizeEx, ReadFile, WriteFile,
        },
        winbase::{CreateDirError, CreateDirectory},
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

fn create_with_dispose<A: Access>(
    path: &Path,
    dispose: FileDispose,
) -> Result<FileHandle<A>, FileCreateError> {
    const fn map_err(err: CreateFileError) -> FileCreateError {
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

    CreateFile(
        path,
        FileShare::Read,
        dispose,
        FileAttributes::new(),
        CreateOptions::new().synchronize(),
    )
    .map_err(map_err)
}

pub fn create_or_rewrite_file<A: Access>(path: &Path) -> Result<FileHandle<A>, FileCreateError> {
    create_with_dispose(path, FileDispose::CreateAlways)
}

pub fn create_file<A: Access>(path: &Path) -> Result<FileHandle<A>, FileCreateError> {
    create_with_dispose(path, FileDispose::CreateNew)
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

pub fn write_file<A: AccessCanWrite>(
    handle: &FileHandle<A>,
    buf: &[u8],
) -> Result<usize, FileWriteError> {
    const fn map_err(nt: NtStatus) -> FileWriteError {
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

    WriteFile(handle, buf).map_err(map_err)
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

pub fn read_file<A: AccessCanRead>(
    handle: &FileHandle<A>,
    buf: &mut [u8],
    start: usize,
    count: usize,
) -> Result<usize, FileReadError> {
    const fn map_err(nt: NtStatus) -> FileReadError {
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

    if buf.len() < count {
        return Err(FileReadError::BufTooSmall);
    }
    ReadFile(handle, buf, count, Some(start)).map_err(map_err)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileSizeError {
    InvalidHandle,
    FileClosed,
    AccessDenied,
    InvalidParam,
    InvalidDeviceRequest,
    InvalidInfoClass,
    InfoLengthMismatch,
    NotImplemented,
    NotSupported,
    FileInvalid,
    NoSuchFile,
    NotEnoughMemory,
    Crc,
    IoDeviceError,
    Unknown(NtStatus),
}

impl fmt::Display for FileSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileSizeError {}

pub fn size_of_file<A: Access>(handle: &FileHandle<A>) -> Result<usize, FileSizeError> {
    const fn map_err(nt: NtStatus) -> FileSizeError {
        use super::fs::FileSizeError as FSE;

        match nt.into_inner() {
            0xC000_0008 => FSE::InvalidHandle,
            0xC000_0128 => FSE::FileClosed,
            0xC000_0022 => FSE::AccessDenied,
            0xC000_000D => FSE::InvalidParam,
            0xC000_0010 => FSE::InvalidDeviceRequest,
            0xC000_0003 => FSE::InvalidInfoClass,
            0xC000_0004 => FSE::InfoLengthMismatch,
            0xC000_0002 => FSE::NotImplemented,
            0xC000_00BB => FSE::NotSupported,
            0xC000_0098 => FSE::FileInvalid,
            0xC000_000F => FSE::NoSuchFile,
            0xC000_009A | 0xC000_0017 => FSE::NotEnoughMemory,
            0xC000_003F => FSE::Crc,
            0xC000_009D => FSE::IoDeviceError,
            _ => FSE::Unknown(nt),
        }
    }

    GetFileSizeEx(handle).map_err(map_err)
}

pub fn create_dir(path: &Path) -> Result<(), FileCreateError> {
    const fn map_err(err: CreateDirError) -> FileCreateError {
        use super::fs::FileCreateError as FCE;

        let nt = match err {
            CreateDirError::UnicodePathAllocate => return FCE::BadUnicode,
            CreateDirError::FailedToCreate(nt) => nt,
        };

        match nt.into_inner() {
            0xC000_0035 | 0xC000_00BA => FCE::AlreadyExists,
            0xC000_003A => FCE::PathNotFound,
            0xC000_0034 => FCE::NameNotFound,
            0xC000_0022 => FCE::AccessDenied,
            0xC000_0056 => FCE::AccessPending,
            0xC000_000D => FCE::InvalidParam,
            0xC000_0033 | 0xC000_003B => FCE::InvalidName,
            0xC000_007F => FCE::DiskFull,
            0xC000_0106 => FCE::NameTooLong,
            0xC000_0103 => FCE::NotADir,
            0xC000_0044 => FCE::QuotaExceeded,
            0xC000_009A | 0xC000_0017 => FCE::NoSystemResource,
            0xC000_0061 => FCE::BadPrivilege,
            _ => FCE::Unknown(nt),
        }
    }

    CreateDirectory(path, FileAttributes::new()).map_err(map_err)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileExistsError {
    BadUnicode,
    AccessDenied,
    AccessPending,
    InvalidParam,
    InvalidName,
    NotADir,
    NoSystemResource,
    BadPrivilege,
    Crc,
    IoDeviceError,
    Unknown(NtStatus),
}

impl fmt::Display for FileExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileExistsError {}

pub fn exists(path: &Path) -> Result<bool, FileExistsError> {
    const fn map_err(err: FileAttributesError) -> Result<bool, FileExistsError> {
        use super::fs::FileExistsError as FEE;

        let nt = match err {
            FileAttributesError::UnicodePathAllocate => return Err(FEE::BadUnicode),
            FileAttributesError::QueryAttributes(nt) => nt,
        };

        match nt.into_inner() {
            0xC000_0034 | 0xC000_003A | 0xC000_000F | 0xC000_00BE => Ok(false),

            0xC000_0022 => Err(FEE::AccessDenied),
            0xC000_0056 => Err(FEE::AccessPending),
            0xC000_000D => Err(FEE::InvalidParam),
            0xC000_0033 | 0xC000_003B => Err(FEE::InvalidName),
            0xC000_0103 => Err(FEE::NotADir),
            0xC000_009A | 0xC000_0017 => Err(FEE::NoSystemResource),
            0xC000_0061 => Err(FEE::BadPrivilege),
            0xC000_003F => Err(FEE::Crc),
            0xC000_009D => Err(FEE::IoDeviceError),
            _ => Err(FEE::Unknown(nt)),
        }
    }

    match GetFileAttributes(path) {
        Ok(_) => Ok(true),
        Err(e) => map_err(e),
    }
}
