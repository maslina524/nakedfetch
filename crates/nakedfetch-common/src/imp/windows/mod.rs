pub mod alloc;
pub mod env;
pub mod fs;
pub(crate) mod handle;
pub mod io;

pub type RawError = winapi::ntstatus::NtStatus;
