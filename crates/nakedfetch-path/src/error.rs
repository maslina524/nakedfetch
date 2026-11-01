use alloc::string::String;
use core::{error::Error, fmt};

use crate::{Components, INVALID_NAMES, INVALID_SYMBOLS};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathBuildError {
    InvalidSymbol(char),
    InvalidName(String),
}

impl fmt::Display for PathBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for PathBuildError {}

pub fn check(comps: Components<'_>) -> Result<(), PathBuildError> {
    fn check_valid_chars(comp: &str, is_first: &mut bool) -> Result<(), PathBuildError> {
        for ch in comp.chars() {
            if !*is_first && ch == ':' {
                return Err(PathBuildError::InvalidSymbol(ch));
            }

            if INVALID_SYMBOLS.contains(&ch) {
                return Err(PathBuildError::InvalidSymbol(ch));
            }
            if (0..=31).contains(&u32::from(ch)) {
                return Err(PathBuildError::InvalidSymbol(ch));
            }
        }
        *is_first = false;
        Ok(())
    }

    fn matches_ignore_case(s: &str, list: &[&str]) -> bool {
        list.iter().any(|x| x.eq_ignore_ascii_case(s))
    }

    let mut is_first = true;
    for comp in comps {
        check_valid_chars(comp, &mut is_first)?;

        let slice = comp.rfind('.').map_or(comp, |idx| &comp[..idx]);
        if matches_ignore_case(slice, &INVALID_NAMES) {
            return Err(PathBuildError::InvalidName(comp.into()));
        }
    }
    Ok(())
}
