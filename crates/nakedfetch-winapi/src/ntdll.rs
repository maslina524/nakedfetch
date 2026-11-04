#![allow(
    clippy::upper_case_acronyms,
    non_camel_case_types,
    reason = "Windows style"
)]

use crate::{
    native::peb::Peb,
    ntstatus::NtStatus,
    types::{self as t, Boolean, class, primitives as prim, ptr},
};

link!("ntdll" safe fn RtlGetCurrentPeb() -> &'static Peb);
link!("ntdll" fn RtlFreeHeap(hHeap: t::Handle, dwFlags: prim::ULong, lpMem: ptr::VoidPtr) -> Boolean);
link!("ntdll" fn RtlReAllocateHeap(hHeap: t::Handle, dwFlags: prim::ULong, lpMem: ptr::VoidPtr, size: usize) -> ptr::VoidPtr);
link!("ntdll" fn RtlDosPathNameToNtPathName_U(DosFileName: ptr::WideStrConstPtr, NtFileName: *mut t::RawUnicodeString, FilePart: *mut ptr::WideStrPtr, RelativeName: ptr::VoidPtr) -> Boolean);
link!("ntdll" fn RtlFreeUnicodeString(UnicodeString: *mut t::RawUnicodeString) -> ());
link!("ntdll" fn RtlQueryEnvironmentVariable_U(Environment: ptr::WideStrPtr, Name: *const t::RawUnicodeString, Value: *mut t::RawUnicodeString) -> NtStatus);
link!("ntdll" fn RtlInitUnicodeString(DestinationString: *mut t::RawUnicodeString, SourceString: ptr::WideStrConstPtr) -> ());

// Primitives
type BOOLEAN = Boolean;
type SIZE_T = usize;
type ULONG = prim::ULong;
type ACCESS_MASK = prim::ULong;

// Primitives Ptr
type PULONG = *mut ULONG;
type PLARGE_INTEGER = *mut prim::LargeInteger;

type PVOID = ptr::VoidPtr;

// Strings
type PWSTR = *mut u16;
type PCWSTR = *const u16;

// Structs
type NTSTATUS = NtStatus;
type HANDLE = t::Handle;

// Structs mut ptr
type PHANDLE = *mut t::Handle;
type PFILE_BASIC_INFORMATION = *mut t::FileBasicInfo;

type PIO_APC_ROUTINE = ptr::IoApcRoutinePtr;
type PIO_STATUS_BLOCK = *mut t::IoStatusBlock;

// Structs const ptr
type PCOBJECT_ATTRIBUTES = *const t::ObjectAttributes;

// Classes
type FILE_INFORMATION_CLASS = class::FileInformation;

define_fn!(
    NTSTATUS NTAPI NtClose(
        _In__Post_ptr_invalid_ HANDLE Handle
    );

    NTSTATUS NTAPI NtCreateFile(
        _Out_ PHANDLE FileHandle,
        _In_ ACCESS_MASK DesiredAccess,
        _In_ PCOBJECT_ATTRIBUTES ObjectAttributes,
        _Out_ PIO_STATUS_BLOCK IoStatusBlock,
        _In_opt_ PLARGE_INTEGER AllocationSize,
        _In_ ULONG FileAttributes,
        _In_ ULONG ShareAccess,
        _In_ ULONG CreateDisposition,
        _In_ ULONG CreateOptions,
        _In_ PVOID EaBuffer,
        _In_ ULONG EaLength
    );

    NTSTATUS NTAPI NtQueryAttributesFile(
        _In_ PCOBJECT_ATTRIBUTES ObjectAttributes,
        _Out_ PFILE_BASIC_INFORMATION FileInformation
    );

    NTSTATUS NTAPI NtQueryInformationFile(
        _In_ HANDLE FileHandle,
        _Out_ PIO_STATUS_BLOCK IoStatusBlock,
        _Out_ PVOID FileInformation,
        _In_ ULONG Length,
        _In_ FILE_INFORMATION_CLASS FileInformationClass
        );

    NTSTATUS NTAPI NtReadFile(
        _In_ HANDLE FileHandle,
        _In_opt_ HANDLE Event,
        _In_opt_ PIO_APC_ROUTINE ApcRoutine,
        _In_opt_ PVOID ApcContext,
        _Out_ PIO_STATUS_BLOCK IoStatusBlock,
        _Out_ PVOID Buffer,
        _In_ ULONG Length,
        _In_opt_ PLARGE_INTEGER ByteOffset,
        _In_opt_ PULONG Key
    );

    NTSTATUS NTAPI NtWriteFile(
        _In_ HANDLE FileHandle,
        _In_opt_ HANDLE Event,
        _In_opt_ PIO_APC_ROUTINE ApcRoutine,
        _In_opt_ PVOID ApcContext,
        _Out_ PIO_STATUS_BLOCK IoStatusBlock,
        _In_ PVOID Buffer,
        _In_ ULONG Length,
        _In_opt_ PLARGE_INTEGER ByteOffset,
        _In_opt_ PULONG Key
    );

    PVOID NTAPI RtlAllocateHeap(
        _In_ HANDLE HeapHandle,
        _In_opt_ ULONG Flags,
        _In_ SIZE_T Size
    );

);

// RtlFreeHeap
// RtlFreeUnicodeString
// RtlGetCurrentPeb
// RtlInitUnicodeString
// RtlQueryEnvironmentVariable_U
// RtlReAllocateHeap
