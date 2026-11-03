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
    pub fn new<T: AsRef<Wtf8> + ?Sized>(wtf8: &'wtf8 T) -> Self {
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

pub struct DecodeUtf16Iter<I>
where
    I: Iterator<Item = u16>,
{
    iter: I,
    next: Option<u16>,
}

impl<I: Iterator<Item = u16>> DecodeUtf16Iter<I> {
    #[must_use]
    pub const fn new(iter: I) -> Self {
        Self { iter, next: None }
    }
}

impl<I: Iterator<Item = u16>> Iterator for DecodeUtf16Iter<I> {
    type Item = CodePoint;

    fn next(&mut self) -> Option<Self::Item> {
        let first = match self.next.take() {
            Some(unit) => unit,
            None => self.iter.next()?,
        };

        if !(0xD800..=0xDFFF).contains(&first) {
            // SAFETY: Valid char
            let ch = unsafe { char::from_u32_unchecked(u32::from(first)) };
            return Some(CodePoint::Char(ch));
        }

        if (0xDC00..=0xDFFF).contains(&first) {
            return Some(CodePoint::Surrogate(first));
        }

        let Some(second) = self.next.take().or_else(|| self.iter.next()) else {
            return Some(CodePoint::Surrogate(first));
        };

        if (0xDC00..=0xDFFF).contains(&second) {
            let hi = u32::from(first) - 0xD800;
            let lo = u32::from(second) - 0xDC00;
            let scalar = 0x10000 + (hi << 10) + lo;
            // SAFETY: Valid scalar
            let ch = unsafe { char::from_u32_unchecked(scalar) };
            Some(CodePoint::Char(ch))
        } else {
            self.next = Some(second);
            Some(CodePoint::Surrogate(first))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let extra = usize::from(self.next.is_some());

        let (lo, hi) = self.iter.size_hint();
        let lower = lo.saturating_add(extra).div_ceil(2);
        let upper = hi.and_then(|h| h.checked_add(extra));

        (lower, upper)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CodePoint, Utf16Iter, Wtf8Buf, utf16::DecodeUtf16Iter};

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

    #[test]
    fn decode_utf16_test() {
        let units = [0xD83D, 0xDE00, 0xD800, 0x0041, 0xDC00];

        let v: Vec<_> = DecodeUtf16Iter::new(units.into_iter()).collect();

        assert_eq!(
            v,
            vec![
                CodePoint::Char('😀'),
                CodePoint::Surrogate(0xD800),
                CodePoint::Char('A'),
                CodePoint::Surrogate(0xDC00),
            ]
        );
    }
}
