use alloc::{boxed::Box, string::String, vec::Vec};
use core::{ffi::CStr, sync::atomic::Ordering};

use libc::{stdio, unistd};
use path::{OsStr, OsString};

use crate::{env::EnvVarError, sync::OnceLock};

pub fn var_os(name: &OsStr) -> Result<OsString, EnvVarError> {
    stdio::getenv(name).ok_or(EnvVarError::NotFound)
}

pub fn var(name: &OsStr) -> Result<String, EnvVarError> {
    stdio::getenv(name)
        .map(String::from_utf8_lossy_owned)
        .ok_or(EnvVarError::NotFound)
}

static ARGS_VEC: OnceLock<Vec<&str>> = OnceLock::new();

fn args_vec() -> &'static [&'static str] {
    ARGS_VEC.get_or_init(|| {
        let arg_count = unistd::ARGC.load(Ordering::Relaxed);
        let arg_values = unistd::ARGV.load(Ordering::Relaxed);

        let mut buf = Vec::with_capacity(arg_count);
        for i in 0..arg_count {
            let arg_ptr = unsafe { arg_values.add(i).read() };
            if arg_ptr.is_null() {
                break;
            }

            // SAFETY: POSIX returns valid pointers
            let c_str = unsafe { CStr::from_ptr(arg_ptr) };
            let boxed = c_str.to_string_lossy().into_owned().into_boxed_str();
            let leaked: &'static str = Box::leak(boxed);

            buf.push(leaked);
        }

        buf
    })
}

#[derive(Debug, Clone)]
pub struct ArgsInner {
    pos: usize,
}

impl ArgsInner {
    #[must_use]
    pub fn new() -> Self {
        Self { pos: 0 }
    }
}

impl Iterator for ArgsInner {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let val = args_vec().get(self.pos).copied()?;
        self.pos += 1;
        Some(val)
    }
}
