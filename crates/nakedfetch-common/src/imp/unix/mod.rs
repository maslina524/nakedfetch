pub mod fs;
pub(crate) mod handle;
pub mod io;

pub type RawError = libc::errno::Errno;
