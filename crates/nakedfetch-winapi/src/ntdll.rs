#![allow(
    clippy::upper_case_acronyms,
    non_camel_case_types,
    reason = "Windows style"
)]

use crate::{
    native::peb,
    ntstatus::NtStatus,
    types::{self as t, Boolean, Logical, RawUnicodeString, class, primitives as prim, ptr},
};

// Primitives
type VOID = core::ffi::c_void;
type BOOLEAN = Boolean;
type SIZE_T = usize;
type ULONG = prim::ULong;
type LARGE_INTEGER = prim::LargeInteger;
type ACCESS_MASK = prim::ULong;
type LOGICAL = Logical;

// Primitives Ptr
type PULONG = *mut ULONG;
type PLARGE_INTEGER = *mut LARGE_INTEGER;

type PVOID = *mut VOID;

// Strings
type PWSTR = *mut u16;
type PCWSTR = *const u16;

// Structs
type HANDLE = t::Handle;
type FILE_BASIC_INFORMATION = t::FileBasicInfo;
type UNICODE_STRING = RawUnicodeString;
type IO_STATUS_BLOCK = t::IoStatusBlock;
type OBJECT_ATTRIBUTES = t::ObjectAttributes;
type PEB = peb::Peb;

type NTSTATUS = NtStatus;

// Structs mut ptr
type PHANDLE = *mut HANDLE;
type PFILE_BASIC_INFORMATION = *mut FILE_BASIC_INFORMATION;
type PUNICODE_STRING = *mut UNICODE_STRING;
type PIO_STATUS_BLOCK = *mut IO_STATUS_BLOCK;

type PIO_APC_ROUTINE = ptr::IoApcRoutinePtr;
type PRTL_RELATIVE_NAME_U = PVOID;
type PPEB = &'static PEB;

// Structs const ptr
type PCOBJECT_ATTRIBUTES = *const OBJECT_ATTRIBUTES;
type PCUNICODE_STRING = *const UNICODE_STRING;

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

    BOOLEAN NTAPI RtlDosPathNameToNtPathName_U(
        _In_ PCWSTR DosFileName,
        _Out_ PUNICODE_STRING NtFileName,
        _Out_opt_ PWSTR *FilePart,
        _Out_opt_ PRTL_RELATIVE_NAME_U RelativeName
    );

    LOGICAL NTAPI RtlFreeHeap(
        _In_ HANDLE HeapHandle,
        _In_opt_ ULONG Flags,
        _Frees_ptr_opt__Post_invalid_ PVOID BaseAddress
    );

    VOID NTAPI RtlFreeUnicodeString(
        _Inout_ PUNICODE_STRING UnicodeString
    );

    PPEB NTAPI RtlGetCurrentPeb(
        VOID
    );

    VOID NTAPI RtlInitUnicodeString(
        _Out_ PUNICODE_STRING DestinationString,
        _In_opt_z_ PCWSTR SourceString
    );

    NTSTATUS NTAPI RtlQueryEnvironmentVariable_U(
        _In_opt_ PVOID Environment,
        _In_ PCUNICODE_STRING Name,
        _Inout_ PUNICODE_STRING Value
    );

    PVOID NTAPI RtlReAllocateHeap(
        _In_ HANDLE HeapHandle,
        _In_ ULONG Flags,
        _Frees_ptr_opt_ PVOID BaseAddress,
        _In_ SIZE_T Size
    );
);
