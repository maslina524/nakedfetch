use libc::AsRawHandle;

pub fn as_raw_handle<T: AsRawHandle>(handle: &T) -> i32 {
    handle.as_raw()
}
