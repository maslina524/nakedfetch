use core::{marker::PhantomData, ptr::NonNull};

use crate::{AsRawHandle, ntdll, types::Handle};

mod HeapAlloc;
pub use HeapAlloc::HeapAlloc;

mod_sealed!();
pub trait FlagTag: sealed::Sealed {}

pub struct Alloc;
pub struct Free;
pub struct ReAlloc;

impl_sealed!(Alloc, Free, ReAlloc);
impl_tag!(FlagTag: Alloc, Free, ReAlloc);

#[derive(Clone, Copy)]
pub struct HeapFlags<T: FlagTag> {
    val: u32,
    _marker: PhantomData<T>,
}

impl<T: FlagTag> HeapFlags<T> {
    const HEAP_GENERATE_EXCEPTIONS: u32 = 0b100;
    const HEAP_NO_SERIALIZE: u32 = 0b1;
    const HEAP_REALLOC_IN_PLACE_ONLY: u32 = 0b10000;
    const HEAP_ZERO_MEMORY: u32 = 0b1000;

    #[must_use]
    pub const fn build(self) -> u32 {
        self.val
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct HeapPtr<T: Sized>(NonNull<T>);

impl<T> HeapPtr<T> {
    /// # Safety
    ///
    /// `ptr` must be non-null and point to a live allocation obtained
    /// from the same Windows heap that will later be used to free
    /// it. If `T` has alignment stricter than the heap
    /// guarantees, the caller must ensure the pointer is properly
    /// aligned for `T`.
    pub const unsafe fn new(ptr: *mut T) -> Self {
        // SAFETY: null == does not point to the heap, read Safety
        let nonnull = unsafe { NonNull::new_unchecked(ptr) };
        Self(nonnull)
    }

    #[must_use]
    pub const fn as_non_null(&self) -> NonNull<T> {
        self.0
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    #[must_use]
    pub const fn into_ptr(self) -> *mut T {
        self.0.as_ptr()
    }

    #[must_use]
    pub const fn cast<U>(self) -> HeapPtr<U> {
        let nonnull = self.as_non_null().cast::<U>();
        HeapPtr(nonnull)
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct HeapHandle(Handle);

impl AsRawHandle for HeapHandle {
    fn from_raw(val: Handle) -> Self {
        Self(val)
    }

    fn as_raw(&self) -> Handle {
        self.0
    }
}

#[must_use]
pub fn GetProcessHeap() -> HeapHandle {
    let peb = ntdll::RtlGetCurrentPeb();
    let raw = peb.process_heap;
    HeapHandle::from_raw(raw)
}
