use core::{ops::Deref, ptr, slice};

use crate::{Path, PathBufInner};

#[repr(transparent)]
#[derive(Debug, Default, PartialEq, Eq)]
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

        let s = unsafe {
            let ptr = self.as_ptr().add(path_off);
            let bytes = slice::from_raw_parts(ptr, path_len);
            str::from_utf8_unchecked(bytes)
        };

        Some(Path::from_str(s))
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

impl From<&str> for PathBuf {
    fn from(s: &str) -> Self {
        Self(PathBufInner::from(s))
    }
}

#[cfg(test)]
mod tests {
    use crate::PathBuf;

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
}
