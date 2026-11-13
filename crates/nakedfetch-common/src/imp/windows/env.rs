use alloc::string::String;
use core::{error::Error, fmt};

use OsStr::{OsStr, OsString};
use path::PathBuf;
use winapi::{
    ntstatus::NtStatus,
    win32::processenv::{GetCommandLine, GetEnvironmentVariable},
};

use crate::sync::OnceLock;

pub fn var_os(name: &OsStr) -> Result<OsString, EnvVarError> {
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

pub fn var(name: &OsStr) -> Result<String, EnvVarError> {
    var_os(name).map(String::to_string_lossy)
}

static COMMAND_LINE_STRING: OnceLock<String> = OnceLock::new();

fn command_line_string() -> &'static str {
    COMMAND_LINE_STRING.get_or_init(|| GetCommandLine().to_string_lossy())
}

#[derive(Debug, Clone)]
pub struct ArgsInner {
    source: &'static str,
    pos: usize,
}

impl ArgsInner {
    #[must_use]
    pub fn new() -> Self {
        Self::with_custom(command_line_string())
    }

    #[must_use]
    pub const fn with_custom(source: &'static str) -> Self {
        Self { source, pos: 0 }
    }

    const fn in_bound(&self) -> bool {
        self.pos < self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    fn peek_unchecked(&self) -> char {
        self.peek()
            .unwrap_or_else(|| unreachable!("Check via `in_bound`"))
    }

    fn pop(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn skip_whitespaces(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.pos += ch.len_utf8();
        }
    }

    fn read_in_quotes_range(&mut self) -> (usize, usize) {
        self.pop();
        let start = self.pos;

        while let Some(ch) = self.peek() {
            if ch == '"' {
                break;
            }
            self.pos += ch.len_utf8();
        }

        let end = self.pos;
        self.pop();
        (start, end)
    }

    fn read_just_text_range(&mut self) -> (usize, usize) {
        let start = self.pos;
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                break;
            }
            self.pos += ch.len_utf8();
        }
        (start, self.pos)
    }

    fn build_str(&self, start: usize, end: usize) -> &'static str {
        let source: &'static str = self.source;
        &source[start..end]
    }
}

impl Iterator for ArgsInner {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();
        if !self.in_bound() {
            return None;
        }

        let (start, end) = if self.peek_unchecked() == '"' {
            self.read_in_quotes_range()
        } else {
            self.read_just_text_range()
        };

        Some(self.build_str(start, end))
    }
}

const LOCALAPPDATA_NAME: &OsStr = OsStr::from_str("LOCALAPPDATA");

pub fn cache() -> PathBuf {
    let mut base = match var(LOCALAPPDATA_NAME) {
        Ok(w) => w,
        Err(e) => panic!("Failed to get APPDATA env: {e}"),
    };
    base.push_str("/nakedfetch/cache");
    base.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! next_is {
        ($args:expr, $lit:literal) => {
            assert_eq!(Some($lit), $args.next().as_deref());
        };
    }

    #[test]
    fn args_check_quotes_test() {
        let mut args = ArgsInner::with_custom("\"Hello\" \"World\" \"\"");
        next_is!(args, "Hello");
        next_is!(args, "World");
        next_is!(args, "");
    }

    #[test]
    fn args_check_all_test() {
        let mut args = ArgsInner::with_custom("git commit -m \"Initial commit\"");
        next_is!(args, "git");
        next_is!(args, "commit");
        next_is!(args, "-m");
        next_is!(args, "Initial commit");
    }

    #[test]
    fn args_ru_test() {
        let mut args = ArgsInner::with_custom("гит коммит -м \"Инициализация\"");
        next_is!(args, "гит");
        next_is!(args, "коммит");
        next_is!(args, "-м");
        next_is!(args, "Инициализация");
    }
}
