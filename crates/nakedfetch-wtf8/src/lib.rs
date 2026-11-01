#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod buf;
use core::ptr;

pub use buf::Wtf8Buf;

mod codepoint;
pub use codepoint::CodePointsIter;

mod utf16;
use alloc::string::String;

pub use utf16::Utf16Iter;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Wtf8([u8]);

impl Wtf8 {
    #[must_use]
    pub const fn from_bytes(bytes: &[u8]) -> &Self {
        // SAFETY: Layout of [u8] == Wtf8
        unsafe { &*(ptr::from_ref(bytes) as *const Self) }
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    pub fn to_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.0).ok()
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }
}

impl<'a> From<&'a str> for &'a Wtf8 {
    fn from(s: &'a str) -> Self {
        Wtf8::from_bytes(s.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for &'a Wtf8 {
    fn from(s: &'a [u8]) -> Self {
        Wtf8::from_bytes(s)
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for &'a Wtf8 {
    fn from(bytes: &'a [u8; N]) -> Self {
        Self::from(&bytes[..])
    }
}
