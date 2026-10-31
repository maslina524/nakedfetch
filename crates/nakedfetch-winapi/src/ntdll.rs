use crate::{
    native::peb::Peb,
    ntstatus::Ntstatus,
    types::{self as t, primitives, ptr},
};

#[macro_export]
macro_rules! link {
    ($library:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern "system" {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    );
    ($library:literal $($link_name:literal)? $(#[$doc:meta])? safe fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern "system" {
            $(#[link_name=$link_name])?
            pub safe fn $($function)*;
        }
    );
}

link!("ntdll" fn NtWriteFile(fileHandle: t::Handle, event: t::Handle, apcRoutine: ptr::IoApcRoutinePtr, apcContext: ptr::VoidPtr, ioStatusBlock: *mut t::IoStatusBlock, buffer: ptr::VoidPtr, length: primitives::ULong, byteOffset: ptr::LargeIntegerPtr, key: ptr::ULongPtr) -> Ntstatus);
link!("ntdll" fn NtClose(handle: t::Handle) -> Ntstatus);
link!("ntdll" safe fn RtlGetCurrentPeb() -> &'static Peb);
