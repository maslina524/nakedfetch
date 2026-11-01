use core::alloc::{GlobalAlloc, Layout};

use crate::imp::alloc;

pub struct Allocator;

// SAFETY: VERY safe wrappers are used, but it is still
// required that only pointers pointing to the initialized
// part of the heap are passed
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match alloc::alloc(layout) {
            Ok(p) => p,
            Err(e) => panic!("Failed to alloc: {e}"),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let ret = alloc::dealloc(ptr, layout);
        assert!(ret.is_ok(), "Failed to free");
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        match alloc::alloc_zeroed(layout) {
            Ok(p) => p,
            Err(e) => panic!("Failed to alloc: {e}"),
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        alloc::realloc(ptr, layout, new_size)
            .unwrap_or_else(|_| panic!("Failed to realloc (returned old ptr)"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[global_allocator]
    static ALLOCATOR: Allocator = Allocator;

    const N: usize = 2048;

    #[test]
    fn allocator_test() {
        let mut buf = Vec::new();

        // Don't use `collect` to check reallocation
        for i in 0..=N {
            buf.push(i);
        }

        let sum = buf.into_iter().sum::<usize>();
        assert_eq!(sum, N * (N + 1) / 2);
    }
}
