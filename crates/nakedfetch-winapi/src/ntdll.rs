use crate::{
    native::peb::Peb,
    ntstatus::Ntstatus,
    types::{self as t, Boolean, primitives, ptr},
};

link!("ntdll" fn NtWriteFile(fileHandle: t::Handle, event: t::Handle, apcRoutine: ptr::IoApcRoutinePtr, apcContext: ptr::VoidPtr, ioStatusBlock: *mut t::IoStatusBlock, buffer: ptr::VoidPtr, length: primitives::ULong, byteOffset: ptr::LargeIntegerPtr, key: ptr::ULongPtr) -> Ntstatus);
link!("ntdll" fn NtClose(handle: t::Handle) -> Ntstatus);
link!("ntdll" safe fn RtlGetCurrentPeb() -> &'static Peb);
link!("ntdll" fn RtlAllocateHeap(hHeap: t::Handle, dwFlags: primitives::ULong, dwBytes: usize) -> ptr::VoidPtr);
link!("ntdll" fn RtlFreeHeap(hHeap: t::Handle, dwFlags: primitives::ULong, lpMem: ptr::VoidPtr) -> Boolean);
link!("ntdll" fn RtlReAllocateHeap(hHeap: t::Handle, dwFlags: primitives::ULong, lpMem: ptr::VoidPtr, size: usize) -> ptr::VoidPtr);
link!("ntdll" fn RtlDosPathNameToNtPathName_U(DosFileName: ptr::WideStrConstPtr, NtFileName: *mut t::RawUnicodeString, FilePart: *mut ptr::WideStrPtr, RelativeName: ptr::VoidPtr) -> Boolean);
link!("ntdll" fn RtlFreeUnicodeString(UnicodeString: *mut t::RawUnicodeString) -> ());
