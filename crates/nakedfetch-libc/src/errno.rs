use core::{error::Error, fmt, num::NonZeroU16};

#[derive(Debug)]
pub struct IsNotErrno;

impl fmt::Display for IsNotErrno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for IsNotErrno {}

/// Stores `-errno` `(-4096..0)`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Errno(NonZeroU16);

impl Errno {
    /// # Errors
    ///
    /// Accepts a "raw" value from syscall, if the value is `-4096..0`, `Errno`
    /// is returned, otherwise `IsNotErrno`.
    pub const fn from_syscall(ret: isize) -> Result<Self, IsNotErrno> {
        if let Some(nonzero) = NonZeroU16::new(-ret as u16)
            && nonzero.get() <= 4096
        {
            Ok(Self(nonzero))
        } else {
            Err(IsNotErrno)
        }
    }

    #[must_use]
    pub const fn into_errno(self) -> isize {
        -(self.0.get() as isize)
    }
}

#[cfg(test)]
mod tests {
    use crate::errno::Errno;

    #[test]
    fn errno_from_syscall() {
        let syscall_ret = -22;
        let errno = Errno::from_syscall(syscall_ret).unwrap();
        assert_eq!(syscall_ret, errno.into_errno());

        let invalid_errno = Errno::from_syscall(1);
        assert!(invalid_errno.is_err());
    }
}
