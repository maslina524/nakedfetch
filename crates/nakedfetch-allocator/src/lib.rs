#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::{
    alloc::{GlobalAlloc, Layout},
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

use common::allocator::Allocator as AllocatorInner;

pub mod snapshot;
pub use snapshot::Snapshot;

pub(crate) static ALLOC_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub(crate) static REALLOC_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub(crate) static DEALLOC_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub(crate) static ALLOCATED_TOTAL: AtomicUsize = AtomicUsize::new(0);
pub(crate) static DEALLOCATED_TOTAL: AtomicUsize = AtomicUsize::new(0);

pub(crate) static CURRENT_ALLOCATED: AtomicUsize = AtomicUsize::new(0);
pub(crate) static MAX_IN_RUNTIME: AtomicUsize = AtomicUsize::new(0);

fn allocated(size: usize) {
    ALLOCATED_TOTAL.fetch_add(size, Relaxed);
    let cur = CURRENT_ALLOCATED.fetch_add(size, Relaxed) + size;
    MAX_IN_RUNTIME.fetch_max(cur, Relaxed);
}

fn deallocated(size: usize) {
    DEALLOCATED_TOTAL.fetch_add(size, Relaxed);
    let _ = CURRENT_ALLOCATED.try_update(Relaxed, Relaxed, |cur| Some(cur.saturating_sub(size)));
}

pub struct Allocator;

// SAFETY: All unsafe code has SAFETY comments
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: Call in unsafe fn; see the comment in `AllocatorInner`
        let ptr = unsafe { AllocatorInner::alloc(&AllocatorInner, layout) };

        ALLOC_COUNTER.fetch_add(1, Relaxed);
        allocated(layout.size());
        ptr.cast::<u8>()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: Call in unsafe fn; see the comment in `AllocatorInner`
        unsafe {
            AllocatorInner::dealloc(&AllocatorInner, ptr, layout);
        }

        DEALLOC_COUNTER.fetch_add(1, Relaxed);
        deallocated(layout.size());
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: Call in unsafe fn; see the comment in `AllocatorInner`
        let ptr = unsafe { AllocatorInner::alloc_zeroed(&AllocatorInner, layout) };

        ALLOC_COUNTER.fetch_add(1, Relaxed);
        allocated(layout.size());
        ptr.cast::<u8>()
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: Call in unsafe fn; see the comment in `AllocatorInner`
        let new_ptr = unsafe { AllocatorInner::realloc(&AllocatorInner, ptr, layout, new_size) };

        REALLOC_COUNTER.fetch_add(1, Relaxed);

        let old_size = layout.size();
        if new_size > old_size {
            allocated(new_size - old_size);
        } else {
            deallocated(old_size - new_size);
        }

        new_ptr.cast::<u8>()
    }
}
