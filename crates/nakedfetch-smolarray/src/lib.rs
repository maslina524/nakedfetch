#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

use alloc::{sync::Arc, vec::Vec};
use core::{
    fmt,
    mem::{self, MaybeUninit},
    num::NonZeroU8,
    ops::Deref,
    ptr, slice,
};

enum Repr<T: 'static, const N: usize> {
    Empty,
    Inline {
        len: NonZeroU8,
        data: [MaybeUninit<T>; N],
    },
    Heap(Arc<[T]>),
    Static(&'static [T]),
}

pub struct SmolArray<T: 'static, const N: usize>(Repr<T, N>);

const _: () = assert!(size_of::<SmolArray<u8, 22>>() == 24);

impl<T, const N: usize> SmolArray<T, N> {
    const ASSERT: () = {
        assert!(N >= 1, "SmolArray inline capacity N must be >= 1");
        assert!(
            N <= u8::MAX as usize,
            "SmolArray inline capacity N must be <= 255",
        );
    };

    #[must_use]
    pub const fn new() -> Self {
        let () = Self::ASSERT;
        Self(Repr::Empty)
    }

    #[must_use]
    pub const fn from_static(s: &'static [T]) -> Self {
        let () = Self::ASSERT;
        Self(Repr::Static(s))
    }

    #[must_use]
    pub fn from_slice(s: &[T]) -> Self
    where
        T: Clone,
    {
        struct Guard<'a, T, const N: usize> {
            data: &'a mut [MaybeUninit<T>; N],
            init: usize,
        }
        impl<T, const N: usize> Drop for Guard<'_, T, N> {
            fn drop(&mut self) {
                for slot in &mut self.data[..self.init] {
                    // SAFETY: The slice is created only from the already
                    // initialized part
                    unsafe { slot.assume_init_drop() };
                }
            }
        }

        let () = Self::ASSERT;
        let len = s.len();
        if len > N {
            return Self(Repr::Heap(s.into()));
        }
        let Some(len_nonzero) = NonZeroU8::new(len as u8) else {
            return Self(Repr::Empty);
        };

        let mut data: [MaybeUninit<T>; N] = core::array::from_fn(|_| MaybeUninit::uninit());

        let mut guard = Guard {
            data: &mut data,
            init: 0,
        };
        for (i, item) in s.iter().enumerate() {
            guard.data[i].write(item.clone());
            guard.init = i + 1;
        }
        mem::forget(guard);

        Self(Repr::Inline {
            len: len_nonzero,
            data,
        })
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        match &self.0 {
            Repr::Empty => &[],
            Repr::Inline { len, data } => {
                let len = len.get() as usize;
                // SAFETY: The slice is created only from the already
                // initialized part
                unsafe { slice::from_raw_parts(data.as_ptr().cast::<T>(), len) }
            },
            Repr::Heap(a) => a,
            Repr::Static(s) => s,
        }
    }
}

impl<T, const N: usize> Drop for SmolArray<T, N> {
    fn drop(&mut self) {
        if let Repr::Inline { len, data } = &mut self.0 {
            let len = len.get() as usize;
            // SAFETY: Only one drop
            unsafe {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    data.as_mut_ptr().cast::<T>(),
                    len,
                ));
            }
        }
    }
}

impl<T, const N: usize> Default for SmolArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, const N: usize> Clone for SmolArray<T, N> {
    fn clone(&self) -> Self {
        match &self.0 {
            Repr::Empty => Self(Repr::Empty),
            Repr::Static(s) => Self(Repr::Static(s)),
            Repr::Heap(a) => Self(Repr::Heap(a.clone())),
            Repr::Inline { .. } => Self::from_slice(self.as_slice()),
        }
    }
}

impl<T, const N: usize> Deref for SmolArray<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> From<Vec<T>> for SmolArray<T, N> {
    fn from(value: Vec<T>) -> Self {
        let () = Self::ASSERT;
        let len = value.len();
        if len > N {
            return Self(Repr::Heap(value.into()));
        }
        let Some(len_nonzero) = NonZeroU8::new(len as u8) else {
            return Self(Repr::Empty);
        };

        let mut data: [MaybeUninit<T>; N] = core::array::from_fn(|_| MaybeUninit::uninit());
        for (i, item) in value.into_iter().enumerate() {
            data[i].write(item);
        }
        Self(Repr::Inline {
            len: len_nonzero,
            data,
        })
    }
}

impl<T: Clone, const N: usize> From<&[T]> for SmolArray<T, N> {
    fn from(value: &[T]) -> Self {
        Self::from_slice(value)
    }
}

impl<T, const N: usize> From<Arc<[T]>> for SmolArray<T, N> {
    fn from(value: Arc<[T]>) -> Self {
        let () = Self::ASSERT;
        if value.is_empty() {
            Self(Repr::Empty)
        } else {
            Self(Repr::Heap(value))
        }
    }
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for SmolArray<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T: PartialEq, const N: usize> PartialEq for SmolArray<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq, const N: usize> Eq for SmolArray<T, N> {}

impl<T: PartialEq, const N: usize> PartialEq<[T]> for SmolArray<T, N> {
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T: PartialEq, const N: usize> PartialEq<Vec<T>> for SmolArray<T, N> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_and_len() {
        let a: SmolArray<u8, 22> = SmolArray::from_slice(b"Hello World!");
        assert_eq!(a.as_slice(), b"Hello World!");
        assert_eq!(a.len(), 12);
        assert!(!a.is_empty());
    }

    #[test]
    fn empty() {
        let a: SmolArray<u8, 22> = SmolArray::new();
        assert!(a.is_empty());
        assert_eq!(a.as_slice(), &[]);
    }

    #[test]
    fn heap_fallback() {
        let v: Vec<u32> = (0..100).collect();
        let a: SmolArray<u32, 24> = v.clone().into();
        assert_eq!(a.as_slice(), &v[..]);
    }

    #[test]
    fn statics() {
        static DATA: &[u8] = b"static data";
        let a: SmolArray<u8, 22> = SmolArray::from_static(DATA);
        assert_eq!(a.as_slice(), DATA);
    }

    #[test]
    fn clone_eq() {
        let a: SmolArray<String, 4> = SmolArray::from_slice(&["a".into(), "b".into(), "c".into()]);
        let b = a.clone();
        assert_eq!(a, b);
        assert_eq!(a, vec!["a".to_string(), "b".into(), "c".into()]);
    }

    #[test]
    fn drop_non_copy() {
        let a: SmolArray<String, 4> = SmolArray::from_slice(&["x".into(), "y".into(), "z".into()]);
        drop(a);
    }

    #[test]
    fn display_debug() {
        let a: SmolArray<u8, 22> = SmolArray::from_slice(&[1, 2, 3]);
        assert_eq!(format!("{a:?}"), "[1, 2, 3]");
    }
}
