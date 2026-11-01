use core::marker::PhantomData;

mod CreateFile;
pub use CreateFile::*;

mod GetFileAttributes;
pub use GetFileAttributes::*;

mod WriteFile;
pub use WriteFile::*;

use crate::{AsRawHandle, types::Handle, win32::handleapi};

mod_sealed!();
pub trait FileAccess: sealed::Sealed {
    fn raw() -> u32;
}

pub trait CanRead: FileAccess {}
pub trait CanWrite: FileAccess {}

pub struct Read;
pub struct Write;
pub struct Append;
pub struct All;

impl_tag!(CanRead: Read, All);
impl_tag!(CanWrite: Write, Append, All);

impl_sealed!(Read, Write, Append, All);

impl FileAccess for Read {
    fn raw() -> u32 {
        0x8000_0000
    }
}

impl FileAccess for Write {
    fn raw() -> u32 {
        0x4000_0000
    }
}

impl FileAccess for Append {
    fn raw() -> u32 {
        0x04
    }
}

impl FileAccess for All {
    fn raw() -> u32 {
        0x8000_0000 | 0x4000_0000
    }
}

#[repr(transparent)]
pub struct FileHandle<Access> {
    raw: Handle,
    _marker: PhantomData<Access>,
}

impl<Access> AsRawHandle for FileHandle<Access> {
    fn from_raw(raw: Handle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    fn as_raw(&self) -> Handle {
        self.raw
    }
}

impl<Access> Drop for FileHandle<Access> {
    fn drop(&mut self) {
        handleapi::CloseHandle(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AsRawHandle,
        win32::fileapi::{FileHandle, Write},
    };

    const STDOUT: u32 = (-11i32).cast_unsigned();

    #[test]
    fn write_to_std_test() {
        unsafe extern "system" {
            safe fn GetStdHandle(nStdHandle: u32) -> *mut ();
            safe fn SetConsoleOutputCP(wCodePage: u32) -> i32;
        }

        SetConsoleOutputCP(65001);

        let stdout = GetStdHandle(STDOUT);
        let handle = FileHandle::<Write>::from_raw(stdout);

        let bytes = b"Hello world!!!\n";
        let written = WriteFile(&handle, bytes).expect("Failed to write to the console, Ntstatus");
        assert_eq!(written, bytes.len());

        let bytes = "Привет Мир!!!\n".as_bytes();
        let written = WriteFile(&handle, bytes).expect("Failed to write to the console, Ntstatus");
        assert_eq!(written, bytes.len());
    }

    #[test]
    fn create_file_test() {
        let _: FileHandle<Read> = CreateFile(
            "Cargo.toml",
            FileShare::ReadWrite,
            FileDispose::OpenAlways,
            FileAttributes::new(),
            CreateOptions::new().synchronize(),
        )
        .expect("Failed to open file");
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
}
