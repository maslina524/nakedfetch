#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

use alloc::string::String;

#[cfg(target_os = "windows")]
use wtf8::Utf16Iter;

#[cfg(target_os = "windows")]
type PathInner = wtf8::Wtf8;

#[derive(Debug, PartialEq, Eq)]
pub struct Path(PathInner);

impl Path {
    #[must_use]
    pub const fn as_inner(&self) -> &PathInner {
        &self.0
    }

    #[must_use]
    #[cfg(target_os = "windows")]
    pub fn as_utf16(&'_ self) -> Utf16Iter<'_> {
        self.0.utf16_with_zero()
    }

    #[must_use]
    pub fn to_str(&self) -> Option<&str> {
        str::from_utf8(self.0.as_bytes()).ok()
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.0.as_bytes()).into_owned()
    }
}
