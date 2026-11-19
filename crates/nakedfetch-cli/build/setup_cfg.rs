use std::{env, error::Error, ffi::OsStr, fmt};

use crate::target::Target;

static NO_ALLOC_IN_PANIC_VARIANTS: [&str; 4] = ["1", "yes", "true", "enable"];

#[derive(Debug)]
pub struct UnknownLibc(&'static OsStr);

impl UnknownLibc {
    pub const fn new(s: &'static OsStr) -> Self {
        Self(s)
    }
}

impl fmt::Display for UnknownLibc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to determine the version of libc for your system ({})",
            self.0.display()
        )
    }
}

impl Error for UnknownLibc {}

pub fn libc() -> Result<(), UnknownLibc> {
    let os = Target::os();
    let os_bytes = os.as_encoded_bytes();

    match os_bytes {
        b"linux" => set_cfg!("glibc"),
        b"android" => set_cfg!("bionic"),
        b"windows" => {},
        _ => return Err(UnknownLibc::new(os)),
    }

    Ok(())
}

pub fn no_alloc_in_panic() {
    let Ok(env) = env::var("NO_ALLOC_IN_PANIC") else {
        return;
    };

    let lowercase_env = env.to_ascii_lowercase();
    let lowercase_str = lowercase_env.as_str();
    if NO_ALLOC_IN_PANIC_VARIANTS.contains(&lowercase_str) {
        warning!("Heap allocations are completely disabled during a panic!");
        set_cfg!("no_alloc_in_panic");
    }
}
