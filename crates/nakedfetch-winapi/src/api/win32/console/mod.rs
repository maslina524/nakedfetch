use core::mem::ManuallyDrop;

use crate::{
    AsRawHandle, ntdll,
    types::HANDLE,
    win32::fileapi::{FileHandle, Write},
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ConsoleHandle(HANDLE);

impl AsRawHandle for ConsoleHandle {
    fn as_raw(&self) -> HANDLE {
        self.0
    }

    fn from_raw(raw: HANDLE) -> Self {
        Self(raw)
    }
}

impl From<ConsoleHandle> for ManuallyDrop<FileHandle<Write>> {
    fn from(val: ConsoleHandle) -> Self {
        Self::new(FileHandle::from_raw(val.as_raw()))
    }
}

#[derive(Clone, Copy)]
pub enum StdHandle {
    Stdout,
    Stderr,
    Stdin,
}

#[must_use]
pub fn GetStdHandle(typ: StdHandle) -> ConsoleHandle {
    // SAFETY: Completely safe
    let peb = unsafe { ntdll::RtlGetCurrentPeb() };
    let params = peb.ProcessParameters;

    match typ {
        StdHandle::Stdout => params.StandardOutput,
        StdHandle::Stderr => params.StandardError,
        StdHandle::Stdin => params.StandardInput,
    }
}

#[cfg(test)]
mod tests {
    use crate::{AsRawHandle, win32::console::StdHandle};

    const STDIN: u32 = (-10i32).cast_unsigned();
    const STDOUT: u32 = (-11i32).cast_unsigned();
    const STDERR: u32 = (-12i32).cast_unsigned();

    #[test]
    fn std_handles_test() {
        use core::ffi::c_void;

        unsafe extern "system" {
            safe fn GetStdHandle(nStdHandle: u32) -> *mut c_void;
        }

        let winapi = GetStdHandle(STDIN);
        let my = super::GetStdHandle(StdHandle::Stdin);
        assert_eq!(winapi, my.as_raw());

        let winapi = GetStdHandle(STDOUT);
        let my = super::GetStdHandle(StdHandle::Stdout);
        assert_eq!(winapi, my.as_raw());

        let winapi = GetStdHandle(STDERR);
        let my = super::GetStdHandle(StdHandle::Stderr);
        assert_eq!(winapi, my.as_raw());
    }
}
