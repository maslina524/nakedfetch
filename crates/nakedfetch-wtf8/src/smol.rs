use alloc::{string::String, sync::Arc};
use core::{error::Error, fmt, mem, ops::Deref};

use crate::{Wtf8, Wtf8Buf};

const INLINE_CAP: usize = 23;
const EMPTY_WTF8: &Wtf8 = Wtf8::from_str("");

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum InlineLen {
    _V1 = 1,
    _V2,
    _V3,
    _V4,
    _V5,
    _V6,
    _V7,
    _V8,
    _V9,
    _V10,
    _V11,
    _V12,
    _V13,
    _V14,
    _V15,
    _V16,
    _V17,
    _V18,
    _V19,
    _V20,
    _V21,
    _V22,
    _V23,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InlineLenOutOfBounds;

impl fmt::Display for InlineLenOutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for InlineLenOutOfBounds {}

impl TryFrom<usize> for InlineLen {
    type Error = InlineLenOutOfBounds;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if (1usize..=INLINE_CAP).contains(&value) {
            // SAFETY: Enum bounds are checked, it has the same layout as u8,
            // safe
            let len = unsafe { mem::transmute::<u8, Self>(value as u8) };
            Ok(len)
        } else {
            Err(InlineLenOutOfBounds)
        }
    }
}

impl From<InlineLen> for usize {
    fn from(value: InlineLen) -> Self {
        value as Self
    }
}

#[derive(Default, Clone)]
enum Repr {
    #[default]
    Empty,
    Inline {
        len: InlineLen,
        data: [u8; INLINE_CAP],
    },
    Heap(Arc<Wtf8>),
    Static(&'static Wtf8),
}

#[derive(Default, Clone)]
pub struct SmolWtf8(Repr);

impl SmolWtf8 {
    #[must_use]
    pub const fn from_static(s: &'static Wtf8) -> Self {
        Self(Repr::Static(s))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        match &self.0 {
            Repr::Empty => 0,
            Repr::Inline { len, data: _ } => (*len).into(),
            Repr::Heap(a) => a.len(),
            Repr::Static(s) => s.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn as_wtf8(&self) -> &Wtf8 {
        match &self.0 {
            Repr::Empty => EMPTY_WTF8,
            Repr::Inline { len, data } => {
                let usize_len = (*len).into();
                Wtf8::from_bytes(&data[..usize_len])
            },
            Repr::Heap(a) => a.as_ref(),
            Repr::Static(s) => s,
        }
    }
}

impl Deref for SmolWtf8 {
    type Target = Wtf8;

    fn deref(&self) -> &Self::Target {
        self.as_wtf8()
    }
}

impl From<Wtf8Buf> for SmolWtf8 {
    fn from(value: Wtf8Buf) -> Self {
        value.as_wtf8().into()
    }
}

impl From<&Wtf8> for SmolWtf8 {
    fn from(value: &Wtf8) -> Self {
        let bytes = value.as_bytes();
        let len = bytes.len();

        len.try_into().map_or_else(
            |_| Self(Repr::Heap(value.into())),
            |inline| {
                let mut buf = [0u8; INLINE_CAP];
                buf[..len].copy_from_slice(&bytes[..len]);
                Self(Repr::Inline {
                    len: inline,
                    data: buf,
                })
            },
        )
    }
}

impl From<&str> for SmolWtf8 {
    fn from(value: &str) -> Self {
        let bytes = value.as_bytes();
        let len = bytes.len();

        len.try_into().map_or_else(
            |_| {
                let wtf8 = Wtf8::from_str(value);
                Self(Repr::Heap(wtf8.into()))
            },
            |inline| {
                let mut buf = [0u8; INLINE_CAP];
                buf[..len].copy_from_slice(&bytes[..len]);
                Self(Repr::Inline {
                    len: inline,
                    data: buf,
                })
            },
        )
    }
}

impl From<String> for SmolWtf8 {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<Arc<Wtf8>> for SmolWtf8 {
    fn from(value: Arc<Wtf8>) -> Self {
        Self(Repr::Heap(value))
    }
}

impl fmt::Display for SmolWtf8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_wtf8())
    }
}

impl fmt::Debug for SmolWtf8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_wtf8())
    }
}

impl PartialEq for SmolWtf8 {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<Wtf8> for SmolWtf8 {
    fn eq(&self, other: &Wtf8) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<Wtf8Buf> for SmolWtf8 {
    fn eq(&self, other: &Wtf8Buf) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for SmolWtf8 {}

#[cfg(test)]
mod tests {
    use crate::{SmolWtf8, Wtf8, Wtf8Buf};

    #[test]
    fn smol_wtf8_len() {
        let wtf8 = Wtf8::from_str("Hello World!");
        let smol: SmolWtf8 = wtf8.into();

        assert_eq!(wtf8.len(), smol.len());
    }

    #[test]
    fn as_wtf8_len() {
        let wtf8 = Wtf8::from_str("Hello World!");
        let smol: SmolWtf8 = wtf8.into();

        assert_eq!(wtf8, smol.as_wtf8());
    }

    #[test]
    fn smol_from_wtf8buf_len() {
        let wtf8 = Wtf8Buf::from("Hello World!");
        let smol: SmolWtf8 = wtf8.as_wtf8().into();

        assert_eq!(wtf8.as_wtf8(), smol.as_wtf8());
    }

    #[test]
    fn smol_display_len() {
        let orig = "Hello World!";
        let smol = SmolWtf8::from(orig);

        assert_eq!(orig, format!("{smol}"));
    }
}
