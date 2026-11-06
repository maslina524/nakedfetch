use alloc::vec::Vec;
use core::{error::Error, fmt, ptr, slice};

use path::Path;
use wtf8::Wtf8;

use crate::{ntdll, types::UNICODE_STRING};

#[derive(Debug, Clone, Copy)]
pub struct ConvertDosPathToNtPathError;

impl fmt::Display for ConvertDosPathToNtPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ConvertDosPathToNtPathError {}

pub struct UnicodeString {
    raw: UNICODE_STRING,
    owned: Option<Vec<u16>>,
}

impl UnicodeString {
    /// # Errors
    ///
    /// Returns `Err` when it fails to convert DOS path to NT path.
    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, ConvertDosPathToNtPathError> {
        Self::from_path_inner(path.as_ref())
    }

    fn from_path_inner(path: &Path) -> Result<Self, ConvertDosPathToNtPathError> {
        let buf: Vec<u16> = path.as_utf16().collect();

        let mut unicode_raw = UNICODE_STRING::default();
        let ret = Self::dos_path_to_nt_path(&buf, &mut unicode_raw);
        if ret {
            Ok(Self {
                raw: unicode_raw,
                owned: None,
            })
        } else {
            Err(ConvertDosPathToNtPathError)
        }
    }

    pub fn from_wtf8<T: AsRef<Wtf8>>(s: T) -> Self {
        Self::from_wtf8_inner(s.as_ref())
    }

    fn from_wtf8_inner(s: &Wtf8) -> Self {
        let buf: Vec<u16> = s.utf16_with_zero().collect();

        let mut unicode_raw = UNICODE_STRING::default();
        Self::init(&buf, &mut unicode_raw);
        Self {
            raw: unicode_raw,
            owned: Some(buf),
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.raw.Length as usize / size_of::<u16>()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[u16] {
        let inner = &self.raw;
        // SAFETY: Method `len` returns the number of characters, not
        // bytes, safe
        unsafe { slice::from_raw_parts(inner.Buffer, self.len()) }
    }

    #[must_use]
    pub const fn as_inner_ptr(&self) -> *const UNICODE_STRING {
        &raw const self.raw
    }

    #[must_use]
    pub const fn as_inner_mut_ptr(&mut self) -> *mut UNICODE_STRING {
        &raw mut self.raw
    }

    fn dos_path_to_nt_path(s: &[u16], unicode_raw: &mut UNICODE_STRING) -> bool {
        // SAFETY: `&[u16]` slice may be incorrect, mostly this shouldn't happen
        // since in common we convert valid Paths to `&[u16]`
        let ret = unsafe {
            ntdll::RtlDosPathNameToNtPathName_U(
                s.as_ptr(),
                unicode_raw,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        ret.into()
    }

    fn init(s: &[u16], unicode_raw: &mut UNICODE_STRING) {
        // SAFETY: `&[u16]` slice may be incorrect, mostly this shouldn't happen
        // since in common we convert valid Paths to `&[u16]`
        unsafe {
            ntdll::RtlInitUnicodeString(unicode_raw, s.as_ptr());
        }
    }
}

impl Drop for UnicodeString {
    fn drop(&mut self) {
        if self.owned.is_none() {
            // SAFETY: `UNICODE_STRING` is valid, drop is called only once, the
            // function returns `()`
            unsafe { ntdll::RtlFreeUnicodeString(&raw mut self.raw) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UnicodeString;

    #[test]
    fn unicode_new_path_test() {
        let unicode =
            UnicodeString::from_path("C:/Users/").expect("Failet to alloc unicode string");
        let slice = unicode.as_slice();

        let string = String::from_utf16_lossy(slice);
        assert_eq!(string, r"\??\C:\Users\");
    }
}
