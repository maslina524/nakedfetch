use crate::{AsRawHandle, ntdll, ntstatus::NtStatus};

pub fn CloseHandle<T: AsRawHandle>(handle: &mut T) -> NtStatus {
    // SAFETY: Handle is valid, safe
    unsafe { ntdll::NtClose(handle.as_raw()) }
}

#[cfg(test)]
mod tests {
    use crate::{
        AsRawHandle,
        win32::{
            fileapi::{FileHandle, Read},
            handleapi::CloseHandle,
        },
    };

    #[test]
    fn close_handle_test() {
        type Error = u32;
        type Attrs = u32;

        unsafe extern "system" {
            safe fn CreateFileA(
                lpFileName: *const u8,
                dwDesiredAccess: u32,
                dwShareMode: u32,
                lpSecurityAttributes: *mut (),
                dwCreationDisposition: u32,
                dwFlagsAndAttributes: u32,
                hTemplateFile: *mut (),
            ) -> *mut ();
            safe fn GetHandleInformation(hObject: *mut (), lpdwFlags: *mut u32) -> i32;
            safe fn GetLastError() -> u32;
        }

        fn get_handle_attrs(raw: *mut ()) -> Result<Attrs, Error> {
            let mut flags = 0;
            let ret = GetHandleInformation(raw, &raw mut flags);
            if ret == 0 {
                Err(GetLastError())
            } else {
                Ok(flags)
            }
        }

        let raw = CreateFileA(
            c"Cargo.toml".as_ptr().cast(),
            0x8000_0000,
            3,
            core::ptr::null_mut(),
            4,
            0x80,
            core::ptr::null_mut(),
        );
        let mut handle = FileHandle::<Read>::from_raw(raw);

        let attrs = get_handle_attrs(handle.as_raw());
        assert!(attrs.is_ok());

        CloseHandle(&mut handle);
        let attrs = get_handle_attrs(handle.as_raw());
        assert!(attrs.is_err());
    }
}
