use core::slice;

use wtf8::Wtf8Buf;

use crate::ntdll;

/// # Panics
#[must_use]
pub fn GetCommandLine() -> Wtf8Buf {
    // SAFETY: Completely safe
    let peb = unsafe { ntdll::RtlGetCurrentPeb() };
    let cmd_line = &peb.ProcessParameters.CommandLine;
    // SAFETY: Slice from the memory area allocated by Windows, safe
    let slice = unsafe {
        slice::from_raw_parts(cmd_line.Buffer, cmd_line.Length as usize / size_of::<u16>())
    };
    Wtf8Buf::from_utf16(slice.iter().copied())
}
