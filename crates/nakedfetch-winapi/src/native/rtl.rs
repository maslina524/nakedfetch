use crate::{
    types::{CurDir, RawUnicodeString, primitives::ULong, ptr::VoidPtr},
    win32::console::ConsoleHandle,
};

STRUCT! {
    Naked
    struct RtlUserProcessParameters {
        maximum_length: ULong,
        length: ULong,

        flags: ULong,
        debug_flags: ULong,

        console_handle: ConsoleHandle,
        console_flags: ULong,
        standard_input: ConsoleHandle,
        standard_output: ConsoleHandle,
        standard_error: ConsoleHandle,

        current_directory: CurDir,
        dll_path: RawUnicodeString,
        image_path_name: RawUnicodeString,
        command_line: RawUnicodeString,
        environment: VoidPtr,
    }
}
