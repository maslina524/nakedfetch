use core::ffi::{c_char, c_int};

pub type State = ();

pub type Pcallk = unsafe extern "C" fn(
    state: *mut State,
    nargs: c_int,
    nresults: c_int,
    errfunc: c_int,
    ctx: isize,
    k: Option<unsafe extern "C" fn(*mut State, c_int, isize)>,
) -> c_int;
pub type Close = unsafe extern "C" fn(state: *mut State);
pub type ToLString =
    unsafe extern "C" fn(state: *mut State, idx: c_int, len: *mut usize) -> *const c_char;
pub type SetTop = unsafe extern "C" fn(state: *mut State, idx: c_int);

unsafe extern "C" {
    #[link_name = "lua_pcallk"]
    unsafe fn pcallk(
        state: *mut State,
        nargs: c_int,
        nresults: c_int,
        errfunc: c_int,
        ctx: isize,
        k: Option<unsafe extern "C" fn(*mut State, c_int, isize)>,
    ) -> c_int;
    #[link_name = "lua_close"]
    unsafe fn close(state: *mut State);
    #[link_name = "lua_tolstring"]
    unsafe fn to_lstring(state: *mut State, idx: c_int, len: *mut usize) -> *const c_char;
    #[link_name = "lua_settop"]
    unsafe fn set_top(state: *mut State, idx: c_int);
}

pub mod aux {
    use core::ffi::{c_char, c_int};

    use super::State;

    pub type NewState = unsafe extern "C" fn() -> *mut State;
    pub type LoadString = unsafe extern "C" fn(state: *mut State, s: *const c_char) -> c_int;
    #[cfg(lua5_4)]
    pub type OpenLibs = unsafe extern "C" fn(state: *mut State);
    #[cfg(lua5_5)]
    pub type OpenSelectedLibs = unsafe extern "C" fn(state: *mut State, mask: c_int);

    unsafe extern "C" {
        #[link_name = "luaL_newstate"]
        unsafe fn new_state() -> *mut State;
        #[link_name = "luaL_loadstring"]
        unsafe fn load_string(state: *mut State, s: *const c_char) -> c_int;

        #[cfg(lua5_4)]
        #[link_name = "luaL_openlibs"]
        unsafe fn open_libs(state: *mut lua_State);
        #[cfg(lua5_5)]
        #[link_name = "luaL_openselectedlibs"]
        unsafe fn open_selected_libs(state: *mut lua_State, mask: c_int);
    }
}
