#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use core::{ffi::CStr, num::NonZeroI32, ptr::NonNull};

use crate::{
    error::{LuaExecError, LuaLoadError, LuaPCallError, LuaStateError},
    types::{State, api, aux},
};

pub mod aslua;
pub mod error;
pub mod types;

#[derive(Debug, PartialEq, Eq)]
pub struct LuaState(NonNull<State>);

impl LuaState {
    /// # Errors
    /// If failed to allocate memory for a new [`State`].
    pub fn new_state() -> Result<Self, LuaStateError> {
        let ptr = aux::new_state();
        let state = NonNull::new(ptr).map(Self).ok_or(LuaStateError)?;
        // SAFETY: `State` was just allocated and is valid
        unsafe { aux::open_libs(state.0.as_ptr()) };
        Ok(state)
    }

    /// Compiles `code` and leaves the compiled function on top of the stack.
    /// On error the stack is restored to its state before the call.
    ///
    /// # Errors
    /// If failed to load code.
    pub fn load_code(&mut self, code: &CStr) -> Result<(), LuaLoadError> {
        // SAFETY: Valid state
        let top = unsafe { api::get_top(self.0.as_ptr()) };

        // SAFETY: Ptrs from a valid `&CStr` and a valid state
        let ret = unsafe { aux::load_string(self.0.as_ptr(), code.as_ptr()) };
        let Some(nonzero) = NonZeroI32::new(ret) else {
            return Ok(());
        };

        // SAFETY: `top` was captured on the same valid state
        unsafe { api::set_top(self.0.as_ptr(), top) };

        Err(match nonzero {
            error::LUA_ERRSYNTAX => LuaLoadError::Syntax,
            error::LUA_ERRMEM => LuaLoadError::MemAlloc,
            other => LuaLoadError::Unknown(other),
        })
    }

    /// # Errors
    /// If failed to call code
    pub fn pcall(&mut self) -> Result<(), LuaPCallError> {
        // SAFETY: Valid state
        let top = unsafe { api::get_top(self.0.as_ptr()) };

        // SAFETY: Valid state, valid function at top of stack
        let ret = unsafe { api::pcallk(self.0.as_ptr(), 0, 1, 0, 0, None) };
        let Some(nonzero) = NonZeroI32::new(ret) else {
            return Ok(());
        };

        // SAFETY: `top` was captured on the same valid state
        unsafe { api::set_top(self.0.as_ptr(), top) };

        Err(match nonzero {
            error::LUA_ERRRUN => LuaPCallError::Runtime,
            error::LUA_ERRMEM => LuaPCallError::MemAlloc,
            error::LUA_ERRERR => LuaPCallError::MessageHandler,
            other => LuaPCallError::Unknown(other),
        })
    }

    /// Copies the string on top of the stack (bytes, may contain `\0`)
    /// and pops it. Returns `None` if the value is not convertible to a string.
    #[must_use]
    pub fn take_string(&mut self) -> Option<Vec<u8>> {
        // Ref: <https://github.com/lua/lua/blob/master/lua.h#L60>
        const LUA_TNIL: i32 = 0;
        const LUA_TBOOLEAN: i32 = 1;
        const LUA_TNUMBER: i32 = 3;
        const LUA_TSTRING: i32 = 4;

        fn pop(this: &mut LuaState) {
            // SAFETY: Pop one element
            unsafe { api::set_top(this.0.as_ptr(), -2) };
        }

        fn to_nil(this: &mut LuaState) -> Vec<u8> {
            pop(this);
            Vec::from(b"nil")
        }

        fn to_boolean(this: &mut LuaState) -> bool {
            // SAFETY: Valid state
            let ret = unsafe { api::to_boolean(this.0.as_ptr(), -1) };
            pop(this);
            ret == 1
        }

        fn to_number_or_string(this: &mut LuaState) -> Option<Vec<u8>> {
            let mut len = 0;
            // SAFETY: Valid state
            let ptr = unsafe { api::to_lstring(this.0.as_ptr(), -1, &raw mut len) };
            if ptr.is_null() {
                return None;
            }
            // SAFETY: `lua_tolstring` guarantees `ptr` points to `len` valid
            // bytes for as long as the value stays on the stack; we copy before
            // popping
            let bytes = unsafe { core::slice::from_raw_parts(ptr.cast::<u8>(), len) };
            let out = bytes.to_vec();
            pop(this);
            Some(out)
        }

        // SAFETY: Valid state
        let typ = unsafe { api::typ(self.0.as_ptr(), -1) };

        let val = match typ {
            LUA_TNIL => to_nil(self),
            LUA_TBOOLEAN => {
                if to_boolean(self) {
                    Vec::from(b"true")
                } else {
                    Vec::from(b"false")
                }
            },
            LUA_TNUMBER | LUA_TSTRING => to_number_or_string(self)?,
            _ => return None,
        };
        Some(val)
    }

    /// # Errors
    /// If failed to load or call code, or if the result is not a string
    pub fn exec(&mut self, code: &CStr) -> Result<Vec<u8>, LuaExecError> {
        // SAFETY: Valid state
        let top = unsafe { api::get_top(self.0.as_ptr()) };
        let result = self.exec_inner(code);
        // SAFETY: Restore to the pre-exec stack level, on both paths.
        unsafe { api::set_top(self.0.as_ptr(), top) };
        result
    }

    fn exec_inner(&mut self, code: &CStr) -> Result<Vec<u8>, LuaExecError> {
        self.load_code(code)?;
        self.pcall()?;
        self.take_string()
            .ok_or(LuaExecError::PCall(LuaPCallError::Runtime))
    }
}

impl Drop for LuaState {
    fn drop(&mut self) {
        // SAFETY: We own the state and close it exactly once here
        unsafe { api::close(self.0.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use core::ffi::CStr;

    use crate::LuaState;

    fn create_and_call(code: &CStr) -> String {
        let mut state = LuaState::new_state().expect("Failed to create lua state");
        let ret = state.exec(code).expect("Failed to call lua-code");

        // Lua is not supposed to return valid UTF8 by contract, but in the
        // context of this test it does
        String::from_utf8(ret).expect("Lua returned a non-UTF8 string")
    }

    #[test]
    fn return_nil() {
        let ret = create_and_call(c"return");
        assert_eq!(ret, "nil");
    }

    #[test]
    fn return_int() {
        let ret = create_and_call(c"return 42");
        assert_eq!(ret, "42");
        let ret = create_and_call(c"return -55");
        assert_eq!(ret, "-55");
    }

    #[test]
    fn return_float() {
        let ret = create_and_call(c"return 100.45");
        assert_eq!(ret, "100.45");
        let ret = create_and_call(c"return 1 / 0");
        assert_eq!(ret, "inf");
        let ret = create_and_call(c"return -1 / 0");
        assert_eq!(ret, "-inf");
    }

    #[test]
    fn return_boolean() {
        let ret = create_and_call(c"return 5 == 5");
        assert_eq!(ret, "true");
        let ret = create_and_call(c"return 5 == 0");
        assert_eq!(ret, "false");
    }

    #[test]
    fn concat_strings() {
        let ret = create_and_call(c"return 'Hello ' .. 'World!'");
        assert_eq!(ret, "Hello World!");
        let ret = create_and_call(c"name = 'rustling'; return 'Hello ' .. name .. '!'");
        assert_eq!(ret, "Hello rustling!");
    }
}
