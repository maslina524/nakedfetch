#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::{ffi::CStr, num::NonZeroI32, ptr::NonNull};

use crate::{
    error::{LuaExecError, LuaLoadError, LuaPCallError, LuaStateError},
    types::{State, api, aux},
};

extern crate alloc;

pub mod aslua;
pub mod error;
pub mod types;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LuaState(NonNull<State>);

impl LuaState {
    /// # Errors
    /// If failed to allocate memory for a new [`State`].
    pub fn new_state() -> Result<Self, LuaStateError> {
        let ptr = aux::new_state();
        NonNull::new(ptr).map(Self).ok_or(LuaStateError)
    }

    /// # Errors
    /// If failed to load code
    pub fn load_code(&self, code: &CStr) -> Result<(), LuaLoadError> {
        // SAFETY: Ptr is created from `&CStr`, valid; `LuaState` is also valid
        let ret = unsafe { aux::load_string(self.0.as_ptr(), code.as_ptr()) };
        let Some(nonzero) = NonZeroI32::new(ret) else {
            return Ok(());
        };

        let err = match nonzero {
            error::LUA_ERRSYNTAX => LuaLoadError::Syntax,
            error::LUA_ERRMEM => LuaLoadError::MemAlloc,
            _ => LuaLoadError::Unknown(nonzero),
        };
        Err(err)
    }

    /// # Errors
    /// If failed to call code
    pub fn pcall(&self) -> Result<(), LuaPCallError> {
        // SAFETY: Call the compiled function with 0 arguments and expect 1
        // result The last argument is a continuation function ptr (NULL)
        let ret = unsafe { api::pcallk(self.0.as_ptr(), 0, 1, 0, 0, None) };
        let Some(nonzero) = NonZeroI32::new(ret) else {
            return Ok(());
        };

        let err = match nonzero {
            error::LUA_ERRRUN => LuaPCallError::Runtime,
            error::LUA_ERRMEM => LuaPCallError::MemAlloc,
            error::LUA_ERRERR => LuaPCallError::MessageHandler,
            _ => LuaPCallError::Unknown(nonzero),
        };
        Err(err)
    }

    /// # Errors
    /// If failed to load or call code
    pub fn exec(self, code: &CStr) -> Result<&CStr, LuaExecError> {
        aux::open_libs(self.0.as_ptr());
        self.load_code(code)?;
        self.pcall()?;

        Ok(c"")
    }
}

impl Drop for LuaState {
    fn drop(&mut self) {
        // SAFETY: Close the State only once
        unsafe { api::close(self.0.as_ptr()) };
    }
}
