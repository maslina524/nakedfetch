use core::{error::Error, fmt, num::NonZeroU16};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsNotErrno;

impl fmt::Display for IsNotErrno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for IsNotErrno {}

/// Stores `-errno` `(-4095..0)`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Errno(NonZeroU16);

impl Errno {
    /// # Errors
    ///
    /// Accepts a "raw" value from syscall, if the value is `-4095..0`, `Errno`
    /// is returned, otherwise `IsNotErrno`.
    pub const fn errno_from_syscall(ret: isize) -> Result<Self, IsNotErrno> {
        if ret < 0 && ret >= -4095 {
            Ok(Self(match NonZeroU16::new(-ret as u16) {
                Some(n) => n,
                None => unreachable!(),
            }))
        } else {
            Err(IsNotErrno)
        }
    }

    /// # Errors
    ///
    /// Accepts a "raw" value from syscall, if the value is `-4095..0`, `Errno`
    /// is returned, otherwise `IsNotErrno`.
    pub const fn result_from_syscall(ret: isize) -> Result<isize, Self> {
        match Self::errno_from_syscall(ret) {
            Ok(t) => Err(t),
            Err(_) => Ok(ret),
        }
    }

    #[must_use]
    pub const fn into_errno(self) -> isize {
        -(self.0.get() as isize)
    }
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for Errno {}

#[cfg(test)]
mod tests {
    use crate::errno::Errno;

    #[test]
    fn errno_from_syscall() {
        let syscall_ret = -22;
        let errno = Errno::errno_from_syscall(syscall_ret).unwrap();
        assert_eq!(syscall_ret, errno.into_errno());

        let invalid_errno = Errno::errno_from_syscall(1);
        assert!(invalid_errno.is_err());
    }
}
