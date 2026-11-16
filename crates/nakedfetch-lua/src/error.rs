use core::{error::Error, fmt, num::NonZeroI32};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LuaStateError;

impl fmt::Display for LuaStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Allocator failed to allocate a new State")
    }
}

impl Error for LuaStateError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LuaLoadError {
    Syntax,
    MemAlloc,
    Unknown(NonZeroI32),
}

impl fmt::Display for LuaLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax => write!(f, "Syntax error"),
            Self::MemAlloc => write!(f, "Allocator returned an error"),
            Self::Unknown(e) => write!(f, "Unexpected error: {e}"),
        }
    }
}

impl Error for LuaLoadError {}
