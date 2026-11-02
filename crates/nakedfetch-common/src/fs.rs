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
}
