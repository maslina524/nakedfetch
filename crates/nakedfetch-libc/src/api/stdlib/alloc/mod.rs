mod page;
mod sync;

mod malloc;
pub use malloc::malloc;

mod free;
pub use free::free;

#[cfg(test)]
mod tests {
    use core::{alloc::GlobalAlloc, ptr::NonNull};

    #[global_allocator]
    static ALLOCATOR: TestAlloc = TestAlloc;

    struct TestAlloc;

    // SAFETY: Test malloc via GlobalAlloc
    unsafe impl GlobalAlloc for TestAlloc {
        unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
            super::malloc(layout).as_ptr()
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
            let nonnull = NonNull::new(ptr).expect("Null ptr in dealloc");
            super::free(nonnull.cast(), layout);
        }
    }

    #[test]
    fn just_alloc() {
        let n = 42u64;
        let boxed = Box::new(n);

        assert_eq!(n, *boxed.as_ref());
    }

    #[test]
    fn sum_via_vec() {
        const N: usize = 1024;

        let sum: usize = {
            let mut buf = Vec::new();
            for i in 0..=N {
                assert!(i <= N);
                buf.push(i);
            }

            buf.iter().sum()
        }; // Drop

        assert_eq!(sum, N * (N + 1) / 2);
    }
}
