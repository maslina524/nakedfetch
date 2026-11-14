use core::{
    alloc::Layout,
    ffi::c_void,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

use winapi::{
    AsRawHandle as _,
    win32::heapapi::{
        GetProcessHeap, HeapAlloc, HeapFlags, HeapFree, HeapHandle, HeapPtr, HeapReAlloc,
    },
};

static HANDLE: AtomicUsize = AtomicUsize::new(0);

fn handle() -> HeapHandle {
    let atomic = HANDLE.load(Relaxed);
    if atomic != 0 {
        let hmodule = atomic as *mut c_void;
        return HeapHandle::from_raw(hmodule);
    }

    let handle = GetProcessHeap();
    let hmodule = handle.as_raw();
    HANDLE.store(hmodule as usize, Relaxed);
    handle
}

pub fn alloc(layout: Layout) -> *mut u8 {
    let heapptr = HeapAlloc(handle(), HeapFlags::alloc(), layout.size())
        .unwrap_or_else(|e| panic!("Failed to alloc: {e}"));
    heapptr.as_ptr()
}

pub fn dealloc(ptr: *mut u8, _layout: Layout) {
    // SAFETY: `ptr` passed to dealloc must point into the
    // initialized part of the heap; using the stable
    // standard library this shouldn't cause problems
    let heapptr = unsafe { HeapPtr::new(ptr) };
    HeapFree(handle(), HeapFlags::free(), heapptr).unwrap_or_else(|_| panic!("Failed to dealloc"));
}

pub fn alloc_zeroed(layout: Layout) -> *mut u8 {
    let heapptr = HeapAlloc(handle(), HeapFlags::alloc().zero_memory(), layout.size())
        .unwrap_or_else(|e| panic!("Failed to alloc (zeroed): {e}"));
    heapptr.as_ptr()
}

pub fn realloc(ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
    // SAFETY: `ptr` passed to dealloc must point into the
    // initialized part of the heap; using the stable
    // standard library this shouldn't cause problems
    let heapptr = unsafe { HeapPtr::new(ptr) };
    let new_heapptr = HeapReAlloc(handle(), HeapFlags::realloc(), heapptr, new_size)
        .unwrap_or_else(|_| panic!("Failed to realloc"));

    new_heapptr.as_ptr()
}
