use libc::unistd;
pub use libc::unistd::ConsoleHandle;

use crate::imp::fs;

#[allow(clippy::missing_const_for_fn, reason = "For comp API with windows")]
pub fn stdout() -> ConsoleHandle {
    unistd::STDOUT
}

#[allow(clippy::missing_const_for_fn, reason = "For comp API with windows")]
pub fn stderr() -> ConsoleHandle {
    unistd::STDERR
}

pub fn console_write(handle: ConsoleHandle, buf: &[u8]) -> usize {
    let manually_drop = handle.into_file();
    fs::write_file(&manually_drop, buf).unwrap_or_default()
    // panic!("Failed to write to the console: {e:?}")
}
