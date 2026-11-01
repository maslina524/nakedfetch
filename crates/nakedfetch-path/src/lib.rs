#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod buf;
pub use buf::PathBuf;

mod components;
use core::ptr;

pub use components::Components;

mod os;
use alloc::string::String;

// pub use error::PathBuildError;
pub use os::*;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Path(PathInner);

impl Path {
    #[must_use]
    pub const fn from_str(s: &str) -> &Self {
        // SAFETY: Layout of str == [u8] == Wtf8 == Path
        unsafe { &*(ptr::from_ref(s) as *const Self) }
    }

    #[must_use]
    pub const fn as_os_str(&self) -> &PathInner {
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

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<Self> for Path {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Path> for str {
    fn as_ref(&self) -> &Path {
        Path::from_str(self)
    }
}

impl AsRef<Path> for PathInner {
    fn as_ref(&self) -> &Path {
        // SAFETY: Layout of Wtf8 == Path
        unsafe { &*(ptr::from_ref(self) as *const Path) }
    }
}

impl AsRef<PathInner> for Path {
    fn as_ref(&self) -> &PathInner {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::Path;

    #[test]
    fn path_to_str_test() {
        let path = Path::from_str("C:/User/rustling");
        assert_eq!(path.to_str(), Some("C:/User/rustling"));
    }
}
