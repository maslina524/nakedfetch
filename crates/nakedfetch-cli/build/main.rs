#[macro_use]
mod macros;

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

    Ok(())
}
