use crate::Wtf8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodePoint {
    Char(char),
    Surrogate(u16),
}

impl CodePoint {
    #[must_use]
    pub const fn wtf8_len(self) -> usize {
        match self {
            Self::Surrogate(_) => 3,
            Self::Char(c) => c.len_utf8(),
        }
    }
}

impl Default for CodePoint {
    fn default() -> Self {
        Self::Char(char::from(0))
    }
}

#[derive(Clone)]
pub struct CodePointsIter<'wtf8> {
    bytes: &'wtf8 [u8],
    pos: usize,
    end: usize,
}

impl<'wtf8> CodePointsIter<'wtf8> {
    #[must_use]
    pub fn new<T: AsRef<Wtf8> + ?Sized>(wtf8: &'wtf8 T) -> Self {
        let bytes = wtf8.as_ref().as_bytes();
        Self {
            bytes,
            pos: 0,
            end: bytes.len(),
        }
    }

    fn decode_at(&self, start: usize) -> Option<(CodePoint, usize)> {
        let bytes = self.bytes;
        let max = self.end;
        let first = *bytes.get(start).filter(|_| start < max)?;

        if first < 0x80 {
            return Some((CodePoint::Char(first as char), 1));
        }
        if (0xC2..=0xDF).contains(&first) {
            let b1 = start + 1;
            if b1 >= max {
                return None;
            }
            let s1 = bytes[b1];
            if s1 & 0xC0 != 0x80 {
                return None;
            }
            let val = ((u32::from(first) & 0x1F) << 6) | (u32::from(s1) & 0x3F);
            return Some((CodePoint::Char(char::from_u32(val)?), 2));
        }
        if (0xE0..=0xEF).contains(&first) {
            let b2 = start + 2;
            if b2 >= max {
                return None;
            }
            let s1 = bytes[start + 1];
            let s2 = bytes[b2];

            if s1 & 0xC0 != 0x80 || s2 & 0xC0 != 0x80 {
                return None;
            }
            if first == 0xE0 && s1 < 0xA0 {
                return None;
            }

            let val = ((u32::from(first) & 0x0F) << 12)
                | ((u32::from(s1) & 0x3F) << 6)
                | (u32::from(s2) & 0x3F);
            let cp = if (0xD800..=0xDFFF).contains(&val) {
                CodePoint::Surrogate(val as u16)
            } else {
                CodePoint::Char(char::from_u32(val)?)
            };

            return Some((cp, 3));
        }
        if (0xF0..=0xF4).contains(&first) {
            let b3 = start + 3;
            if b3 >= max {
                return None;
            }
            let s1 = bytes[start + 1];
            let s2 = bytes[start + 2];
            let s3 = bytes[b3];

            if s1 & 0xC0 != 0x80 || s2 & 0xC0 != 0x80 || s3 & 0xC0 != 0x80 {
                return None;
            }
            if first == 0xF0 && s1 < 0x90 {
                return None;
            }
            if first == 0xF4 && s1 > 0x8F {
                return None;
            }

            let val = ((u32::from(first) & 0x07) << 18)
                | ((u32::from(s1) & 0x3F) << 12)
                | ((u32::from(s2) & 0x3F) << 6)
                | (u32::from(s3) & 0x3F);
            return Some((CodePoint::Char(char::from_u32(val)?), 4));
        }
        None
    }
}

impl Iterator for CodePointsIter<'_> {
    type Item = CodePoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.end {
            return None;
        }
        if let Some((cp, len)) = self.decode_at(self.pos) {
            self.pos += len;
            Some(cp)
        } else {
            self.pos = self.end;
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.pos);
        (remaining.div_ceil(4), Some(remaining))
    }
}

impl DoubleEndedIterator for CodePointsIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.pos >= self.end {
            return None;
        }

        let mut start = self.end - 1;
        while start > self.pos && (self.bytes[start] & 0xC0) == 0x80 {
            start -= 1;
        }

        match self.decode_at(start) {
            Some((cp, len)) if start + len == self.end => {
                self.end = start;
                Some(cp)
            },
            _ => {
                self.pos = self.end;
                None
            },
        }
    }
}

impl core::iter::FusedIterator for CodePointsIter<'_> {}

#[cfg(test)]
mod tests {
    use crate::{CodePointsIter, Wtf8Buf, codepoint::CodePoint};

    #[test]
    fn valid_utf8_code_points_test() {
        let wtf8 = Wtf8Buf::from("Hello World!");
        let iter = CodePointsIter::new(&wtf8);
        for codepoint in iter {
            assert!(matches!(codepoint, CodePoint::Char(_)));
        }

        let wtf8 = Wtf8Buf::from("Привет мир!");
        let iter = CodePointsIter::new(&wtf8);
        for codepoint in iter {
            assert!(matches!(codepoint, CodePoint::Char(_)));
        }
    }

    #[test]
    fn surrogate_code_points_test() {
        let wtf8 = Wtf8Buf::from(b"\xED\xA0\x80");
        let v: Vec<_> = CodePointsIter::new(&wtf8).collect();
        assert!(matches!(v[..], [CodePoint::Surrogate(0xD800)]));

        for hi in 0xD800u32..=0xDFFF {
            let b = [
                0xE0 | ((hi >> 12) as u8),
                0x80 | (((hi >> 6) & 0x3F) as u8),
                0x80 | ((hi & 0x3F) as u8),
            ];
            let wtf8 = Wtf8Buf::from(&b[..]);
            let v: Vec<_> = CodePointsIter::new(&wtf8).collect();
            assert_eq!(v, vec![CodePoint::Surrogate(hi as u16)]);
        }

        let wtf8 = Wtf8Buf::from(b"Hi\xED\xA0\x80!\xC3\xA9");
        let v: Vec<_> = CodePointsIter::new(&wtf8).collect();
        assert_eq!(
            v,
            vec![
                CodePoint::Char('H'),
                CodePoint::Char('i'),
                CodePoint::Surrogate(0xD800),
                CodePoint::Char('!'),
                CodePoint::Char('é'),
            ]
        );
    }
}
