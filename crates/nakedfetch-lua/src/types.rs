#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State(());

pub mod api {
    use core::ffi::{c_char, c_int};

    use super::State;

    unsafe extern "C" {
        #[link_name = "lua_pcallk"]
        pub unsafe fn pcallk(
            state: *mut State,
            nargs: c_int,
            nresults: c_int,
            errfunc: c_int,
            ctx: isize,
            k: Option<unsafe extern "C" fn(*mut State, c_int, isize)>,
        ) -> c_int;
        #[link_name = "lua_close"]
        pub unsafe fn close(state: *mut State);
        #[link_name = "lua_tolstring"]
        pub unsafe fn to_lstring(state: *mut State, idx: c_int, len: *mut usize) -> *const c_char;
        #[link_name = "lua_settop"]
        pub unsafe fn set_top(state: *mut State, idx: c_int);
    }
}

pub mod aux {
    use core::ffi::{c_char, c_int};

    use super::State;

    unsafe extern "C" {
        #[link_name = "luaL_newstate"]
        pub safe fn new_state() -> *mut State;
        #[link_name = "luaL_loadstring"]
        pub unsafe fn load_string(state: *mut State, s: *const c_char) -> c_int;
    }

    #[cfg(lua5_4)]
    pub fn open_libs(state: *mut State) {
        unsafe extern "C" {
            safe fn luaL_openlibs(state: *mut State);
        }
        luaL_openlibs(state);
    }

    #[cfg(lua5_5)]
    pub fn open_libs(state: *mut State) {
        unsafe extern "C" {
            safe fn luaL_openselectedlibs(state: *mut State, mask: c_int);
        }
        luaL_openselectedlibs(state, -1);
    }
}
