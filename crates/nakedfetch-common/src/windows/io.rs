pub use console::ConsoleHandle;
use winapi::win32::{
    console::{self, GetStdHandle, StdHandle},
    fileapi::WriteFile,
};

pub fn stdout() -> ConsoleHandle {
    GetStdHandle(StdHandle::Stdout)
}

pub fn stderr() -> ConsoleHandle {
    GetStdHandle(StdHandle::Stderr)
}

pub fn console_write(handle: ConsoleHandle, buf: &[u8]) -> usize {
    match WriteFile(&handle.into(), buf) {
        Ok(v) => v,
        Err(e) => panic!("Failed to weite to the console: {e}"),
    }
}
