use alloc::{
    string::{FromUtf8Error, String},
    vec,
    vec::Vec,
};
use core::fmt;

use path::Path;

pub use crate::imp::fs::{
    AccessAll, AccessCanRead, AccessCanWrite, AccessRead, AccessWrite, FileCreateError,
    FileExistsError, FileHandle, FileReadError, FileSizeError,
};
use crate::{imp::fs, windows::handle};

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

pub struct File<A: fs::Access>(FileHandle<A>);

impl<A: fs::Access> File<A> {
    /// # Errors
    ///
    /// Error if the system failed to open the file correctly.
    pub fn create_new_or_rewrite<T: AsRef<Path>>(path: T) -> Result<Self, FileCreateError> {
        fs::create_or_rewrite_file(path.as_ref()).map(Self)
    }

    /// # Errors
    ///
    /// Error if the system failed to open the file correctly.
    pub fn create_new<T: AsRef<Path>>(path: T) -> Result<Self, FileCreateError> {
        fs::create_file(path.as_ref()).map(Self)
    }

    /// # Errors
    ///
    /// Error if the system failed to open the file correctly.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self, FileCreateError> {
        fs::open_file(path.as_ref()).map(Self)
    }

    /// # Errors
    ///
    /// Error if the system failed to get size of file correctly.
    pub fn file_size(&self) -> Result<usize, FileSizeError> {
        fs::size_of_file(&self.0)
    }

    #[must_use]
    pub const fn as_handle(&self) -> &FileHandle<A> {
        &self.0
    }

    #[must_use]
    pub fn as_raw_handle(&self) -> *mut () {
        handle::as_raw_handle(&self.0)
    }
}

impl<A: AccessCanRead> File<A> {
    /// # Errors
    ///
    /// Error if the system failed to read the file correctly.
    pub fn read_part(
        &self,
        buf: &mut [u8],
        start: usize,
        count: usize,
    ) -> Result<usize, FileReadError> {
        fs::read_file(&self.0, buf, start, count)
    }

    /// # Errors
    ///
    /// Error if the system failed to read the file correctly.
    pub fn read(&self) -> Result<Vec<u8>, FileSizeReadError> {
        let size = self.file_size()?;

        let mut buf = vec![0u8; size];
        let bytes_read = fs::read_file(&self.0, &mut buf, 0, size)?;
        buf.truncate(bytes_read);

        Ok(buf)
    }
}

impl<A: AccessCanWrite> File<A> {
    /// # Errors
    ///
    /// Error if the system failed to write to the file correctly.
    #[allow(
        clippy::needless_pass_by_ref_mut,
        reason = "For good API with fmt::Write"
    )]
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, fs::FileWriteError> {
        fs::write_file(&self.0, bytes)
    }
}

impl<A: AccessCanWrite> fmt::Write for File<A> {
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
/// `Err` if the system fails to open or read the file.
pub fn read<T: AsRef<Path>>(path: T) -> Result<Vec<u8>, FileCreateReadError> {
    let file = File::<AccessRead>::open(path)?;
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
/// `Err` if the system fails to open, read the file as UTF8.
pub fn read_to_string<T: AsRef<Path>>(path: T) -> Result<String, FileReadToStringError> {
    let file = File::<AccessRead>::open(path)?;
    let buf = file.read()?;
    String::from_utf8(buf).map_err(Into::into)
}

/// # Errors
///
/// `Err` if the system fails to create dir.
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
/// `Err` if the system fails to create dirs.
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
/// `Err` if the system fails to check file exists or no.
pub fn exists<T: AsRef<Path>>(path: T) -> Result<bool, FileExistsError> {
    fs::exists(path.as_ref())
}

#[cfg(test)]
mod tests {
    use core::fmt::Write as _;
    use std::fs;

    use crate::fs::{AccessRead, AccessWrite, File};

    #[test]
    fn create_new_file_test() {
        let path = "Common-Fs-File-CreateNew-1.txt";
        {
            let _ =
                File::<AccessRead>::create_new_or_rewrite(path).expect("Failed to crate new file");
            assert!(fs::exists(path).expect("fs::exits failed"));
        }

        fs::remove_file(path).expect("fs::remove_file failed");
    }

    #[test]
    fn write_bytes_file_test() {
        let path = "Common-Fs-File-WriteBytes-1.txt";
        {
            let mut file =
                File::<AccessWrite>::create_new_or_rewrite(path).expect("Failed to crate new file");
            assert!(fs::exists(path).expect("fs::exits failed"));

            let bytes = b"Some bytes";
            file.write_bytes(bytes).expect("Failed to write bytes");
            assert_eq!(fs::read(path).expect("fs::read failed"), bytes);
        }

        fs::remove_file(path).expect("fs::remove_file failed");
    }

    #[test]
    fn fmt_write_file_test() {
        let path = "Common-Fs-File-FmtWrite-1.txt";
        {
            let mut file =
                File::<AccessWrite>::create_new_or_rewrite(path).expect("Failed to crate new file");
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
    fn read_part_file_test() {
        let path = "Cargo.toml";
        {
            let file = File::<AccessRead>::open(path).expect("Failed to crate new file");

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
    fn read_and_read_to_string_test() {
        let path = "Cargo.toml";

        let std = fs::read(path).expect("fs::read_to_string failed");

        let bytes = super::read(path).expect("Failed to read file");
        let string = super::read_to_string(path).expect("Failed to read file or convert Utf8");

        assert_eq!(std, bytes);
        assert_eq!(std, string.as_bytes());
    }

    #[test]
    fn file_size_test() {
        let path = "Cargo.toml";

        let file = File::<AccessRead>::open(path).expect("Failed to open file");
        let size = file.file_size().expect("Failed to get size");
        assert!(size > 0);
    }

    #[test]
    fn create_dir_test() {
        let path = "Common-Fs-File-CreateDir-1";
        super::create_dir(path).expect("Failed to create dir");

        assert!(fs::exists(path).expect("fs::exists failed"));

        fs::remove_dir(path).expect("fs::remove_dir failed");
    }

    #[test]
    fn create_dirs_test() {
        let path = "Common-Fs-File-CreateDir-1/2/3/4/5";
        super::create_dirs(path).expect("Failed to create dir");

        assert!(fs::exists(path).expect("fs::exists failed"));

        fs::remove_dir_all("Common-Fs-File-CreateDir-1").expect("fs::remove_dir failed");
    }

    #[test]
    fn exists_test() {
        assert!(super::exists("Cargo.toml").expect("Failed to check exists"));
        assert!(super::exists("src").expect("Failed to check exists"));
        assert!(!super::exists("NotExists").expect("Failed to check exists"));
    }
}
