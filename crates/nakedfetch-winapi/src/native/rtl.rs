use super::{CONSOLE_HANDLE, CURDIR, HANDLE, PVOID, ULONG, UNICODE_STRING};

define_struct!(
    typedef struct RtlUserProcessParameters {
        ULONG MaximumLength;
        ULONG Length;

        ULONG Flags; // RTL_USER_PROC_FLAGS
        ULONG DebugFlags; // RTL_USER_DEBUG_FLAGS

        HANDLE ConsoleHandle;
        ULONG ConsoleFlags; // RTL_USER_PROC_CONSOLE_FLAGS
        CONSOLE_HANDLE StandardInput;
        CONSOLE_HANDLE StandardOutput;
        CONSOLE_HANDLE StandardError;

        CURDIR CurrentDirectory;
        UNICODE_STRING DllPath;
        UNICODE_STRING ImagePathName;
        UNICODE_STRING CommandLine;
        PVOID Environment;
    };
);
