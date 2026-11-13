use core::ptr::{self, NonNull};

use crate::{api::sys::types::c_off, errno::Errno, raw::Table, syscall};

pub const PROT_READ: i32 = 0x1;
pub const PROT_WRITE: i32 = 0x2;

pub const MAP_PRIVATE: i32 = 0x02;
pub const MAP_ANONYMOUS: i32 = 0x20;

/// # Errors
///
/// If failed to allocate mem
pub fn mmap(
    addr: Option<NonNull<()>>,
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: c_off,
) -> Result<NonNull<()>, Errno> {
    let addr_ptr = addr.map_or(ptr::null_mut(), NonNull::as_ptr);
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    unsafe {
        let size = syscall!(Table::Mmap, addr_ptr, length, prot, flags, fd, offset);
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

    use crate::api::sys::mman::{MAP_ANONYMOUS, MAP_PRIVATE, PROT_READ, PROT_WRITE};

    #[test]
    fn alloc_via_mmap() {
        let ptr = super::mmap(
            None,
            128,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
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
