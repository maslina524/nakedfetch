use core::{alloc::Layout, ptr::NonNull};

use super::page::king_page;
use crate::stdlib::alloc::page::{CHUNK_SIZE, DATA_SIZE};

#[allow(clippy::missing_panics_doc, reason = "Panics only in debug for test")]
pub fn free(ptr: NonNull<u8>, layout: Layout) {
    let size = layout.size();
    match size {
        0..=1024 => assert!(free_small(ptr, size), "Pointer not found in Pages"),
        _ => todo!(),
    }
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

    page.meta().release_chunks(start..start + count);

    true
}
