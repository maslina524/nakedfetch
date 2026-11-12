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
#[cfg(target_family = "unix")]
use core::ffi::CStr;
use core::{fmt, ptr};

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

    #[cfg(target_os = "windows")]
    #[must_use]
    pub const fn from_os_str(s: &OsStr) -> &Self {
        Self::from_bytes(s.as_bytes())
    }

    #[cfg(target_family = "unix")]
    #[must_use]
    pub const fn from_os_str(s: &CStr) -> &Self {
        Self::from_bytes(s.to_bytes())
    }

    #[must_use]
    pub const fn as_os_str(&self) -> &OsStr {
        &self.0
    }

    #[must_use]
    pub fn as_pathbuf(&self) -> PathBuf {
        PathBuf::from(&self.0)
    }

    #[must_use]
    pub fn as_smol_path(&self) -> SmolPath {
        SmolPath::from(self)
    }

    #[cfg(target_os = "windows")]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[cfg(target_family = "unix")]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
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

    #[cfg(target_family = "unix")]
    pub fn with_c_str<F>(&self, f: impl FnOnce(&CStr) -> F) -> F {
        match self.len() {
            0..16 => self.with_buf_unchecked::<16, _>(f),
            16..32 => self.with_buf_unchecked::<32, _>(f),
            32..64 => self.with_buf_unchecked::<64, _>(f),
            64..128 => self.with_buf_unchecked::<128, _>(f),
            128..256 => self.with_buf_unchecked::<256, _>(f),
            256..512 => self.with_buf_unchecked::<512, _>(f),
            512..1024 => self.with_buf_unchecked::<1024, _>(f),
            1024..2048 => self.with_buf_unchecked::<2048, _>(f),
            2048..4096 => self.with_buf_unchecked::<4096, _>(f),
            _ => unreachable!(),
        }
    }

    /// # Panics
    ///
    /// If buf is empty
    #[must_use]
    #[cfg(target_family = "unix")]
    pub fn as_c_str<'c>(&self, buf: &'c mut [u8]) -> &'c CStr {
        use core::ffi::CStr;

        let len = self.0.len();
        assert!(
            buf.len() > len,
            "buffer too small: need {} bytes, got {}",
            len + 1,
            buf.len()
        );
        assert!(!self.0.contains(&0), "self.0 must not contain interior NUL");

        buf[..len].copy_from_slice(&self.0);
        buf[len] = 0;

        // SAFETY: The last byte is always NUL
        unsafe { CStr::from_bytes_with_nul_unchecked(&buf[..=len]) }
    }

    #[inline(never)]
    #[cfg(target_family = "unix")]
    fn with_buf_unchecked<const SIZE: usize, F>(&self, f: impl FnOnce(&CStr) -> F) -> F {
        assert!(self.len() < SIZE);
        let mut buf = [0u8; SIZE];
        let c_str = self.as_c_str(&mut buf);
        f(c_str)
    }

    #[must_use]
    pub fn to_str(&self) -> Option<&str> {
        str::from_utf8(self.as_bytes()).ok()
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.as_bytes()).into_owned()
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

        let bytes = &self.as_bytes()[..offset];

        #[cfg(target_os = "windows")]
        return Some(OsStr::from_bytes(bytes).as_ref());

        #[cfg(target_family = "unix")]
        Some(bytes.as_ref())
    }

    #[must_use]
    pub fn last(&self) -> Option<&Self> {
        self.components().next_back()
    }

    #[cfg(target_os = "windows")]
    #[must_use]
    pub fn extension(&self) -> Option<&OsStr> {
        use wtf8::CodePoint;

        let path_part = self.last()?;
        let name = path_part.as_os_str();
        let idx = name.rfind(CodePoint::Char('.'))?;
        Some(&name[idx + 1..])
    }

    #[cfg(target_family = "unix")]
    #[must_use]
    pub fn extension(&self) -> Option<&OsStr> {
        fn rfind(bytes: &[u8], cp: u8) -> Option<usize> {
            let mut result = None;
            for (i, icp) in bytes.iter().enumerate() {
                if *icp == cp {
                    result = Some(i);
                }
            }
            result
        }

        let path_part = self.last()?;
        let name = path_part.as_os_str();
        let idx = rfind(self.as_bytes(), b'.')?;
        Some(&name[idx + 1..])
    }

    #[must_use]
    pub fn trim_extension(&self) -> Option<&Self> {
        self.extension().map(|ext| {
            let bytes = self.as_bytes();
            let trim_len = ext.len() + 1;
            if bytes.len() >= trim_len {
                Self::from_bytes(&bytes[..bytes.len() - trim_len])
            } else {
                self
            }
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

#[cfg(target_os = "windows")]
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[cfg(target_os = "windows")]
impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.0)
    }
}

#[cfg(target_family = "unix")]
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write as _;

        let iter = self.0.utf8_chunks();
        for chunk in iter {
            f.write_str(chunk.valid())?;
            if !chunk.invalid().is_empty() {
                f.write_char(char::REPLACEMENT_CHARACTER)?;
            }
        }
        Ok(())
    }
}

#[cfg(target_family = "unix")]
impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write as _;
        f.write_char('"')?;

        let mut rest = &self.0;
        while !rest.is_empty() {
            match str::from_utf8(rest) {
                Ok(s) => {
                    for c in s.chars() {
                        for e in c.escape_debug() {
                            f.write_char(e)?;
                        }
                    }
                    break;
                },
                Err(e) => {
                    let valid = e.valid_up_to();
                    // SAFETY: `str::from_utf8` guarantees the validity of
                    // [..valid]
                    let valid_s = unsafe { str::from_utf8_unchecked(&rest[..valid]) };
                    for c in valid_s.chars() {
                        for e in c.escape_debug() {
                            f.write_char(e)?;
                        }
                    }
                    let err_len = e.error_len().unwrap_or(rest.len() - valid);
                    for &b in &rest[valid..valid + err_len] {
                        write!(f, "\\x{b:02X}")?;
                    }
                    rest = &rest[valid + err_len..];
                },
            }
        }

        f.write_char('"')
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

    #[cfg(target_os = "windows")]
    #[test]
    fn display_for_path_test() {
        let path = Path::from_bytes(b"C:/Hello/World\xED");
        let formatted = format!("{path}");
        assert_eq!(
            formatted,
            format!("C:/Hello/World{}", char::REPLACEMENT_CHARACTER)
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn debug_for_path_test() {
        let path = Path::from_bytes(b"C:/Hello/World\xED");
        let formatted = format!("{path:?}");
        assert_eq!(
            formatted,
            format!("\"C:/Hello/World{}\"", char::REPLACEMENT_CHARACTER)
        );
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn as_c_str() {
        let path = Path::from_str("/home/rustling/nakedfetch");
        path.with_c_str(|cstr| {
            assert_eq!(cstr, c"/home/rustling/nakedfetch");
        });
    }
}
