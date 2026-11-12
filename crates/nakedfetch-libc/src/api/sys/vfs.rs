use core::{ffi::c_int, mem::MaybeUninit};

use crate::{errno::Errno, raw::Table, syscall};

pub type StatFsWord = u64;

#[cfg(not(any(target_arch = "mips", target_arch = "s390x")))]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KernelFsId {
    pub val: [c_int; 2],
}

/// Ref: <https://elixir.bootlin.com/linux/v6.17/source/include/uapi/asm-generic/statfs.h#L23>
#[cfg(not(any(target_arch = "mips", target_arch = "s390x")))]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StatFs {
    pub typ: StatFsWord,
    pub bsize: StatFsWord,
    pub blocks: StatFsWord,
    pub bfree: StatFsWord,
    pub bavail: StatFsWord,
    pub files: StatFsWord,
    pub ffree: StatFsWord,
    pub fsid: KernelFsId,
    pub namelen: StatFsWord,
    pub frsize: StatFsWord,
    pub flags: StatFsWord,
    pub spare: [StatFsWord; 4],
}

/// # Errors
///
/// If system failed to get statfs
pub fn fstatfs(fd: i32) -> Result<StatFs, Errno> {
    let mut buf = MaybeUninit::uninit();
    // SAFETY: The syscall complies with the documentation, errors are checked;
    // if there are none, the function has guaranteed to correctly write the
    // structure, which becomes initialized
    unsafe {
        syscall!(Table::FStatFs, fd, buf.as_mut_ptr())?;
        Ok(buf.assume_init())
    }
}

#[cfg(test)]
mod tests {
    use super::StatFs;

    #[test]
    fn fstatfs_comp_with_libc() {
        use core::ffi::c_char;

        unsafe extern "C" {
            unsafe fn open(path: *const c_char, flags: i32, ...) -> i32;
            unsafe fn close(fd: i32) -> i32;
            safe fn fstatfs(fd: i32, buf: *mut StatFs) -> i32;
        }

        // SAFETY: Open the file almost directly to get the fd
        let fd = unsafe { open(c"Cargo.toml".as_ptr(), 0o200_0000, 0) };
        assert!(fd >= 0, "Libc fopen failed");

        let my = super::fstatfs(fd).expect("Failed to get file stats");
        let mut libc = StatFs::default();
        assert!(fstatfs(fd, &raw mut libc) == 0, "Libc fstatfs failed");

        // SAFETY: Close valid fd
        unsafe { close(fd) };

        println!("{my:?}");
        assert_eq!(my, libc);
    }
}
