use core::{alloc::Layout, ptr::NonNull};

use libc::stdlib;

pub fn alloc(layout: Layout) -> *mut u8 {
    stdlib::malloc(layout).as_ptr()
}

pub fn dealloc(ptr: *mut u8, layout: Layout) {
    if let Some(nonnull) = NonNull::new(ptr) {
        stdlib::free(nonnull, layout);
    }
}

pub fn alloc_zeroed(layout: Layout) -> *mut u8 {
    stdlib::calloc(layout).as_ptr()
}

pub fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    // SAFETY: Create a `Layout` from the old `Layout`
    let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
    let nonnull =
        NonNull::new(ptr).unwrap_or_else(|| panic!("A null pointer was passed to realloc"));
    stdlib::realloc(nonnull, layout, new_layout).as_ptr()
}
