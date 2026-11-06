use core::{
    alloc::Layout,
    error::Error,
    ffi::c_void,
    fmt,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

use winapi::{
    AsRawHandle as _,
    win32::heapapi::{
        AllocateError, GetProcessHeap, HeapAlloc, HeapFlags, HeapFree, HeapHandle, HeapPtr,
        HeapReAlloc,
    },
};

static HANDLE: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy)]
pub struct DeallocError;

impl fmt::Display for DeallocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for DeallocError {}

#[derive(Debug, Clone, Copy)]
pub struct ReAllocError;

impl fmt::Display for ReAllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ReAllocError {}

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

pub fn alloc(layout: Layout) -> Result<*mut u8, AllocateError> {
    HeapAlloc(handle(), HeapFlags::alloc(), layout.size()).map(HeapPtr::into_ptr)
}

pub fn dealloc(ptr: *mut u8, _layout: Layout) -> Result<(), DeallocError> {
    // SAFETY: `ptr` passed to dealloc must point into the
    // initialized part of the heap; using the stable
    // standard library this shouldn't cause problems
    let heapptr = unsafe { HeapPtr::new(ptr) };
    HeapFree(handle(), HeapFlags::free(), heapptr).map_err(|_| DeallocError)
}

pub fn alloc_zeroed(layout: Layout) -> Result<*mut u8, AllocateError> {
    HeapAlloc(handle(), HeapFlags::alloc().zero_memory(), layout.size()).map(HeapPtr::into_ptr)
}

pub fn realloc(ptr: *mut u8, _layout: Layout, new_size: usize) -> Result<*mut u8, ReAllocError> {
    // SAFETY: `ptr` passed to dealloc must point into the
    // initialized part of the heap; using the stable
    // standard library this shouldn't cause problems
    let heapptr = unsafe { HeapPtr::new(ptr) };
    HeapReAlloc(handle(), HeapFlags::realloc(), heapptr, new_size)
        .map(HeapPtr::into_ptr)
        .map_err(|_| ReAllocError)
}
