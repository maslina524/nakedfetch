#[allow(
    clippy::wildcard_imports,
    reason = "There's no point in importing them separately"
)]
use super::*;

define_struct!(
    typedef struct _RTL_USER_PROCESS_PARAMETERS {
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
    } RTL_USER_PROCESS_PARAMETERS;
);

pub type PRTL_USER_PROCESS_PARAMETERS = &'static RTL_USER_PROCESS_PARAMETERS;
