use crate::{CodePointsIter, Wtf8, codepoint::CodePoint};

#[derive(Clone, PartialEq, Eq)]
enum NextAction {
    None,
    CharU16Part(u16),
    NonZero,
}

#[derive(Clone)]
pub struct Utf16Iter<'wtf8> {
    codepoints: CodePointsIter<'wtf8>,
    next: NextAction,
}

impl<'wtf8> Utf16Iter<'wtf8> {
    #[must_use]
    pub fn new<T: AsRef<Wtf8>>(wtf8: &'wtf8 T) -> Self {
        Self {
            codepoints: CodePointsIter::new(wtf8),
            next: NextAction::None,
        }
    }
}

impl Iterator for Utf16Iter<'_> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if let NextAction::CharU16Part(p) = self.next {
            self.next = NextAction::None;
            if p != 0 {
                return Some(p);
            }
        }

        match self.codepoints.next() {
            Some(CodePoint::Char(ch)) => {
                let mut buf = [0u16; 2];
                ch.encode_utf16(&mut buf);
                self.next = NextAction::CharU16Part(buf[1]);
                Some(buf[0])
            },
            Some(CodePoint::Surrogate(s)) => Some(s),
            None => {
                if matches!(self.next, NextAction::NonZero) {
                    None
                } else {
                    self.next = NextAction::NonZero;
                    Some(0)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Utf16Iter, Wtf8Buf};

    fn build_utf16_std(s: &str) -> Vec<u16> {
        let iter = s.encode_utf16();
        let mut buf: Vec<u16> = iter.collect();
        buf.push(0);
        buf
    }

    #[test]
    fn wtf8_to_utf16_test() {
        let text = "Hello World!";
        let wtf8 = Wtf8Buf::from(text);
        let utf16: Vec<u16> = Utf16Iter::new(&wtf8).collect();
        let std = build_utf16_std(text);
        assert_eq!(utf16, std);

        let text = "Привет мир";
        let wtf8 = Wtf8Buf::from(text);
        let utf16: Vec<u16> = Utf16Iter::new(&wtf8).collect();
        let std = build_utf16_std(text);
        assert_eq!(utf16, std);

        let wtf8 = Wtf8Buf::from(b"Hi\xED\xA0\x80!\xC3\xA9");
        let utf16: Vec<u16> = Utf16Iter::new(&wtf8).collect();
        assert_eq!(
            utf16,
            vec![
                u16::from(b'H'),
                u16::from(b'i'),
                0xD800,
                u16::from(b'!'),
                0x00E9,
                0
            ]
        );
    }
}
