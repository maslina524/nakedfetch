use winapi::win32::console::{self, GetStdHandle, StdHandle};
pub use console::ConsoleHandle;

pub fn stdout() -> ConsoleHandle {
    GetStdHandle(StdHandle::Stdout)
}

pub fn stderr() -> ConsoleHandle {
    GetStdHandle(StdHandle::Stderr)
}