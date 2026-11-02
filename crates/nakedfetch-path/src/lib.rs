#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod buf;
pub use buf::PathBuf;

mod components;
use core::{fmt, ptr};

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
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr().cast()
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

    #[must_use]
    pub fn components(&'_ self) -> Components<'_> {
        Components::new(self)
    }

    #[must_use]
    pub fn parent(&self) -> Option<&Self> {
        let last = self.components().next_back()?;
        let offset = last.as_ptr() as usize - self.as_ptr() as usize;

        if offset == 0 {
            return None;
        }

        let bytes = &self.0.as_bytes()[..offset];
        Some(PathInner::from_bytes(bytes).as_ref())
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
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

impl AsRef<Path> for String {
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

    #[test]
    fn parent_test() {
        let path = Path::from_str("C:/Hello/World/Learn/Rust/");
        assert_eq!(path.parent(), Some("C:/Hello/World/Learn/".as_ref()));
    }
}
