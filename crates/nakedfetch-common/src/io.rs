use crate::imp::io;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Stdout(io::ConsoleHandle);

impl Stdout {
    #[must_use]
    pub fn new() -> Self {
        Self(io::stdout())
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
}

impl Default for Stderr {
    fn default() -> Self {
        Self::new()
    }
}