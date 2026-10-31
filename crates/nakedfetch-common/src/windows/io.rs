use core::mem::ManuallyDrop;

pub use console::ConsoleHandle;
use winapi::win32::{
    console::{self, GetStdHandle, StdHandle},
    fileapi::{FileHandle, Write, WriteFile},
};

pub fn stdout() -> ConsoleHandle {
    GetStdHandle(StdHandle::Stdout)
}

pub fn stderr() -> ConsoleHandle {
    GetStdHandle(StdHandle::Stderr)
}

pub fn console_write(handle: ConsoleHandle, buf: &[u8]) -> usize {
    let filehandle: ManuallyDrop<FileHandle<Write>> = handle.into();
    match WriteFile(&filehandle, buf) {
        Ok(v) => v,
        Err(e) => panic!("Failed to write to the console: {e:?}"),
    }
}
