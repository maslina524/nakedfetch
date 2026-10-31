use crate::{types::primitives::ULong, win32::console::ConsoleHandle};

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
    }
}
