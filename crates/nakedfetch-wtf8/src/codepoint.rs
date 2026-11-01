use crate::Wtf8Buf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodePoint {
    Char(char),
    Surrogate(u16),
}

#[derive(Clone)]
pub struct CodePointsIter<'wtf8> {
    bytes: &'wtf8 [u8],
    pos: usize,
}

impl<'wtf8> CodePointsIter<'wtf8> {
    #[must_use]
    pub fn new(wtf8: &'wtf8 Wtf8Buf) -> Self {
        Self {
            bytes: wtf8.as_bytes(),
            pos: 0,
        }
    }

    fn pop(&mut self) -> Option<u8> {
        let val = self.bytes.get(self.pos).copied();
        self.pos += 1;
        val
    }
}

impl Iterator for CodePointsIter<'_> {
    type Item = CodePoint;

    fn next(&mut self) -> Option<Self::Item> {
        let first_byte = self.pop()?;
        if first_byte < 0x80 {
            // U+0000..U+007F
            let cp = CodePoint::Char(char::from(first_byte));
            Some(cp)
        } else if (0xC2..=0xDF).contains(&first_byte) {
            // U+0080..U+07FF
            let second_byte = self.pop()?;
            if second_byte & 0xC0 != 0x80 {
                return None;
            }

            let val = ((u32::from(first_byte) & 0x1F) << 6) | (u32::from(second_byte) & 0x3F);
            let cp = CodePoint::Char(char::from_u32(val)?);
            Some(cp)
        } else if (0xE0..=0xEF).contains(&first_byte) {
            // U+0800..U+FFFF
            let second_byte = self.pop()?;
            let third_byte = self.pop()?;

            if second_byte & 0xC0 != 0x80 || third_byte & 0xC0 != 0x80 {
                return None;
            }

            if first_byte == 0xE0 && second_byte < 0xA0 {
                return None;
            }

            let val = ((u32::from(first_byte) & 0x0F) << 12)
                | ((u32::from(second_byte) & 0x3F) << 6)
                | (u32::from(third_byte) & 0x3F);

            let cp = if (0xD800..=0xDFFF).contains(&val) {
                CodePoint::Surrogate(val as u16)
            } else {
                CodePoint::Char(char::from_u32(val)?)
            };
            Some(cp)
        } else if (0xF0..=0xF4).contains(&first_byte) {
            // U+10000..U+10FFFF
            let second_byte = self.pop()?;
            let third_byte = self.pop()?;
            let fourth_byte = self.pop()?;

            if second_byte & 0xC0 != 0x80 || third_byte & 0xC0 != 0x80 || fourth_byte & 0xC0 != 0x80
            {
                return None;
            }

            if first_byte == 0xF0 && second_byte < 0x90 {
                return None;
            }
            if first_byte == 0xF4 && second_byte > 0x8F {
                return None;
            }

            let val = ((u32::from(first_byte) & 0x07) << 18)
                | ((u32::from(second_byte) & 0x3F) << 12)
                | ((u32::from(third_byte) & 0x3F) << 6)
                | (u32::from(fourth_byte) & 0x3F);

            let cp = CodePoint::Char(char::from_u32(val)?);
            Some(cp)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.bytes.len();
        (len / 4, Some(len))
    }
}

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
