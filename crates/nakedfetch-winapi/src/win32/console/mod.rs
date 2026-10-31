use crate::{AsRawHandle, ntdll, types::Handle};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ConsoleHandle(Handle);

impl AsRawHandle for ConsoleHandle {
    fn as_raw(&self) -> Handle {
        self.0
    }

    fn from_raw(raw: Handle) -> Self {
        Self(raw)
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
    let peb = ntdll::RtlGetCurrentPeb();
    let params = peb.process_parameters;

    match typ {
        StdHandle::Stdout => params.standard_output,
        StdHandle::Stderr => params.standard_error,
        StdHandle::Stdin => params.standard_input,
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
        unsafe extern "system" {
            safe fn GetStdHandle(nStdHandle: u32) -> *mut ();
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
