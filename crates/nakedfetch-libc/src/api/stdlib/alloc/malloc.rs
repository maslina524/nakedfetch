use core::{alloc::Layout, ptr::NonNull};

use super::page::{CHUNK_COUNT, CHUNK_SIZE, Page, PageLargeAlloc, SMALL_MAX, king_page};

#[must_use]
pub fn malloc(layout: Layout) -> NonNull<u8> {
    let size = layout.size();
    let align = layout.align();
    if size <= SMALL_MAX && align <= CHUNK_SIZE {
        malloc_small(size)
    } else {
        malloc_large(layout)
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

fn malloc_large(layout: Layout) -> NonNull<u8> {
    let large = PageLargeAlloc::allocate_new(layout);
    large.data_ptr()
}
