#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod components;
use core::ptr;

pub use components::Components;

mod error;
mod os;
use alloc::string::String;

pub use error::PathBuildError;
pub use os::*;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Path(PathInner);

impl Path {
    /// # Safety
    ///
    /// Сaller must guarantee that the path is valid for the OS.
    #[must_use]
    pub unsafe fn from_str_unchecked(s: &str) -> &Self {
        let inner: &PathInner = s.as_ref();
        // SAFETY: Layout of PathInner == Path
        unsafe { &*(ptr::from_ref(inner) as *const Self) }
    }

    /// # Errors
    ///
    /// Returns `Err` if the path is not valid for the OS.
    #[allow(clippy::should_implement_trait, reason = "Bad lifetime")]
    pub fn from_str(s: &str) -> Result<&Self, PathBuildError> {
        let comps = Components::new(s);
        error::check(comps)?;
        // SAFETY: Check the validity of the path.
        Ok(unsafe { Self::from_str_unchecked(s) })
    }

    #[must_use]
    pub const fn as_inner(&self) -> &PathInner {
        &self.0
    }

    #[must_use]
    #[cfg(target_os = "windows")]
    pub fn as_utf16(&'_ self) -> wtf8::Utf16Iter<'_> {
        self.0.utf16_with_zero()
    }

    #[must_use]
    pub fn to_str(&self) -> Option<&str> {
        str::from_utf8(self.0.as_bytes()).ok()
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.0.as_bytes()).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::Path;

    #[test]
    fn path_to_str_test() {
        let path = Path::from_str("C:/User/rustling").expect("Failed to check path");
        assert_eq!(path.to_str(), Some("C:/User/rustling"));
    }
}
