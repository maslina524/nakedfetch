use winapi::AsRawHandle;

pub fn as_raw_handle<T: AsRawHandle>(handle: &T) -> *mut () {
    handle.as_raw()
}
