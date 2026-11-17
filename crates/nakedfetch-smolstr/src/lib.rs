#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod repr;

use alloc::{borrow::Cow, string::String, sync::Arc};
use core::{ffi::CStr, fmt, ops::Deref};

use repr::{INLINE_CAP, Repr};

/// [`SmolStr`] only stores the string, does not modify it
#[derive(Default, Clone)]
pub struct SmolStr(Repr);

impl SmolStr {
    #[must_use]
    pub const fn empty() -> Self {
        Self(Repr::Empty)
    }

    #[must_use]
    pub const fn from_static(s: &'static str) -> Self {
        if s.is_empty() {
            Self::empty()
        } else {
            Self(Repr::Static(s))
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match &self.0 {
            Repr::Empty => "",
            Repr::Heap(a) => a,
            // SAFETY: `SmolStr` is created only from `str`, utf8 is always
            // valid
            Repr::Inline { len, data } => unsafe {
                str::from_utf8_unchecked(&data[..*len as usize])
            },
            Repr::Static(s) => s,
        }
    }
}

impl Deref for SmolStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl PartialEq for SmolStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&str> for SmolStr {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<SmolStr> for &str {
    fn eq(&self, other: &SmolStr) -> bool {
        *self == other.as_str()
    }
}

#[inline]
fn str_to_arr(s: &str) -> [u8; INLINE_CAP] {
    let mut out = [0u8; INLINE_CAP];
    let mut len = s.len().min(INLINE_CAP);

    while len > 0 && !s.is_char_boundary(len) {
        len -= 1;
    }

    out[..len].copy_from_slice(&s.as_bytes()[..len]);
    out
}

macro_rules! impl_from_string {
    ($($typ:ty),+ $(,)?) => {$(
        impl From<$typ> for SmolStr {
            fn from(value: $typ) -> Self {
                let len = value.len();
                let repr = if len == 0 {
                    Repr::Empty
                } else if let Ok(inline) = (len as u8).try_into() {
                    Repr::Inline {
                        len: inline,
                        data: str_to_arr(&value),
                    }
                } else {
                    Repr::Heap(Arc::from(value))
                };

                Self(repr)
            }
        }
    )+};
}

impl_from_string!(String, &str, Cow<'static, str>);

impl TryFrom<&CStr> for SmolStr {
    type Error = core::str::Utf8Error;

    fn try_from(value: &CStr) -> Result<Self, Self::Error> {
        let len = value.count_bytes();
        let repr = if len == 0 {
            Repr::Empty
        } else if let Ok(inline) = (len as u8).try_into() {
            let s = value.to_str()?;
            Repr::Inline {
                len: inline,
                data: str_to_arr(s),
            }
        } else {
            let s = value.to_str()?;
            Repr::Heap(Arc::from(s))
        };

        Ok(Self(repr))
    }
}

impl fmt::Display for SmolStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for SmolStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;

    use crate::{SmolStr, repr::Repr};

    const S23: &str = "01020304050607080910111";
    const S24: &str = "010203040506070809101112";

    #[test]
    fn empty_str() {
        let smolstr = SmolStr::empty();
        assert!(matches!(smolstr.0, Repr::Empty));
    }

    #[test]
    fn static_str() {
        let smolstr = SmolStr::from_static(S23);
        assert!(matches!(smolstr.0, Repr::Static(_)));

        let smolstr = SmolStr::from_static(S24);
        assert!(matches!(smolstr.0, Repr::Static(_)));
    }

    #[test]
    fn inline_or_heap() {
        let smolstr = SmolStr::from(S23);
        assert!(matches!(smolstr.0, Repr::Inline { .. }));

        let smolstr = SmolStr::from(S24);
        assert!(matches!(smolstr.0, Repr::Heap(_)));
    }

    #[test]
    fn display_and_debug() {
        let smol_str = SmolStr::from(S23);

        assert_eq!(smol_str, S23);
        assert_eq!(format!("{smol_str:?}"), format!("{S23:?}"));
    }

    #[test]
    fn from_x_for_smolstr() {
        let smol_str = SmolStr::try_from(c"&CStr").expect("From &CStr failed");
        assert_eq!(smol_str, "&CStr");

        let smol_str = SmolStr::from(Cow::Borrowed("Borrowed"));
        assert_eq!(smol_str, "Borrowed");
        let smol_str = SmolStr::from(Cow::Owned("Owned".to_string()));
        assert_eq!(smol_str, "Owned");

        let smol_str = SmolStr::from("String".to_string());
        assert_eq!(smol_str, "String");
    }

    #[test]
    fn non_ascii() {
        let smol_str = SmolStr::from("Привет Мир!");
        assert_eq!(smol_str, "Привет Мир!");

        let smol_str = SmolStr::from("Прывітанне Мір!");
        assert_eq!(smol_str, "Прывітанне Мір!");

        let smol_str = SmolStr::from("Γεια σου κόσμε!");
        assert_eq!(smol_str, "Γεια σου κόσμε!");

        let smol_str = SmolStr::from("你好世界！");
        assert_eq!(smol_str, "你好世界！");
    }
}
