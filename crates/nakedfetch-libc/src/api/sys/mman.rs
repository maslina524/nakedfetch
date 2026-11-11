use core::ptr::{self, NonNull};

use crate::{api::sys::types::c_off, raw::Table, syscall};

pub const PROT_READ: i32 = 0x1;
pub const PROT_WRITE: i32 = 0x2;

pub const MAP_PRIVATE: i32 = 0x02;
pub const MAP_ANONYMOUS: i32 = 0x20;

#[must_use]
pub fn mmap(
    addr: Option<NonNull<()>>,
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: c_off,
) -> *mut () {
    let addr_ptr = addr.map_or(ptr::null_mut(), NonNull::as_ptr);
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let size = unsafe { syscall!(Table::Mmap, addr_ptr, length, prot, flags, fd, offset) };
    size as *mut ()
}

#[must_use]
pub fn munmap(addr: *mut (), length: usize) -> i32 {
    // let addr_ptr = addr.as_ptr();
    // SAFETY: We pass a valid nr,
    // all values are synchronized with https://syscalls.mebeim.net
    let ret = unsafe { syscall!(Table::Munmap, addr, length) };
    ret as i32
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
        );

        // SAFETY: We write bytes only to the allocated memory
        unsafe { ptr::write_bytes(ptr.cast::<u8>(), 42u8, 128) };
        for i in 0..128 {
            // SAFETY: All 128 bytes must be equal to 42
            let val = unsafe { ptr::read(ptr.cast::<u8>().add(i)) };
            assert_eq!(val, 42u8);
        }

        assert_eq!(super::munmap(ptr, 128), 0);
    }
}
