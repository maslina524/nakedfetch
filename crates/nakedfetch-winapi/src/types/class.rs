use core::mem;

#[allow(
    clippy::wildcard_imports,
    reason = "There's no point in importing them separately"
)]
use super::*;

// SAFETY: It runs only at compile time, the layout of u32 and FileInformation
// is the same, check whether the WinAPI function will perceive the enum
// class as a number
const _: () = assert!(
    5u32 == unsafe {
        mem::transmute::<FILE_INFORMATION_CLASS, u32>(
            FILE_INFORMATION_CLASS::FileStandardInformation,
        )
    }
);

#[repr(u32)]
pub enum FILE_INFORMATION_CLASS {
    FileStandardInformation = 5,
}

define_struct!(
    typedef struct _FILE_STANDARD_INFORMATION {
        LARGE_INTEGER AllocationSize;
        LARGE_INTEGER EndOfFile;
        ULONG NumberOfLinks;
        BOOLEAN DeletePending;
        BOOLEAN Directory;
    } FILE_STANDARD_INFORMATION, PFILE_STANDARD_INFORMATION;
);

#[repr(u32)]
pub enum SYSTEM_INFORMATION_CLASS {
    SystemProcessInformation = 5,
}

define_struct!(
    typedef struct _SYSTEM_PROCESS_INFORMATION {
        ULONG NextEntryOffset;
        ULONG NumberOfThreads;
        ULONGLONG WorkingSetPrivateSize;
        ULONG HardFaultCount;
        ULONG NumberOfThreadsHighWatermark;
        ULONGLONG CycleTime;
        LARGE_INTEGER CreateTime;
        LARGE_INTEGER UserTime;
        LARGE_INTEGER KernelTime;
        UNICODE_STRING ImageName;
        KPRIORITY BasePriority;
        HANDLE UniqueProcessId;
        HANDLE InheritedFromUniqueProcessId;
        ULONG HandleCount;
        ULONG SessionId;
        ULONG_PTR UniqueProcessKey;
        SIZE_T PeakVirtualSize;
        SIZE_T VirtualSize;
        ULONG PageFaultCount;
        SIZE_T PeakWorkingSetSize;
        SIZE_T WorkingSetSize;
        SIZE_T QuotaPeakPagedPoolUsage;
        SIZE_T QuotaPagedPoolUsage;
        SIZE_T QuotaPeakNonPagedPoolUsage;
        SIZE_T QuotaNonPagedPoolUsage;
        SIZE_T PagefileUsage;
        SIZE_T PeakPagefileUsage;
        SIZE_T PrivatePageCount;
        LARGE_INTEGER ReadOperationCount;
        LARGE_INTEGER WriteOperationCount;
        LARGE_INTEGER OtherOperationCount;
        LARGE_INTEGER ReadTransferCount;
        LARGE_INTEGER WriteTransferCount;
        LARGE_INTEGER OtherTransferCount;
        SYSTEM_THREAD_INFORMATION Threads;
    } SYSTEM_PROCESS_INFORMATION, PSYSTEM_PROCESS_INFORMATION;
);
