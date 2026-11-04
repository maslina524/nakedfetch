use crate::{
    native::peb::Peb,
    ntstatus::NtStatus,
    types::{self as t, Boolean, class, primitives as prim, ptr},
};

link!("ntdll" fn NtWriteFile(fileHandle: t::Handle, event: t::Handle, apcRoutine: ptr::IoApcRoutinePtr, apcContext: ptr::VoidPtr, ioStatusBlock: *mut t::IoStatusBlock, buffer: ptr::VoidPtr, length: prim::ULong, byteOffset: ptr::LargeIntegerPtr, key: ptr::ULongPtr) -> NtStatus);
link!("ntdll" fn NtReadFile(fileHandle: t::Handle, event: t::Handle, apcRoutine: ptr::IoApcRoutinePtr, apcContext: ptr::VoidPtr, ioStatusBlock: *mut t::IoStatusBlock, buffer: ptr::VoidPtr, length: prim::ULong, byteOffset: ptr::LargeIntegerPtr, key: ptr::ULongPtr) -> NtStatus);
link!("ntdll" fn NtClose(handle: t::Handle) -> NtStatus);
link!("ntdll" safe fn RtlGetCurrentPeb() -> &'static Peb);
link!("ntdll" fn RtlAllocateHeap(hHeap: t::Handle, dwFlags: prim::ULong, dwBytes: usize) -> ptr::VoidPtr);
link!("ntdll" fn RtlFreeHeap(hHeap: t::Handle, dwFlags: prim::ULong, lpMem: ptr::VoidPtr) -> Boolean);
link!("ntdll" fn RtlReAllocateHeap(hHeap: t::Handle, dwFlags: prim::ULong, lpMem: ptr::VoidPtr, size: usize) -> ptr::VoidPtr);
link!("ntdll" fn RtlDosPathNameToNtPathName_U(DosFileName: ptr::WideStrConstPtr, NtFileName: *mut t::RawUnicodeString, FilePart: *mut ptr::WideStrPtr, RelativeName: ptr::VoidPtr) -> Boolean);
link!("ntdll" fn RtlFreeUnicodeString(UnicodeString: *mut t::RawUnicodeString) -> ());
link!("ntdll" fn NtCreateFile(FileHandle: *mut t::Handle, DesiredAccess: prim::ULong, ObjectAttributes: *mut t::ObjectAttributes, IoStatusBlock: *mut t::IoStatusBlock, AllocationSize: ptr::LargeIntegerPtr, FileAttributes: prim::ULong, ShareAccess: prim::ULong, CreateDisposition: prim::ULong, CreateOptions: prim::ULong, EaBuffer: ptr::VoidPtr, EaLength: prim::ULong) -> NtStatus);
link!("ntdll" fn NtQueryAttributesFile(ObjectAttributes: *mut t::ObjectAttributes, FileInformation: *mut t::FileBasicInfo) -> NtStatus);
link!("ntdll" fn NtQueryInformationFile(FileHandle: t::Handle, IoStatusBlock: *mut t::IoStatusBlock, FileInformation: ptr::VoidPtr, Length: prim::ULong, FileInformationClass: class::FileInformation) -> NtStatus);
link!("ntdll" fn RtlQueryEnvironmentVariable_U(Environment: ptr::WideStrPtr, Name: *const t::RawUnicodeString, Value: *mut t::RawUnicodeString) -> NtStatus);
link!("ntdll" fn RtlInitUnicodeString(DestinationString: *mut t::RawUnicodeString, SourceString: ptr::WideStrConstPtr) -> ());
