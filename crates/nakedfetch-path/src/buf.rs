use core::{ops::Deref, ptr};

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
