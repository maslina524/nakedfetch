mod CreateFile;
pub use CreateFile::*;

mod GetFileAttributes;
pub use GetFileAttributes::*;

mod GetFileSizeEx;
pub use GetFileSizeEx::*;

mod ReadFile;
pub use ReadFile::*;

mod WriteFile;
pub use WriteFile::*;

use crate::{AsRawHandle, types::HANDLE, win32::handleapi};

#[repr(transparent)]
pub struct FileHandle(HANDLE);

impl AsRawHandle for FileHandle {
    fn from_raw(raw: HANDLE) -> Self {
        Self(raw)
    }

    fn as_raw(&self) -> HANDLE {
        self.0
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        handleapi::CloseHandle(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AsRawHandle, win32::fileapi::FileHandle};

    const STDOUT: u32 = (-11i32).cast_unsigned();

    #[test]
    fn write_to_std_test() {
        use core::ffi::c_void;

        unsafe extern "system" {
            safe fn GetStdHandle(nStdHandle: u32) -> *mut c_void;
            safe fn SetConsoleOutputCP(wCodePage: u32) -> i32;
        }

        SetConsoleOutputCP(65001);

        let stdout = GetStdHandle(STDOUT);
        let handle = FileHandle::from_raw(stdout);

        let bytes = b"Hello world!!!\n";
        let written = WriteFile(&handle, bytes).expect("Failed to write to the console, Ntstatus");
        assert_eq!(written, bytes.len());

        let bytes = "Привет Мир!!!\n".as_bytes();
        let written = WriteFile(&handle, bytes).expect("Failed to write to the console, Ntstatus");
        assert_eq!(written, bytes.len());
    }

    #[test]
    fn create_file_test() {
        let _: FileHandle = CreateFile(
            "Cargo.toml",
            FileAccess::Read,
            FileShare::ReadWrite,
            FileDispose::OpenAlways,
            FileAttributes::new(),
            CreateOptions::new().synchronize(),
        )
        .expect("Failed to open file");
    }

    #[test]
    fn read_file_test() {
        let handle: FileHandle = CreateFile(
            "Cargo.toml",
            FileAccess::Read,
            FileShare::ReadWrite,
            FileDispose::OpenAlways,
            FileAttributes::new(),
            CreateOptions::new().synchronize(),
        )
        .expect("Failed to open file");

        let mut buf = [0u8; 1024];
        let readed = ReadFile(&handle, &mut buf, 1024, None).expect("Failed to read file");
        assert!(readed > 0);

        let string = str::from_utf8(&buf).expect("Incorrect file read");
        assert!(string.contains("[package]"));
    }

    #[test]
    fn get_file_attrs_test() {
        let attrs = GetFileAttributes("Cargo.toml").expect("Failed to get file attrs");
        assert_eq!(
            attrs,
            ItemAttributes {
                is_readonly: false,
                is_hidden: false,
                is_system: false,
                typ: ItemType::File
            }
        );
    }

    #[test]
    fn get_file_size_test() {
        use core::ffi::c_void;

        unsafe extern "system" {
            safe fn GetFileSizeEx(hFile: *mut c_void, lpFileSize: *mut i64) -> i32;
        }

        let handle: FileHandle = CreateFile(
            "Cargo.toml",
            FileAccess::Read,
            FileShare::ReadWrite,
            FileDispose::OpenAlways,
            FileAttributes::new(),
            CreateOptions::new().synchronize(),
        )
        .expect("Failed to open file");

        let mut winapi = 0;
        assert!(
            GetFileSizeEx(handle.as_raw(), &raw mut winapi) != 0,
            "WinAPI failed"
        );
        let my = super::GetFileSizeEx(&handle).expect("Failed to get file size");

        assert_eq!(winapi as usize, my);
    }
}
