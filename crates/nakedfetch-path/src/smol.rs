use alloc::{string::String, sync::Arc};
use core::{fmt, ops::Deref};

use crate::{OsStr, Path, PathBuf, SmolOsInner};

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct SmolPath(SmolOsInner);

impl SmolPath {
    #[must_use]
    pub const fn from_static(s: &'static OsStr) -> Self {
        Self(SmolOsInner::from_static(s))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[cfg(target_os = "windows")]
    #[must_use]
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_wtf8()
    }

    #[cfg(target_family = "unix")]
    #[must_use]
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_slice()
    }

    #[must_use]
    pub fn as_path(&self) -> &Path {
        Path::from_os_str(self.as_os_str())
    }

    #[must_use]
    pub fn as_pathbuf(&self) -> PathBuf {
        PathBuf::from(self.as_path())
    }

    #[cfg(target_os = "windows")]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }

    #[cfg(target_family = "unix")]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_os_str()
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
        Self(SmolOsInner::from(value.as_os_str()))
    }
}

impl From<PathBuf> for SmolPath {
    fn from(value: PathBuf) -> Self {
        Self(SmolOsInner::from(value.as_os_str()))
    }
}

#[cfg(target_os = "windows")]
impl From<&str> for SmolPath {
    fn from(value: &str) -> Self {
        Self(SmolOsInner::from(value))
    }
}

#[cfg(target_family = "unix")]
impl From<&str> for SmolPath {
    fn from(value: &str) -> Self {
        Self(SmolOsInner::from(value.as_bytes()))
    }
}

#[cfg(target_os = "windows")]
impl From<String> for SmolPath {
    fn from(value: String) -> Self {
        Self(SmolOsInner::from(value))
    }
}

#[cfg(target_family = "unix")]
impl From<String> for SmolPath {
    fn from(value: String) -> Self {
        Self(SmolOsInner::from(value.as_bytes()))
    }
}

impl From<Arc<Path>> for SmolPath {
    fn from(value: Arc<Path>) -> Self {
        Self(SmolOsInner::from(value.as_os_str()))
    }
}

impl fmt::Display for SmolPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_path())
    }
}

impl fmt::Debug for SmolPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_path())
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
