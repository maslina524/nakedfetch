use core::{
    alloc::Layout,
    ptr::{self, NonNull},
};

#[must_use]
pub fn calloc(layout: Layout) -> NonNull<u8> {
    let ptr = super::malloc(layout);
    // SAFETY: Zero only the allocated memory
    unsafe { ptr::write_bytes(ptr.as_ptr(), 0u8, layout.size()) };
    ptr
}
