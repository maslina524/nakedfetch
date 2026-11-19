use std::{error::Error, ffi::OsStr, fmt};

use crate::target::Target;

#[derive(Debug)]
pub struct LibcUnsupported {
    os: &'static OsStr,
    arch: &'static OsStr,
}

impl LibcUnsupported {
    pub const fn new(os: &'static OsStr, arch: &'static OsStr) -> Self {
        Self { os, arch }
    }
}

impl fmt::Display for LibcUnsupported {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Libc is not supported on your platform ({} {})",
            self.os.display(),
            self.arch.display()
        )
    }
}

impl Error for LibcUnsupported {}

pub fn link_libc() -> Result<(), LibcUnsupported> {
    let os = Target::os();
    let arch = Target::arch();

    let os_bytes = os.as_encoded_bytes();
    let arch_bytes = arch.as_encoded_bytes();

    match (os_bytes, arch_bytes) {
        (b"linux", b"x86_64") => {
            link_arg!("dylib=dl");
            link_arg!("dylib=m");
            // link_arg!("-pthread");
            // link_arg!("-lc");
        },
        (b"android", b"aarch64") => {
            link_arg!("-Wl,--no-as-needed");
            link_arg!("-lc");
            link_arg!("dylib=m");
            link_arg!("dylib=dl");
        },
        (b"windows", _) => {
            // FIXME: This just suppresses the error rather than solving it,
            // in the future Lua should be built manually for the linker
            // that Rust uses
            link_arg!("/NODEFAULTLIB:LIBCMT");
        },
        _ => return Err(LibcUnsupported::new(os, arch)),
    }
    Ok(())
}
