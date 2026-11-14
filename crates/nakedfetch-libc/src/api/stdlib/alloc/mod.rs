mod page;
mod sync;

mod malloc;
pub use malloc::malloc;

mod free;
pub use free::free;

mod calloc;
pub use calloc::calloc;

mod realloc;
pub use realloc::realloc;

#[cfg(test)]
mod tests {
    use core::{
        alloc::{GlobalAlloc, Layout},
        ptr::NonNull,
    };

    #[global_allocator]
    static ALLOCATOR: TestAlloc = TestAlloc;

    struct TestAlloc;

    // SAFETY: Test malloc via GlobalAlloc
    unsafe impl GlobalAlloc for TestAlloc {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            super::malloc(layout).as_ptr()
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            let nonnull = NonNull::new(ptr).expect("Null ptr in dealloc");
            super::free(nonnull.cast(), layout);
        }

        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            super::calloc(layout).as_ptr()
        }

        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            // SAFETY: Create a `Layout` from the old `Layout`
            let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
            let nonnull = NonNull::new(ptr).expect("Null ptr in dealloc");
            super::realloc(nonnull, layout, new_layout).as_ptr()
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

    #[test]
    fn alloc_zeroed() {
        // SAFETY: Is power of 2
        const SMALL: Layout = unsafe { Layout::from_size_align_unchecked(128, 1) };
        // SAFETY: Is power of 2
        const LARGE: Layout = unsafe { Layout::from_size_align_unchecked(8192, 1) };

        // Check for small
        let ptr = super::calloc(SMALL);
        for i in 0..SMALL.size() {
            // SAFETY: Check that all allocated bytes are zeros
            assert_eq!(unsafe { ptr.add(i).read() }, 0u8);
        }
        super::free(ptr, SMALL);

        // Check for large
        let ptr = super::calloc(LARGE);
        for i in 0..LARGE.size() {
            // SAFETY: Check that all allocated bytes are zeros
            assert_eq!(unsafe { ptr.add(i).read() }, 0u8);
        }
        super::free(ptr, LARGE);
    }
}
