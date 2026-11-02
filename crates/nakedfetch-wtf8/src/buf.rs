use alloc::{string::String, vec::Vec};
use core::{
    fmt::{self, Write as _},
    ops::Deref,
    ptr,
};

use crate::{Wtf8, codepoint::CodePoint};

#[repr(transparent)]
#[derive(Default, Clone, PartialEq, Eq)]
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

    #[must_use]
    pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        (&raw mut self.0).cast()
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

    pub fn push_wtf8<T: AsRef<Wtf8>>(&mut self, s: &T) {
        self.0.extend(&s.as_ref().0);
    }

    pub fn push_surrogate(&mut self, surrogate: u16) {
        debug_assert!((0xD800..=0xDFFF).contains(&surrogate));

        let cp = u32::from(surrogate);
        self.0.push(0xE0 | (cp >> 12) as u8);
        self.0.push(0x80 | ((cp >> 6) & 0x3F) as u8);
        self.0.push(0x80 | (cp & 0x3F) as u8);
    }

    pub fn pop(&mut self) -> Option<CodePoint> {
        let cp = self.codepoints().next_back()?;
        let len = self.len() - cp.wtf8_len();
        self.truncate(len);
        Some(cp)
    }

    pub fn pop_n<const N: usize>(&mut self) -> Option<[CodePoint; N]> {
        let mut ret = [CodePoint::Char('\0'); N];
        let mut new_len = self.0.len();

        {
            let mut iter = self.codepoints();

            for i in (0..N).rev() {
                let cp = iter.next_back()?;
                new_len -= cp.wtf8_len();
                ret[i] = cp;
            }
        }

        self.truncate(new_len);
        Some(ret)
    }

    #[must_use]
    pub fn ends_with_char(&self, ch: char) -> bool {
        let Some(cp) = self.codepoints().next_back() else {
            return false;
        };
        match cp {
            CodePoint::Char(cp_char) => cp_char == ch,
            CodePoint::Surrogate(_) => false,
        }
    }

    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
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

impl From<String> for Wtf8Buf {
    fn from(s: String) -> Self {
        Self(s.into_bytes())
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

impl fmt::Display for Wtf8Buf {
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

impl fmt::Debug for Wtf8Buf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Wtf8Buf, codepoint::CodePoint};

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

    #[test]
    fn pop_wtf8buf_test() {
        let mut wtf8 = Wtf8Buf::from("Привет Мир 👋");

        assert_eq!(wtf8.pop(), Some(CodePoint::Char('👋')));
        assert_eq!(wtf8.as_str(), Some("Привет Мир "));

        assert_eq!(wtf8.pop(), Some(CodePoint::Char(' ')));
        assert_eq!(wtf8.as_str(), Some("Привет Мир"));

        assert_eq!(
            wtf8.pop_n::<3>(),
            Some([
                CodePoint::Char('М'),
                CodePoint::Char('и'),
                CodePoint::Char('р')
            ])
        );
        assert_eq!(wtf8.as_str(), Some("Привет "));

        assert_eq!(wtf8.pop(), Some(CodePoint::Char(' ')));
        assert_eq!(wtf8.as_str(), Some("Привет"));
    }

    #[test]
    fn display_for_pathbuf_test() {
        let path = Wtf8Buf::from(b"C:/Hello/World\xED");
        let formatted = format!("{path}");
        assert_eq!(
            formatted,
            format!("C:/Hello/World{}", char::REPLACEMENT_CHARACTER)
        );
    }

    #[test]
    fn debug_for_pathbuf_test() {
        let path = Wtf8Buf::from(b"C:/Hello/World\xED");
        let formatted = format!("{path:?}");
        assert_eq!(
            formatted,
            format!("\"C:/Hello/World{}\"", char::REPLACEMENT_CHARACTER)
        );
    }
}
