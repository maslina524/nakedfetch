use libc::unistd;
pub use libc::unistd::ConsoleHandle;

use crate::imp::fs;

pub fn stdout() -> ConsoleHandle {
    unistd::STDOUT
}

pub fn stderr() -> ConsoleHandle {
    unistd::STDERR
}

pub fn console_write(handle: ConsoleHandle, buf: &[u8]) -> usize {
    match fs::write_file(&handle.into_file(), buf) {
        Ok(v) => v,
        Err(e) => panic!("Failed to write to the console: {e:?}"),
    }
}
