use alloc::vec::Vec;
use core::fmt;

use path::Path;

use crate::imp::fs;
pub use crate::imp::fs::{AccessAll, AccessCanRead, AccessCanWrite, AccessRead, AccessWrite};

pub struct File<A: fs::Access>(fs::FileHandle<A>);

impl<A: fs::Access> File<A> {
    /// # Errors
    ///
    /// Error if the system failed to open the file correctly.
    pub fn create_new<T: AsRef<Path>>(path: T) -> Result<Self, fs::FileCreateError> {
        fs::create_file(path.as_ref()).map(Self)
    }

    /// # Errors
    ///
    /// Error if the system failed to open the file correctly.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self, fs::FileCreateError> {
        fs::open_file(path.as_ref()).map(Self)
    }

    /// # Errors
    ///
    /// Error if the system failed to get size of file correctly.
    pub fn file_size(&self) -> Result<usize, fs::FileSizeError> {
        fs::size_of_file(&self.0)
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
    ) -> Result<usize, fs::FileReadError> {
        fs::read_file(&self.0, buf, start, count)
    }

    /// # Errors
    ///
    /// Error if the system failed to read the file correctly.
    pub fn read(&self) -> Result<Vec<u8>, fs::FileReadError> {
        let size = self
            .file_size()
            .map_err(|_| fs::FileReadError::BufTooSmall)?;

        let mut buf = Vec::with_capacity(size);
        let written = fs::read_file(&self.0, buf.as_mut(), 0, size)?;
        // SAFETY: Set the length equal to the number of bytes read, safe
        unsafe { buf.set_len(written) };

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

#[cfg(test)]
mod tests {
    use core::fmt::Write as _;
    use std::fs;

    use crate::fs::{AccessRead, AccessWrite, File};

    #[test]
    fn create_new_file_test() {
        let path = "Common-Fs-File-CreateNew-1.txt";
        {
            let _ = File::<AccessRead>::create_new(path).expect("Failed to crate new file");
            assert!(fs::exists(path).expect("fs::exits failed"));
        }

        fs::remove_file(path).expect("fs::remove_file failed");
    }

    #[test]
    fn write_bytes_file_test() {
        let path = "Common-Fs-File-WriteBytes-1.txt";
        {
            let mut file = File::<AccessWrite>::create_new(path).expect("Failed to crate new file");
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
            let mut file = File::<AccessWrite>::create_new(path).expect("Failed to crate new file");
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
}
