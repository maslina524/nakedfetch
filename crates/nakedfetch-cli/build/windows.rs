use std::{error::Error, fmt, io};

#[derive(Debug)]
pub struct WinResError(io::Error);

impl WinResError {
    pub const fn new(e: io::Error) -> Self {
        Self(e)
    }
}

impl From<io::Error> for WinResError {
    fn from(e: io::Error) -> Self {
        Self::new(e)
    }
}

impl fmt::Display for WinResError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to install Windows resource: {}", self.0)
    }
}

impl Error for WinResError {}

pub fn setup_resource() -> Result<(), WinResError> {
    use winresource::WindowsResource;

    let res = WindowsResource::new();
    res.compile()?; // io::Error -> WinResError
    Ok(())
}
