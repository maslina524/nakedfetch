use core::ffi::c_void;

use winapi::AsRawHandle;

pub fn as_raw_handle<T: AsRawHandle>(handle: &T) -> *mut c_void {
    handle.as_raw()
}
