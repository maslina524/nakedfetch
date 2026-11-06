#![allow(
    clippy::upper_case_acronyms,
    non_camel_case_types,
    reason = "Windows style"
)]

#[allow(
    clippy::wildcard_imports,
    reason = "There's no point in importing them separately"
)]
use crate::{native::peb::PPEB, ntstatus::NtStatus, types::*};

define_fn!(
    NtStatus NTAPI NtClose(
        _In__Post_ptr_invalid_ HANDLE Handle
    );

    NtStatus NTAPI NtCreateFile(
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

    NtStatus NTAPI NtOpenProcess(
        _Out_ PHANDLE ProcessHandle,
        _In_ ACCESS_MASK DesiredAccess,
        _In_ PCOBJECT_ATTRIBUTES ObjectAttributes,
        _In_opt_ PCLIENT_ID ClientId
    );

    NtStatus NTAPI NtQueryAttributesFile(
        _In_ PCOBJECT_ATTRIBUTES ObjectAttributes,
        _Out_ PFILE_BASIC_INFORMATION FileInformation
    );

    NtStatus NTAPI NtQueryInformationFile(
        _In_ HANDLE FileHandle,
        _Out_ PIO_STATUS_BLOCK IoStatusBlock,
        _Out_ PVOID FileInformation,
        _In_ ULONG Length,
        _In_ FILE_INFORMATION_CLASS FileInformationClass
        );

    NtStatus NTAPI NtReadFile(
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

    NtStatus NTAPI NtWriteFile(
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

    NtStatus NTAPI RtlQueryEnvironmentVariable_U(
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
