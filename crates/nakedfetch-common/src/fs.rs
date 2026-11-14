use alloc::{
    string::{FromUtf8Error, String},
    vec,
    vec::Vec,
};
use core::{error::Error, fmt, marker::PhantomData, str::Utf8Error};

use path::Path;

use crate::imp::{self, fs};

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
    Unknown(imp::RawError),
}

impl fmt::Display for FileCreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileCreateError {}

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
    Unknown(imp::RawError),
}

impl fmt::Display for FileWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileWriteError {}

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
    Unknown(imp::RawError),
}

impl fmt::Display for FileReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileReadError {}

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
    Unknown(imp::RawError),
}

impl fmt::Display for FileSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileSizeError {}

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
    Unknown(imp::RawError),
}

impl fmt::Display for FileExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileExistsError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileSizeReadError {
    Size(FileSizeError),
    Read(FileReadError),
}

impl fmt::Display for FileSizeReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<FileSizeError> for FileSizeReadError {
    fn from(value: FileSizeError) -> Self {
        Self::Size(value)
    }
}

impl From<FileReadError> for FileSizeReadError {
    fn from(value: FileReadError) -> Self {
        Self::Read(value)
    }
}

pub trait Access {
    fn into_file_access() -> fs::FileAccess;
}
pub trait CanRead: Access {}
pub trait CanWrite: Access {}

pub struct Read;
impl Access for Read {
    fn into_file_access() -> fs::FileAccess {
        fs::FileAccess::Read
    }
}
impl CanRead for Read {}

pub struct Write;
impl Access for Write {
    fn into_file_access() -> fs::FileAccess {
        fs::FileAccess::Write
    }
}
impl CanWrite for Write {}

pub struct All;
impl Access for All {
    fn into_file_access() -> fs::FileAccess {
        fs::FileAccess::All
    }
}
impl CanRead for All {}
impl CanWrite for All {}

#[cfg(target_os = "windows")]
pub type RawFileHandle = *mut core::ffi::c_void;
#[cfg(target_family = "unix")]
pub type RawFileHandle = i32;

pub struct File<A: Access> {
    fd: fs::FileHandle,
    _marker: PhantomData<A>,
}

impl<A: Access> File<A> {
    /// # Errors
    ///
    /// Error if the system failed to open the file correctly.
    pub fn create_new_or_rewrite<T: AsRef<Path>>(path: T) -> Result<Self, FileCreateError> {
        fs::create_or_rewrite_file(path.as_ref(), A::into_file_access()).map(|fd| Self {
            fd,
            _marker: PhantomData,
        })
    }

    /// # Errors
    ///
    /// Error if the system failed to open the file correctly.
    pub fn create_new<T: AsRef<Path>>(path: T) -> Result<Self, FileCreateError> {
        fs::create_file(path.as_ref(), A::into_file_access()).map(|fd| Self {
            fd,
            _marker: PhantomData,
        })
    }

    /// # Errors
    ///
    /// Error if the system failed to open the file correctly.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self, FileCreateError> {
        fs::open_file(path.as_ref(), A::into_file_access()).map(|fd| Self {
            fd,
            _marker: PhantomData,
        })
    }

    /// # Errors
    ///
    /// Error if the system failed to get size of file correctly.
    pub fn file_size(&self) -> Result<usize, FileSizeError> {
        fs::size_of_file(&self.fd)
    }

    #[must_use]
    pub const fn as_handle(&self) -> &fs::FileHandle {
        &self.fd
    }

    #[must_use]
    pub fn as_raw_handle(&self) -> RawFileHandle {
        imp::handle::as_raw_handle(&self.fd)
    }
}

impl<A: CanRead> File<A> {
    /// # Errors
    ///
    /// Error if the system failed to read the file correctly.
    pub fn read_part(
        &self,
        buf: &mut [u8],
        start: usize,
        count: usize,
    ) -> Result<usize, FileReadError> {
        fs::read_file(&self.fd, buf, start, count)
    }

    /// # Errors
    ///
    /// Error if the system failed to read the file correctly.
    pub fn read(&self) -> Result<Vec<u8>, FileSizeReadError> {
        let size = self.file_size()?;

        let mut buf = vec![0u8; size];
        let bytes_read = fs::read_file(&self.fd, &mut buf, 0, size)?;
        buf.truncate(bytes_read);

        Ok(buf)
    }
}

impl<A: CanWrite> File<A> {
    /// # Errors
    ///
    /// Error if the system failed to write to the file correctly.
    #[allow(
        clippy::needless_pass_by_ref_mut,
        reason = "For good API with fmt::Write"
    )]
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, FileWriteError> {
        fs::write_file(&self.fd, bytes)
    }
}

impl<A: CanWrite> fmt::Write for File<A> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes())
            .map(|_| ())
            .map_err(|_| fmt::Error)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileCreateReadError {
    Create(FileCreateError),
    Size(FileSizeError),
    Read(FileReadError),
}

impl fmt::Display for FileCreateReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<FileCreateError> for FileCreateReadError {
    fn from(value: FileCreateError) -> Self {
        Self::Create(value)
    }
}

impl From<FileSizeReadError> for FileCreateReadError {
    fn from(value: FileSizeReadError) -> Self {
        match value {
            FileSizeReadError::Size(e) => Self::Size(e),
            FileSizeReadError::Read(e) => Self::Read(e),
        }
    }
}

/// # Errors
///
/// If the system fails to open or read the file.
pub fn read<T: AsRef<Path>>(path: T) -> Result<Vec<u8>, FileCreateReadError> {
    let file = File::<Read>::open(path)?;
    file.read().map_err(Into::into)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileReadToStringError {
    Create(FileCreateError),
    Size(FileSizeError),
    Read(FileReadError),
    Utf8(FromUtf8Error),
}

impl fmt::Display for FileReadToStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<FileCreateError> for FileReadToStringError {
    fn from(value: FileCreateError) -> Self {
        Self::Create(value)
    }
}

impl From<FileSizeReadError> for FileReadToStringError {
    fn from(value: FileSizeReadError) -> Self {
        match value {
            FileSizeReadError::Size(e) => Self::Size(e),
            FileSizeReadError::Read(e) => Self::Read(e),
        }
    }
}

impl From<FromUtf8Error> for FileReadToStringError {
    fn from(value: FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

/// # Errors
///
/// If the system fails to open, read the file as UTF8.
pub fn read_to_string<T: AsRef<Path>>(path: T) -> Result<String, FileReadToStringError> {
    let file = File::<Read>::open(path)?;
    let buf = file.read()?;
    String::from_utf8(buf).map_err(Into::into)
}

/// # Errors
///
/// If the system fails to create dir.
pub fn create_dir<T: AsRef<Path>>(path: T) -> Result<(), FileCreateError> {
    fs::create_dir(path.as_ref())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CreateDirsError {
    Create(FileCreateError),
    Exists(FileExistsError),
}

impl fmt::Display for CreateDirsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<FileCreateError> for CreateDirsError {
    fn from(value: FileCreateError) -> Self {
        Self::Create(value)
    }
}

impl From<FileExistsError> for CreateDirsError {
    fn from(value: FileExistsError) -> Self {
        Self::Exists(value)
    }
}

/// # Errors
///
/// If the system fails to create dirs.
pub fn create_dirs<T: AsRef<Path>>(path: T) -> Result<(), CreateDirsError> {
    create_dirs_inner(path.as_ref())
}

fn create_dirs_inner(path: &Path) -> Result<(), CreateDirsError> {
    if !exists(path)? {
        if let Some(parent) = path.parent() {
            create_dirs(parent)?;
        }
        create_dir(path)?;
    }
    Ok(())
}

/// # Errors
///
/// If the system fails to check file exists or no.
pub fn exists<T: AsRef<Path>>(path: T) -> Result<bool, FileExistsError> {
    fs::exists(path.as_ref())
}

const PER_READ_BYTES: usize = 128;

#[derive(Debug)]
pub enum FileByLineError {
    Size(FileSizeError),
    Read(FileReadError),
    LineTooLong,
    InvalidUtf8(Utf8Error),
}

impl fmt::Display for FileByLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for FileByLineError {}

pub struct FileByLine<'f, A: CanRead> {
    file: &'f File<A>,
    pos: usize,
}

impl<'f, A: CanRead> FileByLine<'f, A> {
    #[must_use]
    pub const fn new(file: &'f File<A>) -> Self {
        Self { file, pos: 0 }
    }

    /// # Errors
    ///
    /// If it failed to read a line of the file, see [`FileByLineError`].
    pub fn next<'out>(
        &mut self,
        buf: &'out mut [u8],
    ) -> Result<Option<&'out str>, FileByLineError> {
        if buf.is_empty() {
            return Err(FileByLineError::LineTooLong);
        }

        let size = self.file.file_size().map_err(FileByLineError::Size)?;
        if self.pos >= size {
            return Ok(None);
        }

        let requested = PER_READ_BYTES.min(buf.len());
        let readed = self
            .file
            .read_part(buf, self.pos, requested)
            .map_err(FileByLineError::Read)?;
        if readed == 0 {
            return Ok(None);
        }

        let at_eof = self.pos + readed >= size;

        let mut new_line_idx = 0;
        let has_newline = loop {
            if new_line_idx >= readed {
                break false;
            }
            if buf[new_line_idx] == b'\n' {
                break true;
            }
            new_line_idx += 1;
        };

        if has_newline {
            self.pos += new_line_idx + 1;
        } else if at_eof {
            self.pos += readed;
            new_line_idx = readed;
        } else {
            return Err(FileByLineError::LineTooLong);
        }

        // Trim \r
        if new_line_idx > 0 && buf[new_line_idx - 1] == b'\r' {
            new_line_idx -= 1;
        }

        str::from_utf8(&buf[..new_line_idx])
            .map(Some)
            .map_err(FileByLineError::InvalidUtf8)
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Write as _;
    use std::fs;

    use crate::fs::{File, FileByLine, Read, Write};

    #[test]
    fn create_new_file() {
        let path = "Common-Fs-File-CreateNew-1.txt";
        {
            let _ = File::<Read>::create_new_or_rewrite(path).expect("Failed to crate new file");
            assert!(fs::exists(path).expect("fs::exits failed"));
        }

        fs::remove_file(path).expect("fs::remove_file failed");
    }

    #[test]
    fn write_bytes_file() {
        let path = "Common-Fs-File-WriteBytes-1.txt";
        {
            let mut file =
                File::<Write>::create_new_or_rewrite(path).expect("Failed to crate new file");
            assert!(fs::exists(path).expect("fs::exits failed"));

            let bytes = b"Some bytes";
            file.write_bytes(bytes).expect("Failed to write bytes");
            assert_eq!(fs::read(path).expect("fs::read failed"), bytes);
        }

        fs::remove_file(path).expect("fs::remove_file failed");
    }

    #[test]
    fn fmt_write_file() {
        let path = "Common-Fs-File-FmtWrite-1.txt";
        {
            let mut file =
                File::<Write>::create_new_or_rewrite(path).expect("Failed to crate new file");
            assert!(fs::exists(path).expect("fs::exits failed"));

            file.write_fmt(format_args!("Hello {}; 2 + 2 = {}", "World", 2 + 2))
                .expect("Failed to write bytes");
            assert_eq!(
                fs::read_to_string(path).expect("fs::read failed"),
                "Hello World; 2 + 2 = 4"
            );
        }

        fs::remove_file(path).expect("fs::remove_file failed");
    }

    #[test]
    fn read_part_file() {
        let path = "Cargo.toml";
        {
            let file = File::<Read>::open(path).expect("Failed to crate new file");

            let mut buf = [0u8; 20];
            let std = fs::read(path).expect("fs::read failed");

            file.read_part(&mut buf, 10, 20)
                .expect("Failed to read bytes");
            assert_eq!(&std[10..30], buf);

            file.read_part(&mut buf, 10, 20)
                .expect("Failed to read bytes");
            assert_eq!(&std[10..30], buf);
        }
    }

    #[test]
    fn read_and_read_to_string() {
        let path = "Cargo.toml";

        let std = fs::read(path).expect("fs::read_to_string failed");

        let bytes = super::read(path).expect("Failed to read file");
        let string = super::read_to_string(path).expect("Failed to read file or convert Utf8");

        assert_eq!(std, bytes);
        assert_eq!(std, string.as_bytes());
    }

    #[test]
    fn file_size() {
        let path = "Cargo.toml";

        let file = File::<Read>::open(path).expect("Failed to open file");
        let size = file.file_size().expect("Failed to get size");
        assert!(size > 0);
    }

    #[test]
    fn create_dir() {
        let path = "Common-Fs-File-CreateDir-1";
        super::create_dir(path).expect("Failed to create dir");

        assert!(fs::exists(path).expect("fs::exists failed"));

        fs::remove_dir(path).expect("fs::remove_dir failed");
    }

    #[test]
    fn create_dirs() {
        let path = "Common-Fs-File-CreateDir-1/2/3/4/5";
        super::create_dirs(path).expect("Failed to create dir");

        assert!(fs::exists(path).expect("fs::exists failed"));

        fs::remove_dir_all("Common-Fs-File-CreateDir-1").expect("fs::remove_dir failed");
    }

    #[test]
    fn exists() {
        assert!(super::exists("Cargo.toml").expect("Failed to check exists"));
        assert!(super::exists("src").expect("Failed to check exists"));
        assert!(!super::exists("NotExists").expect("Failed to check exists"));
    }

    #[test]
    fn read_file_by_line() {
        static LINES: [&str; 6] = [
            "First Line",
            "Second Line",
            "Line with `\\r`\r",
            "Next line is empty",
            "",
            "End Of File",
        ];

        fn save_example_file(path: &str) {
            fs::write(path, LINES.join("\n")).expect("std::fs::write failed");
        }

        fn next<'out>(
            by_line: &'out mut FileByLine<'_, Read>,
            buf: &'out mut [u8],
        ) -> Option<&'out str> {
            by_line.next(buf).expect("Failed to read line")
        }

        let path = "Common-Fs-ReadFileByLine-1";
        save_example_file(path);

        {
            let file = File::<Read>::open(path).expect("Failed to open file");
            let mut by_line = FileByLine::new(&file);
            let mut buf = [0u8; 64];

            assert_eq!(next(&mut by_line, &mut buf), Some("First Line"));
            assert_eq!(next(&mut by_line, &mut buf), Some("Second Line"));
            assert_eq!(next(&mut by_line, &mut buf), Some("Line with `\\r`"));
            assert_eq!(next(&mut by_line, &mut buf), Some("Next line is empty"));
            assert_eq!(next(&mut by_line, &mut buf), Some(""));
            assert_eq!(next(&mut by_line, &mut buf), Some("End Of File"));
            assert_eq!(next(&mut by_line, &mut buf), None);
        }

        fs::remove_file(path).expect("std::fs::remove_file failed");
    }
}
