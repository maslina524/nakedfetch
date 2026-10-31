use core::fmt;

use crate::imp::io;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Stdout(io::ConsoleHandle);

impl Stdout {
    #[must_use]
    pub fn new() -> Self {
        Self(io::stdout())
    }

    #[allow(
        clippy::must_use_candidate,
        reason = "The return value will most often be ignored"
    )]
    pub fn write_bytes(self, bytes: &[u8]) -> usize {
        io::console_write(self.0, bytes)
    }
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}

impl Default for Stdout {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Stderr(io::ConsoleHandle);

impl Stderr {
    #[must_use]
    pub fn new() -> Self {
        Self(io::stderr())
    }

    #[allow(
        clippy::must_use_candidate,
        reason = "The return value will most often be ignored"
    )]
    pub fn write_bytes(self, bytes: &[u8]) -> usize {
        io::console_write(self.0, bytes)
    }
}

impl fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}

impl Default for Stderr {
    fn default() -> Self {
        Self::new()
    }
}
