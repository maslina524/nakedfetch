use core::ptr::{self, NonNull};

use crate::{api::sys::types::c_off, errno::Errno, raw::Table, syscall};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmapProt(i32);

impl MmapProt {
    pub const EXEC: i32 = 0x4;
    pub const GROWSDOWN: i32 = 0x0100_0000;
    pub const GROWSUP: i32 = 0x0200_0000;
    pub const READ: i32 = 0x1;
    pub const SEM: i32 = 0x8;
    pub const WRITE: i32 = 0x2;

    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn read(self) -> Self {
        Self(self.0 | Self::READ)
    }

    pub const fn write(self) -> Self {
        Self(self.0 | Self::WRITE)
    }

    pub const fn exec(self) -> Self {
        Self(self.0 | Self::EXEC)
    }

    pub const fn sem(self) -> Self {
        Self(self.0 | Self::SEM)
    }

    pub const fn growsdown(self) -> Self {
        Self(self.0 | Self::GROWSDOWN)
    }

    pub const fn growsup(self) -> Self {
        Self(self.0 | Self::GROWSUP)
    }

    pub const fn build(self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmapFlags(i32);

impl MmapFlags {
    pub const ANONYMOUS: i32 = 0x20;
    pub const DENYWRITE: i32 = 0x0800;
    pub const EXECUTABLE: i32 = 0x1000;
    pub const FIXED: i32 = 0x10;
    pub const FIXED_NOREPLACE: i32 = 0x100_000;
    pub const GROWSDOWN: i32 = 0x0100;
    pub const HUGETLB: i32 = 0x40000;
    pub const LOCKED: i32 = 0x2000;
    pub const NONBLOCK: i32 = 0x10000;
    pub const NORESERVE: i32 = 0x4000;
    pub const POPULATE: i32 = 0x8000;
    pub const PRIVATE: i32 = 0x02;
    pub const SHARED: i32 = 0x01;
    pub const SHARED_VALIDATE: i32 = 0x03;
    pub const STACK: i32 = 0x20000;
    pub const SYNC: i32 = 0x80000;
    pub const _32BIT: i32 = 0x40;

    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn shared(self) -> Self {
        Self((self.0 & !0x03) | Self::SHARED)
    }

    pub const fn private(self) -> Self {
        Self((self.0 & !0x03) | Self::PRIVATE)
    }

    pub const fn shared_validate(self) -> Self {
        Self((self.0 & !0x03) | Self::SHARED_VALIDATE)
    }

    pub const fn fixed(self) -> Self {
        Self(self.0 | Self::FIXED)
    }

    pub const fn anonymous(self) -> Self {
        Self(self.0 | Self::ANONYMOUS)
    }

    pub const fn _32bit(self) -> Self {
        Self(self.0 | Self::_32BIT)
    }

    pub const fn growsdown(self) -> Self {
        Self(self.0 | Self::GROWSDOWN)
    }

    pub const fn denywrite(self) -> Self {
        Self(self.0 | Self::DENYWRITE)
    }

    pub const fn executable(self) -> Self {
        Self(self.0 | Self::EXECUTABLE)
    }

    pub const fn locked(self) -> Self {
        Self(self.0 | Self::LOCKED)
    }

    pub const fn noreserve(self) -> Self {
        Self(self.0 | Self::NORESERVE)
    }

    pub const fn populate(self) -> Self {
        Self(self.0 | Self::POPULATE)
    }

    pub const fn nonblock(self) -> Self {
        Self(self.0 | Self::NONBLOCK)
    }

    pub const fn stack(self) -> Self {
        Self(self.0 | Self::STACK)
    }

    pub const fn hugetlb(self) -> Self {
        Self(self.0 | Self::HUGETLB)
    }

    pub const fn sync(self) -> Self {
        Self(self.0 | Self::SYNC)
    }

    pub const fn fixed_noreplace(self) -> Self {
        Self(self.0 | Self::FIXED_NOREPLACE)
    }

    pub const fn build(self) -> i32 {
        self.0
    }
}

pub fn mmap(
    addr: Option<NonNull<()>>,
    length: usize,
    prot: MmapProt,
    flags: MmapFlags,
    fd: i32,
    offset: c_off,
) -> Result<NonNull<()>, Errno> {
    let addr_ptr = addr.map_or(ptr::null_mut(), NonNull::as_ptr);
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    unsafe {
        let size = syscall!(
            Table::Mmap,
            addr_ptr,
            length,
            prot.build(),
            flags.build(),
            fd,
            offset
        );
        size.map(|v| NonNull::new_unchecked(v as *mut ()))
    }
}

/// # Errors
///
/// If failed to deallocate mem
pub fn munmap(addr: NonNull<()>, length: usize) -> Result<(), Errno> {
    // let addr_ptr = addr.as_ptr();
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let ret = unsafe { syscall!(Table::Munmap, addr.as_ptr(), length) };
    ret.map(|_| ())
}

#[cfg(test)]
mod tests {
    use core::ptr;

    use crate::sys::mman::{MmapFlags, MmapProt};

    #[test]
    fn alloc_via_mmap() {
        let ptr = super::mmap(
            None,
            128,
            MmapProt::new().read().write(),
            MmapFlags::new().private().anonymous(),
            -1,
            0,
        )
        .expect("Failed mmap");

        // SAFETY: We write bytes only to the allocated memory
        unsafe { ptr::write_bytes(ptr.cast::<u8>().as_ptr(), 42u8, 128) };
        for i in 0..128 {
            // SAFETY: All 128 bytes must be equal to 42
            let val = unsafe { ptr::read(ptr.cast::<u8>().add(i).as_ptr()) };
            assert_eq!(val, 42u8);
        }

        super::munmap(ptr, 128).expect("Failed munmap");
    }
}
