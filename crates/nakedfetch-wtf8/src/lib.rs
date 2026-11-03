#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod buf;
mod codepoint;
mod smol;
mod utf16;

use alloc::{string::String, sync::Arc};
use core::{
    fmt::{self, Write as _},
    mem,
    ops::Index,
    ptr,
};

pub use buf::Wtf8Buf;
pub use codepoint::{CodePoint, CodePointsIter};
pub use smol::SmolWtf8;
pub use utf16::*;

#[repr(transparent)]
pub struct Wtf8([u8]);

impl Wtf8 {
    #[must_use]
    pub const fn from_bytes(bytes: &[u8]) -> &Self {
        // SAFETY: Layout of [u8] == Wtf8
        unsafe { &*(ptr::from_ref(bytes) as *const Self) }
    }

    #[must_use]
    pub const fn from_str(s: &str) -> &Self {
        Self::from_bytes(s.as_bytes())
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const u8 {
        (&raw const self.0).cast()
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

    #[must_use]
    pub fn find(&self, cp: CodePoint) -> Option<usize> {
        for (i, icp) in self.codepoints().enumerate() {
            if icp == cp {
                return Some(i);
            }
        }
        None
    }

    #[must_use]
    pub fn rfind(&self, cp: CodePoint) -> Option<usize> {
        let mut result = None;
        for (i, icp) in self.codepoints().enumerate() {
            if icp == cp {
                result = Some(i);
            }
        }
        result
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

impl From<&Wtf8> for Arc<Wtf8> {
    fn from(value: &Wtf8) -> Self {
        let arc_slice: Arc<[u8]> = Arc::from(&value.0[..]);
        // SAFETY: Layout of [u8] == Wtf8
        unsafe { mem::transmute::<Arc<[u8]>, Self>(arc_slice) }
    }
}

impl<I> Index<I> for Wtf8
where
    [u8]: Index<I, Output = [u8]>,
{
    type Output = Self;

    fn index(&self, index: I) -> &Self {
        self.0[index].as_ref()
    }
}

impl fmt::Display for Wtf8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.0.utf8_chunks();
        for chunk in iter {
            f.write_str(chunk.valid())?;
            if !chunk.invalid().is_empty() {
                f.write_char(char::REPLACEMENT_CHARACTER)?;
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Wtf8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl PartialEq for Wtf8 {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<SmolWtf8> for Wtf8 {
    fn eq(&self, other: &SmolWtf8) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<Wtf8Buf> for Wtf8 {
    fn eq(&self, other: &Wtf8Buf) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for Wtf8 {}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;

    use crate::{CodePoint, Wtf8};

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

    #[test]
    fn display_for_wtf8_test() {
        let path = Wtf8::from_bytes(b"C:/Hello/World\xED");
        let formatted = format!("{path}");
        assert_eq!(
            formatted,
            format!("C:/Hello/World{}", char::REPLACEMENT_CHARACTER)
        );
    }

    #[test]
    fn debug_for_wtf8_test() {
        let path = Wtf8::from_bytes(b"C:/Hello/World\xED");
        let formatted = format!("{path:?}");
        assert_eq!(
            formatted,
            format!("\"C:/Hello/World{}\"", char::REPLACEMENT_CHARACTER)
        );
    }

    #[test]
    fn arc_wtf8_test() {
        let path = Wtf8::from_bytes(b"C:/Hello/World");

        let arc: Arc<Wtf8> = path.into();
        let copied = arc.clone();

        assert_eq!(path, arc.as_ref());
        assert_eq!(path, copied.as_ref());
    }

    #[test]
    fn rfind_test() {
        let wtf8 = Wtf8::from_bytes(b"Hello");
        let l = CodePoint::Char('l');
        let h = CodePoint::Char('H');

        assert_eq!(wtf8.find(l), Some(2));
        assert_eq!(wtf8.rfind(l), Some(3));
        assert_eq!(wtf8.find(h), Some(0));
        assert_eq!(wtf8.rfind(h), Some(0));
    }
}
