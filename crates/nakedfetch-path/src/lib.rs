#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod buf;
mod components;
mod smol;

pub use buf::PathBuf;
pub use components::Components;
pub use os::*;
pub use smol::SmolPath;

mod os;
use alloc::string::String;
use core::{fmt, ptr};

use wtf8::CodePoint;

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct Path(OsStr);

impl Path {
    #[must_use]
    pub const fn from_bytes(bytes: &[u8]) -> &Self {
        // SAFETY: Layout of [u8] == OsStr == Path
        unsafe { &*(ptr::from_ref(bytes) as *const Self) }
    }

    #[must_use]
    pub const fn from_str(s: &str) -> &Self {
        Self::from_bytes(s.as_bytes())
    }

    #[must_use]
    pub const fn from_os_str(s: &OsStr) -> &Self {
        Self::from_bytes(s.as_bytes())
    }

    #[must_use]
    pub const fn as_os_str(&self) -> &OsStr {
        &self.0
    }

    #[must_use]
    pub fn as_smol_path(&self) -> SmolPath {
        SmolPath::from(self)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
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
        Some(OsStr::from_bytes(bytes).as_ref())
    }

    #[must_use]
    pub fn last(&self) -> Option<&Self> {
        self.components().next_back()
    }

    #[must_use]
    pub fn extension(&self) -> Option<&OsStr> {
        let path_part = self.last()?;
        let name = path_part.as_os_str();
        let idx = name.rfind(CodePoint::Char('.'))?;
        Some(&name[idx + 1..])
    }

    #[must_use]
    pub fn trim_extension(&self) -> Option<&Self> {
        self.extension().map(|ext| {
            let bytes = self.as_bytes();
            Self::from_bytes(&bytes[..bytes.len() - ext.len()])
        })
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

impl AsRef<Path> for OsStr {
    fn as_ref(&self) -> &Path {
        // SAFETY: Layout of Wtf8 == Path
        unsafe { &*(ptr::from_ref(self) as *const Path) }
    }
}

impl AsRef<OsStr> for Path {
    fn as_ref(&self) -> &OsStr {
        &self.0
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.0)
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

    #[test]
    fn display_for_path_test() {
        let path = Path::from_bytes(b"C:/Hello/World\xED");
        let formatted = format!("{path}");
        assert_eq!(
            formatted,
            format!("C:/Hello/World{}", char::REPLACEMENT_CHARACTER)
        );
    }

    #[test]
    fn debug_for_path_test() {
        let path = Path::from_bytes(b"C:/Hello/World\xED");
        let formatted = format!("{path:?}");
        assert_eq!(
            formatted,
            format!("\"C:/Hello/World{}\"", char::REPLACEMENT_CHARACTER)
        );
    }
}
