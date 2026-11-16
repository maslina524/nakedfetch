#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::{ffi::CStr, num::NonZeroI32, ptr::NonNull};

use crate::{
    error::{LuaLoadError, LuaStateError},
    types::{State, aux},
};

extern crate alloc;

pub mod aslua;
pub mod error;
pub mod types;

macro_rules! nonzeroi32 {
    ($lit:literal) => {{
        match ::core::num::NonZeroI32::new($lit) {
            Some(v) => v,
            None => panic!("Passing 0 to `NonZeroI32` is UB"),
        }
    }};
}

// Ref: <https://github.com/lua/lua/blob/master/lua.h#L47>
const LUA_ERRSYNTAX: NonZeroI32 = nonzeroi32!(3);
const LUA_ERRMEM: NonZeroI32 = nonzeroi32!(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LuaState(NonNull<State>);

impl LuaState {
    /// # Errors
    /// If failed to allocate memory for a new [`State`].
    pub fn new_state() -> Result<Self, LuaStateError> {
        let ptr = aux::new_state();
        NonNull::new(ptr).map(Self).ok_or(LuaStateError)
    }

    pub fn load_code(self, code: &CStr) -> Result<(), LuaLoadError> {
        // SAFETY: Ptr is created from `&CStr`, valid; `LuaState` is also valid
        let ret = unsafe { aux::load_string(self.0.as_ptr(), code.as_ptr()) };
        let Some(nonzero) = NonZeroI32::new(ret) else {
            return Ok(());
        };

        let err = match nonzero {
            LUA_ERRSYNTAX => LuaLoadError::Syntax,
            LUA_ERRMEM => LuaLoadError::MemAlloc,
            _ => LuaLoadError::Unknown(nonzero),
        };
        Err(err)
    }
}
