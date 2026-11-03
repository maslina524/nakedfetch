use alloc::string::String;
use core::{fmt, ops::Deref, ptr, slice};

use wtf8::Wtf8;

use crate::{Path, PathBufInner};

#[repr(transparent)]
#[derive(Default, PartialEq, Eq)]
pub struct PathBuf(PathBufInner);

impl PathBuf {
    #[must_use]
    pub const fn new() -> Self {
        Self(PathBufInner::new())
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self(PathBufInner::with_capacity(cap))
    }

    #[must_use]
    pub const fn as_path(&self) -> &Path {
        // SAFETY: Layout of Wtf8 == Path
        unsafe { &*(ptr::from_ref(self.0.as_wtf8()) as *const Path) }
    }

    #[must_use]
    pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr().cast()
    }

    #[must_use]
    pub fn into_os_string(self) -> PathBufInner {
        self.0
    }

    #[must_use]
    pub fn has_trailing_sep(&self) -> bool {
        self.0.ends_with_char('\\') || self.0.ends_with_char('/')
    }

    pub fn pop_trailing_sep(&mut self) -> bool {
        if !self.has_trailing_sep() {
            return false;
        }
        self.0.pop().is_some()
    }

    pub fn pop(&mut self) -> Option<&Path> {
        let (path_len, path_off) = {
            let path = self.components().next_back()?;
            let path_len = path.len();
            let path_off = path.as_ptr().addr() - self.0.as_ptr().addr();
            (path_len, path_off)
        };

        self.0.truncate(path_off);

        // SAFETY: Just rebuilding the reference, the lifetime is the same
        let s = unsafe {
            let ptr = self.as_ptr().add(path_off);
            let bytes = slice::from_raw_parts(ptr, path_len);
            str::from_utf8_unchecked(bytes)
        };

        Some(Path::from_str(s))
    }

    #[must_use]
    pub fn join<T: AsRef<Path>>(self, path: T) -> Self {
        self.join_inner(path.as_ref())
    }

    fn join_inner(mut self, path: &Path) -> Self {
        if !self.has_trailing_sep() {
            self.0.push_char('/');
        }

        self.0.push_wtf8(&path.as_wtf8());
        self
    }
}

impl Deref for PathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.as_path()
    }
}

impl AsRef<Self> for PathBuf {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl<const N: usize> From<&[u8; N]> for PathBuf {
    fn from(s: &[u8; N]) -> Self {
        Self(PathBufInner::from(s))
    }
}

impl From<&[u8]> for PathBuf {
    fn from(s: &[u8]) -> Self {
        Self(PathBufInner::from(s))
    }
}

impl From<&str> for PathBuf {
    fn from(s: &str) -> Self {
        Self(PathBufInner::from(s))
    }
}

impl From<String> for PathBuf {
    fn from(s: String) -> Self {
        Self(PathBufInner::from(s))
    }
}

impl fmt::Display for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Path, PathBuf};

    #[test]
    fn pop_pathbuf_test() {
        let mut pathbuf = PathBuf::from("C:/Hello/World/");

        assert_eq!(pathbuf.pop(), Some("World".as_ref()));
        assert_eq!(pathbuf.to_str(), Some("C:/Hello/"));

        assert_eq!(pathbuf.pop(), Some("Hello".as_ref()));
        assert_eq!(pathbuf.to_str(), Some("C:/"));

        assert_eq!(pathbuf.pop(), Some("C:".as_ref()));
        assert_eq!(pathbuf.to_str(), Some(""));

        assert_eq!(pathbuf.pop(), None);
        assert_eq!(pathbuf.to_str(), Some(""));
    }

    #[test]
    fn join_pathbuf_test() {
        let mut pathbuf = PathBuf::from("C:/Hello/World");

        pathbuf = pathbuf.join("Learn");
        assert_eq!(pathbuf.as_wtf8(), "C:/Hello/World/Learn".as_ref());

        pathbuf.pop();
        assert_eq!(pathbuf.as_wtf8(), "C:/Hello/World/".as_ref());

        pathbuf.pop();
        assert_eq!(pathbuf.as_wtf8(), "C:/Hello/".as_ref());

        pathbuf = pathbuf.join(Path::from_str("Rust"));
        assert_eq!(pathbuf.as_wtf8(), "C:/Hello/Rust".as_ref());
    }

    #[test]
    fn display_for_pathbuf_test() {
        let path = PathBuf::from(b"C:/Hello/World\xED");
        let formatted = format!("{path}");
        assert_eq!(
            formatted,
            format!("C:/Hello/World{}", char::REPLACEMENT_CHARACTER)
        );
    }

    #[test]
    fn debug_for_pathbuf_test() {
        let path = PathBuf::from(b"C:/Hello/World\xED");
        let formatted = format!("{path:?}");
        assert_eq!(
            formatted,
            format!("\"C:/Hello/World{}\"", char::REPLACEMENT_CHARACTER)
        );
    }
}
