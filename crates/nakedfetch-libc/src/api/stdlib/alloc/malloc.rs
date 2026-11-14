use core::{alloc::Layout, ptr::NonNull};

use super::page::{CHUNK_COUNT, CHUNK_SIZE, PAGE_SIZE, Page, SMALL_MAX, king_page};
use crate::sys::mman::{self, MmapFlags, MmapProt};

#[must_use]
pub fn malloc(layout: Layout) -> NonNull<u8> {
    let size = layout.size();
    let align = layout.align();
    if size <= SMALL_MAX && align <= CHUNK_SIZE {
        malloc_small(size)
    } else {
        malloc_large(size, align)
    }
}

fn malloc_small(size: usize) -> NonNull<u8> {
    let chunks = size.div_ceil(CHUNK_SIZE);
    let mut page = king_page();

    loop {
        for start in (0..=(CHUNK_COUNT - chunks)).step_by(chunks.max(1)) {
            if page.meta().try_acquire_chunks(start..start + chunks) {
                return page.chunk_ptr(start).cast();
            }
        }

        if let Some(next) = page.meta().next() {
            page = next;
        } else {
            let next = Page::allocate_new();
            let _ = page.meta().set_next(next);
            page = next;
        }
    }
}

fn malloc_large(size: usize, align: usize) -> NonNull<u8> {
    assert!(align <= PAGE_SIZE, "alignment {align} > PAGE_SIZE");

    let map_size = (size + PAGE_SIZE).next_multiple_of(PAGE_SIZE);

    let ptr = mman::mmap(
        None,
        map_size,
        MmapProt::new().read().write(),
        MmapFlags::new().private().anonymous(),
        -1,
        0,
    )
    .unwrap_or_else(|e| panic!("mmap({map_size}) failed: {e}"));

    // SAFETY: `mmap` returned at least map_size valid bytes, ptr is
    // page-aligned
    unsafe {
        ptr.cast::<usize>().as_ptr().write(map_size);
        let user = ptr.cast::<u8>().as_ptr().add(PAGE_SIZE);
        NonNull::new_unchecked(user)
    }
}
