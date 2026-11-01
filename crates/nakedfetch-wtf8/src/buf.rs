use alloc::vec::Vec;
use core::{ops::Deref, ptr};

use crate::Wtf8;

#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Wtf8Buf(Vec<u8>);

impl Wtf8Buf {
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    #[must_use]
    pub const fn as_wtf8(&self) -> &Wtf8 {
        // SAFETY: Layout of [u8] == Wtf8
        unsafe { &*(ptr::from_ref(self.0.as_slice()) as *const Wtf8) }
    }

    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(&self.0).ok()
    }

    pub fn push_char_buf(&mut self, c: char, buf: &mut [u8; 4]) {
        self.0.extend_from_slice(c.encode_utf8(buf).as_bytes());
    }

    pub fn push_char(&mut self, c: char) {
        let mut buf = [0u8; 4];
        self.push_char_buf(c, &mut buf);
    }

    pub fn push_str(&mut self, s: &str) {
        let mut buf = [0u8; 4];
        for ch in s.chars() {
            self.push_char_buf(ch, &mut buf);
        }
    }

    pub fn push_surrogate(&mut self, surrogate: u16) {
        debug_assert!((0xD800..=0xDFFF).contains(&surrogate));

        let cp = u32::from(surrogate);
        self.0.push(0xE0 | (cp >> 12) as u8);
        self.0.push(0x80 | ((cp >> 6) & 0x3F) as u8);
        self.0.push(0x80 | (cp & 0x3F) as u8);
    }
}

impl Deref for Wtf8Buf {
    type Target = Wtf8;

    fn deref(&self) -> &Self::Target {
        self.as_wtf8()
    }
}

impl From<&str> for Wtf8Buf {
    fn from(s: &str) -> Self {
        Self(s.as_bytes().to_vec())
    }
}

impl From<Vec<u8>> for Wtf8Buf {
    fn from(s: Vec<u8>) -> Self {
        Self(s)
    }
}

impl From<&[u8]> for Wtf8Buf {
    fn from(s: &[u8]) -> Self {
        Self(s.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for Wtf8Buf {
    fn from(bytes: &[u8; N]) -> Self {
        Self::from(&bytes[..])
    }
}

#[cfg(test)]
mod tests {
    use crate::Wtf8Buf;

    #[test]
    fn valid_utf8_for_wtf8_test() {
        let text = "Hello World!";
        let wtf8 = Wtf8Buf::from(text);
        assert_eq!(wtf8.to_str(), Some(text));

        let text = "Привет мир!";
        let wtf8 = Wtf8Buf::from(text);
        assert_eq!(wtf8.to_str(), Some(text));

        let text = "你好世界！";
        let wtf8 = Wtf8Buf::from(text);
        assert_eq!(wtf8.to_str(), Some(text));
    }

    #[test]
    fn bad_utf8_for_wtf8_test() {
        let bytes = b"Hello World! \xED";
        let wtf8 = Wtf8Buf::from(bytes);
        assert_eq!(
            format!("Hello World! {}", char::REPLACEMENT_CHARACTER),
            wtf8.to_string_lossy()
        );

        let mut bytes = "Привет мир! ".as_bytes().to_vec();
        bytes.push(b'\xED');
        let wtf8 = Wtf8Buf::from(bytes);
        assert_eq!(
            format!("Привет мир! {}", char::REPLACEMENT_CHARACTER),
            wtf8.to_string_lossy()
        );

        let mut bytes = "你好世界！ ".as_bytes().to_vec();
        bytes.push(b'\xED');
        let wtf8 = Wtf8Buf::from(bytes);
        assert_eq!(
            format!("你好世界！ {}", char::REPLACEMENT_CHARACTER),
            wtf8.to_string_lossy()
        );
    }

    #[test]
    fn push_char_wtf8buf_test() {
        let mut wtf8 = Wtf8Buf::with_capacity(24);
        wtf8.push_str("Привет");
        wtf8.push_char(' ');
        wtf8.push_str("World");
        wtf8.push_char('!');
        assert_eq!(wtf8.as_str(), Some("Привет World!"));
    }
}
