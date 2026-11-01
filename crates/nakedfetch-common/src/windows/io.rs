use core::{
    mem::ManuallyDrop,
    sync::atomic::{AtomicBool, Ordering},
};

pub use console::ConsoleHandle;
use winapi::{
    kernel32,
    win32::{
        console::{self, GetStdHandle, StdHandle},
        fileapi::{FileHandle, Write, WriteFile},
    },
};

static IS_UTF8: AtomicBool = AtomicBool::new(false);

pub fn stdout() -> ConsoleHandle {
    GetStdHandle(StdHandle::Stdout)
}

pub fn stderr() -> ConsoleHandle {
    GetStdHandle(StdHandle::Stderr)
}

pub fn console_write(handle: ConsoleHandle, buf: &[u8]) -> usize {
    if IS_UTF8
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
    {
        kernel32::SetConsoleOutputCP(65001);
    }

    let filehandle: ManuallyDrop<FileHandle<Write>> = handle.into();
    match WriteFile(&filehandle, buf) {
        Ok(v) => v,
        Err(e) => panic!("Failed to write to the console: {e:?}"),
    }
}
