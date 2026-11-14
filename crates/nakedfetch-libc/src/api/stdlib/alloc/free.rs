use core::{alloc::Layout, ptr::NonNull};

use super::page::{CHUNK_SIZE, DATA_SIZE, PAGE_SIZE, SMALL_MAX, king_page};
use crate::sys::mman;

#[allow(clippy::missing_panics_doc, reason = "Panics only in debug for test")]
pub fn free(ptr: NonNull<u8>, layout: Layout) {
    let size = layout.size();
    let align = layout.align();
    let ret = if size <= SMALL_MAX && align <= CHUNK_SIZE {
        free_small(ptr, size)
    } else {
        free_large(ptr)
    };

    assert!(ret, "Pointer not found in Pages");
}

fn free_small(ptr: NonNull<u8>, size: usize) -> bool {
    let addr = ptr.addr().get();
    let mut page = king_page();
    loop {
        let data = page.data_ptr().addr();
        if (data..data + DATA_SIZE).contains(&addr) {
            break;
        }
        if let Some(p) = page.meta().next() {
            page = p;
        } else {
            return false;
        }
    }

    let offset = addr - page.data_ptr().addr();
    debug_assert!(offset.is_multiple_of(CHUNK_SIZE));
    let start = offset / CHUNK_SIZE;
    let count = size.div_ceil(CHUNK_SIZE);

    debug_assert!(
        start + count <= super::page::CHUNK_COUNT,
        "free: start={start} count={count} > CHUNK_COUNT (size mismatch between alloc and free?)"
    );
    page.meta().release_chunks(start..start + count);

    true
}

fn free_large(ptr: NonNull<u8>) -> bool {
    // SAFETY: `malloc_large` returns `mmap_base + PAGE_SIZE`, where `mmap_base`
    // is page-aligned (mmap guarantees `PAGE_SIZE == 4096`), so
    // `align_of::<usize>() == 8` is satisfied by definition. The first 8 bytes
    // of `mmap_base` store the total mapping size written by `malloc_large`
    let (base, map_size) = unsafe {
        let base: NonNull<()> =
            NonNull::new_unchecked(ptr.cast::<u8>().as_ptr().sub(PAGE_SIZE).cast());
        let size = base.cast::<usize>().as_ptr().read();
        (base, size)
    };

    mman::munmap(base, map_size).is_ok()
}
