use core::{error::Error, fmt, num::NonZeroU16};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsNotErrno;

impl fmt::Display for IsNotErrno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for IsNotErrno {}

macro_rules! errno_enum {
    ($($(#[$meta:meta])* $name:ident = $val:literal),* $(,)?) => {
        $(const _: () = assert!($val != 0, "errno value must be non-zero");)*

        #[allow(clippy::upper_case_acronyms, reason = "C style")]
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum Errno {
            $($(#[$meta])* $name,)*
            Unknown(NonZeroU16),
        }

        impl Errno {
            pub const fn from_nonzero(n: NonZeroU16) -> Self {
                match n.get() {
                    $($val => Self::$name,)*
                    _ => Self::Unknown(n),
                }
            }

            #[must_use]
            pub const fn get(self) -> NonZeroU16 {
                match self {
                    $(Self::$name => match NonZeroU16::new($val) {
                        Some(n) => n,
                        None => unreachable!(),
                    },)*
                    Self::Unknown(n) => n,
                }
            }

            #[must_use]
            pub const fn name(self) -> Option<&'static str> {
                match self {
                    $(Self::$name => Some(stringify!($name)),)*
                    Self::Unknown(_) => None,
                }
            }
        }

        impl fmt::Debug for Errno {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.name() {
                    Some(name) => f.write_str(name),
                    None => write!(f, "Unknown({})", self.get().get()),
                }
            }
        }

        impl fmt::Display for Errno {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.name() {
                    Some(name) => f.write_str(name),
                    None => write!(f, "errno {}", self.get().get()),
                }
            }
        }
    };
}

errno_enum! {
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENXIO = 6,
    E2BIG = 7,
    ENOEXEC = 8,
    EBADF = 9,
    ECHILD = 10,
    EAGAIN = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    EBUSY = 16,
    EEXIST = 17,
    EXDEV = 18,
    ENODEV = 19,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    ENFILE = 23,
    EMFILE = 24,
    ENOTTY = 25,
    ETXTBSY = 26,
    EFBIG = 27,
    ENOSPC = 28,
    ESPIPE = 29,
    EROFS = 30,
    EMLINK = 31,
    EPIPE = 32,
    EDOM = 33,
    ERANGE = 34,
    EDEADLK = 35,
    ENAMETOOLONG = 36,
    ENOLCK = 37,
    ENOSYS = 38,
    ENOTEMPTY = 39,
    ELOOP = 40,
    ENOMSG = 42,
    EIDRM = 43,
    ENODATA = 61,
    ENOSTR = 60,
    ETIME = 62,
    ENOSR = 63,
    ENONET = 64,
    EREMOTE = 66,
    ENOLINK = 67,
    EPROTO = 71,
    EMULTIHOP = 72,
    EBADMSG = 74,
    EOVERFLOW = 75,
    EILSEQ = 84,
    EUSERS = 87,
    ENOTSOCK = 88,
    EDESTADDRREQ = 89,
    EMSGSIZE = 90,
    EPROTOTYPE = 91,
    ENOPROTOOPT = 92,
    EPROTONOSUPPORT = 93,
    ESOCKTNOSUPPORT = 94,
    EOPNOTSUPP = 95,
    EPFNOSUPPORT = 96,
    EAFNOSUPPORT = 97,
    EADDRINUSE = 98,
    EADDRNOTAVAIL = 99,
    ENETDOWN = 100,
    ENETUNREACH = 101,
    ENETRESET = 102,
    ECONNABORTED = 103,
    ECONNRESET = 104,
    ENOBUFS = 105,
    EISCONN = 106,
    ENOTCONN = 107,
    ESHUTDOWN = 108,
    ETOOMANYREFS = 109,
    ETIMEDOUT = 110,
    ECONNREFUSED = 111,
    EHOSTDOWN = 112,
    EHOSTUNREACH = 113,
    EALREADY = 114,
    EINPROGRESS = 115,
    ESTALE = 116,
    EDQUOT = 122,
    ECANCELED = 125,
    EOWNERDEAD = 130,
    ENOTRECOVERABLE = 131,
    ERFKILL = 132,
    EHWPOISON = 133,
}

impl Errno {
    /// # Errors
    ///
    /// Accepts a "raw" value from syscall, if the value is `-4095..0`, `Errno`
    /// is returned, otherwise `IsNotErrno`
    pub const fn errno_from_syscall(ret: isize) -> Result<Self, IsNotErrno> {
        if ret < 0 && ret >= -4095 {
            match NonZeroU16::new((-ret) as u16) {
                Some(n) => Ok(Self::from_nonzero(n)),
                None => unreachable!(),
            }
        } else {
            Err(IsNotErrno)
        }
    }

    /// # Errors
    ///
    /// Accepts a "raw" value from syscall, if the value is `-4095..0`, `Errno`
    /// is returned, otherwise `IsNotErrno`
    pub const fn result_from_syscall(ret: isize) -> Result<isize, Self> {
        match Self::errno_from_syscall(ret) {
            Ok(e) => Err(e),
            Err(_) => Ok(ret),
        }
    }

    #[must_use]
    pub const fn into_errno(self) -> isize {
        -(self.get().get() as isize)
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

    #[test]
    fn errno_as_enum() {
        let syscall_ret = -22;
        let errno = Errno::errno_from_syscall(syscall_ret).unwrap();
        assert_eq!(errno, Errno::EINVAL);
    }
}
