use alloc::sync::Arc;
use core::{error::Error, fmt, mem};

use crate::Wtf8;

const INLINE_CAP: usize = 23;

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

impl TryFrom<u8> for InlineLen {
    type Error = InlineLenOutOfBounds;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (1u8..=INLINE_CAP as u8).contains(&value) {
            // SAFETY: Enum bounds are checked, it has the same layout as u8,
            // safe
            let len = unsafe { mem::transmute::<u8, Self>(value) };
            Ok(len)
        } else {
            Err(InlineLenOutOfBounds)
        }
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

impl From<&'static Wtf8> for SmolWtf8 {
    fn from(value: &'static Wtf8) -> Self {
        Self(Repr::Static(value))
    }
}
