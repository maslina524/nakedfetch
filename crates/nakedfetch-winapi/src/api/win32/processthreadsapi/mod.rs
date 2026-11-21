use core::{
    ffi::c_void,
    fmt::{self, Write as _},
    hint,
    mem::ManuallyDrop,
};

use crate::{
    ntdll,
    ntstatus::NtStatus,
    win32::{
        console::{self, StdHandle::Stderr},
        fileapi::{self, FileHandle},
    },
};

const SELF: *mut c_void = (-1i32).cast_unsigned() as *mut c_void;

struct ErrorPrinter(ManuallyDrop<FileHandle>);

impl ErrorPrinter {
    pub fn new() -> Self {
        let handle = console::GetStdHandle(Stderr);
        let filehandle: ManuallyDrop<FileHandle> = handle.into();
        Self(filehandle)
    }
}

impl fmt::Write for ErrorPrinter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = fileapi::WriteFile(&self.0, s.as_bytes());
        Ok(())
    }
}

pub fn ExitProcess(exit_code: u32) -> ! {
    // SAFETY: Terminate self; after calling the function, we completely exit
    // the program, otherwise, we handle the error
    let ret = unsafe { ntdll::NtTerminateProcess(SELF, NtStatus::new(exit_code)) };

    // Here we have already exited the program, if this is not the case, it
    // means exit failed with an error
    let mut out = ErrorPrinter::new();
    let _ = out.write_fmt(format_args!("Failed to exit: {ret}"));
    loop {
        hint::spin_loop();
    }
}
