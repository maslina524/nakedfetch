use crate::types::{self as t, ptr};
use crate::ntstatus::Ntstatus;

#[macro_export]
macro_rules! link {
    ($library:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern "system" {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

link!("ntdll" fn NtWriteFile(fileHandle: t::Handle, event: t::Handle, apcRoutine: ptr::IoApcRoutinePtr, apcContext: ptr::VoidPtr, ioStatusBlock: *mut t::IoStatusBlock, buffer: ptr::VoidPtr, length: t::primitives::ULong, byteOffset: ptr::LargeIntegerPtr, key: ptr::ULongPtr) -> Ntstatus);
