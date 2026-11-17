use alloc::sync::Arc;
use core::mem;

pub const INLINE_CAP: usize = 23;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum InlineLen {
    _V1 = 1,
    _V2 = 2,
    _V3 = 3,
    _V4 = 4,
    _V5 = 5,
    _V6 = 6,
    _V7 = 7,
    _V8 = 8,
    _V9 = 9,
    _V10 = 10,
    _V11 = 11,
    _V12 = 12,
    _V13 = 13,
    _V14 = 14,
    _V15 = 15,
    _V16 = 16,
    _V17 = 17,
    _V18 = 18,
    _V19 = 19,
    _V20 = 20,
    _V21 = 21,
    _V22 = 22,
    _V23 = 23,
}

impl TryFrom<u8> for InlineLen {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        if (1u8..=INLINE_CAP as u8).contains(&value) {
            // SAFETY: Enum bounds are checked, it has the same layout as u8,
            // safe
            let len = unsafe { mem::transmute::<u8, Self>(value) };
            Ok(len)
        } else {
            Err(())
        }
    }
}

#[derive(Default, Clone)]
pub enum Repr {
    #[default]
    Empty,
    Inline {
        len: InlineLen,
        data: [u8; INLINE_CAP],
    },
    Heap(Arc<str>),
    Static(&'static str),
}

const _: () = assert!(size_of::<Repr>() == 24);
