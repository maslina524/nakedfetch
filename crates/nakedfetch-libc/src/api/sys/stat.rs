use core::{
    ffi::{self, CStr},
    mem::MaybeUninit,
};

use crate::{
    errno::Errno,
    raw::Table,
    stdio::{self, FileAccess, FileDispose},
    sys::types::c_mode,
    syscall,
};

const AT_FDCWD: i32 = -100;

// Ref: <https://elixir.bootlin.com/linux/v6.17/source/arch/arm/include/uapi/asm/stat.h#L21>
#[cfg(target_arch = "arm")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stat {
    #[cfg(target_endian = "big")]
    pub st_dev: ffi::c_ushort,
    #[cfg(target_endian = "big")]
    __pad1: ffi::c_ushort,
    #[cfg(target_endian = "little")]
    pub st_dev: ffi::c_ulong,
    pub st_ino: ffi::c_ulong,
    pub st_mode: ffi::c_ushort,
    pub st_nlink: ffi::c_ushort,
    pub st_uid: ffi::c_ushort,
    pub st_gid: ffi::c_ushort,
    #[cfg(target_endian = "big")]
    pub st_rdev: ffi::c_ushort,
    #[cfg(target_endian = "big")]
    __pad2: ffi::c_ushort,
    #[cfg(target_endian = "little")]
    pub st_rdev: ffi::c_ulong,
    pub st_size: ffi::c_ulong,
    pub st_blksize: ffi::c_ulong,
    pub st_blocks: ffi::c_ulong,
    pub st_atime: ffi::c_ulong,
    pub st_atime_nsec: ffi::c_ulong,
    pub st_mtime: ffi::c_ulong,
    pub st_mtime_nsec: ffi::c_ulong,
    pub st_ctime: ffi::c_ulong,
    pub st_ctime_nsec: ffi::c_ulong,
    __unused4: ffi::c_ulong,
    __unused5: ffi::c_ulong,
}

// Ref: <https://elixir.bootlin.com/linux/v6.17/source/arch/x86/include/uapi/asm/stat.h#L10>
#[cfg(target_arch = "x86")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stat {
    pub st_dev: ffi::c_ulong,
    pub st_ino: ffi::c_ulong,
    pub st_mode: ffi::c_ushort,
    pub st_nlink: ffi::c_ushort,
    pub st_uid: ffi::c_ushort,
    pub st_gid: ffi::c_ushort,
    pub st_rdev: ffi::c_ulong,
    pub st_size: ffi::c_ulong,
    pub st_blksize: ffi::c_ulong,
    pub st_blocks: ffi::c_ulong,
    pub st_atime: ffi::c_ulong,
    pub st_atime_nsec: ffi::c_ulong,
    pub st_mtime: ffi::c_ulong,
    pub st_mtime_nsec: ffi::c_ulong,
    pub st_ctime: ffi::c_ulong,
    pub st_ctime_nsec: ffi::c_ulong,
    __unused4: ffi::c_ulong,
    __unused5: ffi::c_ulong,
}

/// Ref: <https://elixir.bootlin.com/linux/v6.17/source/include/uapi/asm-generic/stat.h#L24>
#[cfg(target_arch = "x86_64")] // & others
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stat {
    pub st_dev: ffi::c_ulong,
    pub st_ino: ffi::c_ulong,
    pub st_mode: ffi::c_uint,
    pub st_nlink: ffi::c_uint,
    pub st_uid: ffi::c_uint,
    pub st_gid: ffi::c_uint,
    pub st_rdev: ffi::c_ulong,
    __pad1: ffi::c_ulong,
    pub st_size: ffi::c_long,
    pub st_blksize: ffi::c_int,
    __pad2: ffi::c_int,
    pub st_blocks: ffi::c_long,
    pub st_atime: ffi::c_long,
    pub st_atime_nsec: ffi::c_ulong,
    pub st_mtime: ffi::c_long,
    pub st_mtime_nsec: ffi::c_ulong,
    pub st_ctime: ffi::c_long,
    pub st_ctime_nsec: ffi::c_ulong,
    __unused4: ffi::c_uint,
    __unused5: ffi::c_uint,
}

/// # Errors
///
/// If system failed to get stat
pub fn fstat(fd: i32) -> Result<Stat, Errno> {
    let mut buf = MaybeUninit::uninit();
    // SAFETY: The syscall complies with the documentation, errors are checked;
    // if there are none, the function has guaranteed to correctly write the
    // structure, which becomes initialized
    unsafe {
        syscall!(Table::NewFStat, fd, buf.as_mut_ptr())?;
        Ok(buf.assume_init())
    }
}

/// # Errors
///
/// If system failed to get stat
pub fn stat(path: &CStr) -> Result<Stat, Errno> {
    let file = stdio::fopen(path, FileAccess::Read, FileDispose::OpenExisting)?;
    fstat(file.as_raw_fd())
}

/// # Errors
///
/// If system failed to make dir.
pub fn mkdirat(dirfd: i32, path: &CStr, mode: c_mode) -> Result<(), Errno> {
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let ret = unsafe { syscall!(Table::MkDirAt, dirfd, path.as_ptr(), mode) };
    ret.map(|_| ())
}

/// # Errors
///
/// If open(2) returned an error.
pub fn mkdir(path: &CStr, mode: c_mode) -> Result<(), Errno> {
    mkdirat(AT_FDCWD, path, mode)
}

#[cfg(test)]
mod tests {
    use super::Stat;

    #[test]
    fn fstat_comp_with_libc() {
        use core::ffi::c_char;

        unsafe extern "C" {
            unsafe fn open(path: *const c_char, flags: i32, ...) -> i32;
            unsafe fn close(fd: i32) -> i32;
            safe fn fstat(fd: i32, buf: *mut Stat) -> i32;
        }

        // SAFETY: Open the file almost directly to get the fd
        let fd = unsafe { open(c"Cargo.toml".as_ptr(), 0o200_0000, 0) };
        assert!(fd >= 0, "Libc fopen failed");

        let my = super::fstat(fd).expect("Failed to get file stats");
        let mut libc = Stat::default();
        assert!(fstat(fd, &raw mut libc) == 0, "Libc fstat failed");

        // SAFETY: Close valid fd
        unsafe { close(fd) };

        println!("{my:?}");
        assert_eq!(my, libc);
    }

    #[test]
    fn makedir() {
        let path = "Libc-Api-Sys-Stat-MkDir-1";
        let c_path = c"Libc-Api-Sys-Stat-MkDir-1";

        super::mkdir(c_path, 0o755).expect("Failed to make dir");
        assert!(std::fs::exists(path).expect("std::fs::exists failed"));

        std::fs::remove_dir(path).expect("std::fs::remove_dir failed");
    }
}
