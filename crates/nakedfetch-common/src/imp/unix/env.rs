use alloc::string::String;

use libc::stdio;
use path::{OsStr, OsString};

use crate::env::EnvVarError;

pub fn var_os(name: &OsStr) -> Result<OsString, EnvVarError> {
    stdio::getenv(name).ok_or(EnvVarError::NotFound)
}

pub fn var(name: &OsStr) -> Result<String, EnvVarError> {
    stdio::getenv(name)
        .map(String::from_utf8_lossy_owned)
        .ok_or(EnvVarError::NotFound)
}
