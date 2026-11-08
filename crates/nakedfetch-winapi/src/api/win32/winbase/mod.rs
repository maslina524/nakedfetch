mod CreateDirectory;
pub use CreateDirectory::*;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::win32::fileapi::FileAttributes;

    const FILE_ATTRIBUTE_HIDDEN: u32 = 1 << 1;

    #[test]
    fn create_directory_test() {
        let path = "Winbase-CreateDirectory-mod-1";
        CreateDirectory(path, FileAttributes::new()).expect("Failed to create dir");

        let ret = fs::exists(path).expect("fs::exists failed");
        assert!(ret);

        fs::remove_dir(path).expect("fs::remove_dir failed");
    }

    #[test]
    fn create_hidden_directory_test() {
        unsafe extern "system" {
            safe fn GetFileAttributesA(lpFileName: *const u8) -> u32;
        }

        let path = "Winbase-CreateDirectory-mod-2";
        CreateDirectory(path, FileAttributes::new().hidden()).expect("Failed to create dir");

        let ret = fs::exists(path).expect("fs::exists failed");
        assert!(ret);

        let meta = GetFileAttributesA(c"Winbase-CreateDirectory-mod-2".as_ptr().cast());
        assert_ne!(meta, u32::MAX, "GetFileAttributesA failed (meta)");
        assert!(meta & FILE_ATTRIBUTE_HIDDEN != 0, "Dir is not hidden");

        fs::remove_dir(path).expect("fs::remove_dir failed");
    }
}
