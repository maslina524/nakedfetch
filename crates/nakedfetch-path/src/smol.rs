use alloc::{string::String, sync::Arc};
use core::{fmt, ops::Deref};

use wtf8::{SmolWtf8, Wtf8};

use crate::{Path, PathBuf};

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct SmolPath(SmolWtf8);

impl SmolPath {
    #[must_use]
    pub const fn from_static(s: &'static Wtf8) -> Self {
        Self(SmolWtf8::from_static(s))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn as_wtf8(&self) -> &Wtf8 {
        self.0.as_wtf8()
    }

    #[must_use]
    pub fn as_path(&self) -> &Path {
        Path::from_os_str(self.0.as_wtf8())
    }

    #[must_use]
    pub fn as_pathbuf(&self) -> PathBuf {
        PathBuf::from(self.as_path())
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_wtf8().as_bytes()
    }
}

impl Deref for SmolPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.as_path()
    }
}

impl AsRef<Self> for SmolPath {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Path> for SmolPath {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl From<&Path> for SmolPath {
    fn from(value: &Path) -> Self {
        Self(SmolWtf8::from(value.as_os_str()))
    }
}

impl From<PathBuf> for SmolPath {
    fn from(value: PathBuf) -> Self {
        Self(SmolWtf8::from(value.as_os_str()))
    }
}

impl From<&str> for SmolPath {
    fn from(value: &str) -> Self {
        Self(SmolWtf8::from(value))
    }
}

impl From<String> for SmolPath {
    fn from(value: String) -> Self {
        Self(SmolWtf8::from(value))
    }
}

impl From<Arc<Path>> for SmolPath {
    fn from(value: Arc<Path>) -> Self {
        Self(SmolWtf8::from(value.as_os_str()))
    }
}

impl fmt::Display for SmolPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_wtf8())
    }
}

impl fmt::Debug for SmolPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_wtf8())
    }
}

impl PartialEq for SmolPath {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<Path> for SmolPath {
    fn eq(&self, other: &Path) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<PathBuf> for SmolPath {
    fn eq(&self, other: &PathBuf) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for SmolPath {}
