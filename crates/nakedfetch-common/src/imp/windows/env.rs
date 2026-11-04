use core::{error::Error, fmt};

use winapi::{ntstatus::NtStatus, win32::processenv::GetEnvironmentVariable};
use wtf8::{Wtf8, Wtf8Buf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnvVarError {
    NotFound,
    InvalidParam,
    AccessDenied,
    BufferTooSmall,
    TooManyVariables,
    NameTooLong,
    NoSystemResource,
    Unknown(NtStatus),
}

impl fmt::Display for EnvVarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for EnvVarError {}

pub fn var(name: &Wtf8) -> Result<Wtf8Buf, EnvVarError> {
    const fn map_ntstatus(nt: NtStatus) -> EnvVarError {
        use super::env::EnvVarError as EVE;

        match nt.into_inner() {
            0xC000_0100 => EVE::NotFound,
            0xC000_000D => EVE::InvalidParam,
            0xC000_0022 => EVE::AccessDenied,
            0xC000_0023 => EVE::BufferTooSmall,
            0xC000_00B9 => EVE::TooManyVariables,
            0xC000_0106 => EVE::NameTooLong,
            0xC000_009A | 0xC000_0017 => EVE::NoSystemResource,
            _ => EVE::Unknown(nt),
        }
    }

    GetEnvironmentVariable(name).map_err(map_ntstatus)
}
