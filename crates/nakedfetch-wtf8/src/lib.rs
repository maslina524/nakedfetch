#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod buf;
mod codepoint;
mod utf16;

use alloc::string::String;
use core::ptr;

pub use buf::Wtf8Buf;
pub use codepoint::CodePointsIter;
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
        str::from_utf8(&self.0).ok()
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }

    #[must_use]
    pub fn codepoints(&'_ self) -> CodePointsIter<'_> {
        CodePointsIter::new(self)
    }

    #[must_use]
    pub fn utf16_with_zero(&'_ self) -> Utf16Iter<'_> {
        Utf16Iter::new(self)
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<Self> for Wtf8 {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Wtf8> for Wtf8Buf {
    fn as_ref(&self) -> &Wtf8 {
        self
    }
}

impl AsRef<Wtf8> for str {
    fn as_ref(&self) -> &Wtf8 {
        Wtf8::from_bytes(self.as_bytes())
    }
}

impl AsRef<Wtf8> for [u8] {
    fn as_ref(&self) -> &Wtf8 {
        Wtf8::from_bytes(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::Wtf8;

    #[test]
    fn valid_utf8_for_wtf8_test() {
        let text = "Hello World!";
        let wtf8 = Wtf8::from_bytes(text.as_bytes());
        assert_eq!(wtf8.to_str(), Some(text));

        let text = "Привет мир!";
        let wtf8: &Wtf8 = text.as_ref();
        assert_eq!(wtf8.to_str(), Some(text));

        let text = "你好世界！";
        let wtf8: &Wtf8 = text.as_ref();
        assert_eq!(wtf8.to_str(), Some(text));
    }
}
