use core::{error::Error, fmt, num::NonZeroI32};

macro_rules! nonzeroi32 {
    ($lit:literal) => {{
        match ::core::num::NonZeroI32::new($lit) {
            Some(v) => v,
            None => panic!("Passing 0 to `NonZeroI32` is UB"),
        }
    }};
}

// Ref: <https://github.com/lua/lua/blob/master/lua.h#L47>
pub(crate) const LUA_ERRRUN: NonZeroI32 = nonzeroi32!(2);
pub(crate) const LUA_ERRSYNTAX: NonZeroI32 = nonzeroi32!(3);
pub(crate) const LUA_ERRMEM: NonZeroI32 = nonzeroi32!(4);
pub(crate) const LUA_ERRERR: NonZeroI32 = nonzeroi32!(5);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LuaPCallError {
    Runtime,
    MemAlloc,
    MessageHandler,
    Unknown(NonZeroI32),
}

impl fmt::Display for LuaPCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Runtime => write!(f, "Runtime error"),
            Self::MemAlloc => write!(f, "Allocator returned an error"),
            Self::MessageHandler => write!(
                f,
                "Stack overflow during the operation of the message handler"
            ),
            Self::Unknown(e) => write!(f, "Unexpected error: {e}"),
        }
    }
}

impl Error for LuaPCallError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LuaExecError {
    Load(LuaLoadError),
    PCall(LuaPCallError),
}

impl From<LuaLoadError> for LuaExecError {
    fn from(e: LuaLoadError) -> Self {
        Self::Load(e)
    }
}

impl From<LuaPCallError> for LuaExecError {
    fn from(e: LuaPCallError) -> Self {
        Self::PCall(e)
    }
}

impl fmt::Display for LuaExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load(e) => fmt::Display::fmt(e, f),
            Self::PCall(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl Error for LuaExecError {}
