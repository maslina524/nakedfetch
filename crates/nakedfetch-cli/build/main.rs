#[macro_use]
mod macros;

mod codegen;
mod commit;
mod link;
mod logo;
mod setup_cfg;
mod setup_env;
mod target;
mod windows;

use std::{error::Error, fmt};

use commit::Commit;
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
    println!("cargo:rerun-if-changed=force-rebuild-always");

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

    let commit = Commit::read_git()?;
    setup_env::commit(&commit);

    setup_cfg::libc()?;
    setup_cfg::no_alloc_in_panic();

    let (raw, enc) = logo::compress()?;
    #[allow(
        clippy::cast_precision_loss,
        reason = "It will work correctly as long as encoded and raw are less than 2^52 bytes"
    )]
    {
        println!(
            "cargo:warning={}b -> {}b = {:.02}%",
            raw,
            enc,
            (enc as f64 / raw as f64).mul_add(-100.0, 100.0)
        );
    }
    codegen::generate_logo()?;

    Ok(())
}
