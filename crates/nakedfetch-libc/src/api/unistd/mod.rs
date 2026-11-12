mod syscalls;
pub use syscalls::*;

use crate::{AsRawHandle as _, stdio::File};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ConsoleHandle(i32);

impl ConsoleHandle {
    #[must_use]
    pub fn into_file(self) -> File {
        File::from_raw(self.0)
    }
}

pub const STDIN: ConsoleHandle = ConsoleHandle(0);
pub const STDOUT: ConsoleHandle = ConsoleHandle(1);
pub const STDERR: ConsoleHandle = ConsoleHandle(2);
