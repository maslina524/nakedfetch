use crate::{
    native::peb::Peb,
    ntstatus::Ntstatus,
    types::{self as t, primitives, ptr},
};

link!("ntdll" fn NtWriteFile(fileHandle: t::Handle, event: t::Handle, apcRoutine: ptr::IoApcRoutinePtr, apcContext: ptr::VoidPtr, ioStatusBlock: *mut t::IoStatusBlock, buffer: ptr::VoidPtr, length: primitives::ULong, byteOffset: ptr::LargeIntegerPtr, key: ptr::ULongPtr) -> Ntstatus);
link!("ntdll" fn NtClose(handle: t::Handle) -> Ntstatus);
link!("ntdll" safe fn RtlGetCurrentPeb() -> &'static Peb);
