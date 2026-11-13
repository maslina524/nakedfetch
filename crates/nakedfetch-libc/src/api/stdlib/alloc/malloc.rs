use core::{alloc::Layout, ptr::NonNull};

use super::page::{CHUNK_COUNT, Page, king_page};

#[must_use]
pub fn malloc(layout: Layout) -> NonNull<u8> {
    let size = layout.size();
    match size {
        0..=1024 => malloc_small(size),
        _ => todo!(),
    }
}

fn malloc_small(size: usize) -> NonNull<u8> {
    let chunks = size.div_ceil(size);
    let mut page = king_page();

    loop {
        for start in (0..CHUNK_COUNT).step_by(chunks) {
            if page.meta().try_acquire_chunks(start..chunks) {
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
