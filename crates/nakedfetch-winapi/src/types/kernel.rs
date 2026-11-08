#[allow(
    clippy::wildcard_imports,
    reason = "There's no point in importing them separately"
)]
use super::*;
use crate::ntstatus::NtStatus;

define_struct!(
    typedef struct _KSYSTEM_TIME {
        ULONG LowPart;
        LONG High1Time;
        LONG High2Time;
    } KSYSTEM_TIME, PKSYSTEM_TIME;
);

impl KSYSTEM_TIME {
    #[must_use]
    pub const fn quad(&self) -> u64 {
        ((self.High1Time as u64) << 32) | (self.LowPart as u64)
    }
}

define_struct!(
    typedef struct _IO_STATUS_BLOCK {
        IO_STATUS_BLOCK_INNER inner;
        ULONG_PTR Information;
    } IO_STATUS_BLOCK, PIO_STATUS_BLOCK;
);

UNION! {
    union IO_STATUS_BLOCK_INNER {
        status: NtStatus,
        pointer: PVOID,
    }
}

define_struct!(
    typedef struct _UNICODE_STRING {
        USHORT Length;
        USHORT MaximumLength;
        PWCH Buffer;
    } UNICODE_STRING, PUNICODE_STRING;
);

define_struct!(
    typedef struct _OBJECT_ATTRIBUTES {
        ULONG Length;
        HANDLE RootDirectory;
        PCUNICODE_STRING ObjectName;
        ULONG Attributes;
        PSECURITY_DESCRIPTOR SecurityDescriptor;
        PSECURITY_QUALITY_OF_SERVICE SecurityQualityOfService;
    } OBJECT_ATTRIBUTES, POBJECT_ATTRIBUTES;
);

define_struct!(
    typedef struct _FILE_BASIC_INFORMATION {
        LARGE_INTEGER CreationTime;
        LARGE_INTEGER LastAccessTime;
        LARGE_INTEGER LastWriteTime;
        LARGE_INTEGER ChangeTime;
        ULONG FileAttributes;
    } FILE_BASIC_INFORMATION, PFILE_BASIC_INFORMATION;
);

define_struct!(
    typedef struct _CURDIR {
        UNICODE_STRING DosPath;
        HANDLE Handle;
    } CURDIR, PCURDIR;
);

define_struct!(
    typedef struct _CLIENT_ID {
        HANDLE UniqueProcess;
        HANDLE UniqueThread;
    } CLIENT_ID, PCLIENT_ID;
);

define_struct!(
    typedef struct _RTLP_CURDIR_REF {
        LONG ReferenceCount;
        HANDLE DirectoryHandle;
    } RTLP_CURDIR_REF, PRTLP_CURDIR_REF;
);

define_struct!(
    typedef struct _RTL_RELATIVE_NAME_U {
        UNICODE_STRING RelativeName;
        HANDLE ContainingDirectory;
        PRTLP_CURDIR_REF CurDirRef;
    } RTL_RELATIVE_NAME_U, PRTL_RELATIVE_NAME_U;
);

define_struct!(
    typedef struct _SYSTEM_THREAD_INFORMATION {
        LARGE_INTEGER KernelTime;
        LARGE_INTEGER UserTime;
        LARGE_INTEGER CreateTime;
        ULONG WaitTime;
        PVOID StartAddress;
        CLIENT_ID ClientId;
        KPRIORITY Priority;
        KPRIORITY BasePriority;
        ULONG ContextSwitches;
        KTHREAD_STATE ThreadState;
        KWAIT_REASON WaitReason;
    } SYSTEM_THREAD_INFORMATION, PSYSTEM_THREAD_INFORMATION;
);

ENUM! {
    enum KTHREAD_STATE {
        Initialized,
        Ready,
        Running,
        Standby,
        Terminated,
        Waiting,
        Transition,
        DeferredReady,
        GateWaitObsolete,
        WaitingForProcessInSwap,
        MaximumThreadState
    }
}

ENUM! {
    enum KWAIT_REASON {
        Executive,
        FreePage,
        PageIn,
        PoolAllocation,
        DelayExecution,
        Suspended,
        UserRequest,
        WrExecutive,
        WrFreePage,
        WrPageIn,
        WrPoolAllocation,
        WrDelayExecution,
        WrSuspended,
        WrUserRequest,
        WrEventPair,
        WrQueue,
        WrLpcReceive,
        WrLpcReply,
        WrVirtualMemory,
        WrPageOut,
        WrRendezvous,
        WrKeyedEvent,
        WrTerminated,
        WrProcessInSwap,
        WrCpuRateControl,
        WrCalloutStack,
        WrKernel,
        WrResource,
        WrPushLock,
        WrMutex,
        WrQuantumEnd,
        WrDispatchInt,
        WrPreempted,
        WrYieldExecution,
        WrFastMutex,
        WrGuardedMutex,
        WrRundown,
        WrAlertByThreadId,
        WrDeferredPreempt,
        WrPhysicalFault,
        WrIoRing,
        WrMdlCache,
        WrRcu,
        MaximumWaitReason
    }
}

ENUM! {
    enum NtProductType {
        WinNt = 1,
        LanManNt = 2,
        Server = 3,
    }
}

ENUM! {
    enum AlternativeArchitectureType {
        StandardDesign,
        NEC98x86,
        EndAlternatives
    }
}
