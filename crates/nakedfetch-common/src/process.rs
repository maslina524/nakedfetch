use crate::imp::process;

pub fn exit(code: i32) -> ! {
    process::terminate_self(code)
}
