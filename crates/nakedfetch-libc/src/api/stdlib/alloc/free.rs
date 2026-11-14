use core::{alloc::Layout, ptr::NonNull};

use super::page::{CHUNK_SIZE, DATA_SIZE, PageLargeAlloc, SMALL_MAX, king_page};

#[allow(clippy::missing_panics_doc, reason = "Panics only in debug for test")]
pub fn free(ptr: NonNull<()>, layout: Layout) {
    let size = layout.size();
    let align = layout.align();

    if size <= SMALL_MAX && align <= CHUNK_SIZE {
        assert!(free_small(ptr, size), "Pointer not found in Pages");
    } else {
        free_large(ptr);
    }
}

fn free_small(ptr: NonNull<()>, size: usize) -> bool {
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

fn free_large(ptr: NonNull<()>) {
    let large = PageLargeAlloc::restore_from_data(ptr);
    large.free();
}
