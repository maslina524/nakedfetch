#[macro_use]
mod macros;

mod link;
mod setup_cfg;
mod setup_env;
mod target;
mod windows;

use std::{error::Error, fmt};

use target::Target;

struct MainError(Box<dyn Error>);

impl<E: Into<Box<dyn Error>>> From<E> for MainError {
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl fmt::Debug for MainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

fn main() -> Result<(), MainError> {
    if Target::os() == "windows" {
        windows::setup_resource()?;
    }

    link::libc()?;
    link::lua_lib()?;

    setup_env::target();
    setup_env::target_os();
    setup_env::target_arch();
    setup_env::compile_time();
    setup_env::rustc_version()?;
    setup_env::llvm_version()?;

    setup_cfg::libc()?;
    setup_cfg::no_alloc_in_panic();

    Ok(())
}
