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

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::fs::{AccessRead, File};

    #[test]
    fn create_new_file_test() {
        let path = "Common-Fs-File-CreateNew-1.txt";
        let _ = File::<AccessRead>::create_new(path).expect("Failed to crate new file");
        assert!(fs::exists(path).expect("fs::exits failed"));
        fs::remove_file(path).expect("fs::remove_file failed");
    }
}
