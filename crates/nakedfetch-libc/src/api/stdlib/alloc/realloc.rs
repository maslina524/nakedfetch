use core::{
    alloc::Layout,
    ptr::{self, NonNull},
};

#[must_use]
pub fn realloc(ptr: NonNull<u8>, layout: Layout, new_layout: Layout) -> NonNull<u8> {
    let new_ptr = super::malloc(new_layout);
    // SAFETY: Copy the data from the old area to the new one, malloc guarantees
    // that the areas cannot overlap in any way
    unsafe {
        ptr::copy_nonoverlapping(
            ptr.as_ptr(),
            new_ptr.as_ptr(),
            layout.size().min(new_layout.size()),
        );
    }
    super::free(ptr, layout);
    new_ptr
}
