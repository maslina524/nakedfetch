use core::{
    ptr::NonNull,
    sync::atomic::{AtomicPtr, AtomicU8, Ordering},
};

use crate::{stdlib::sync::OnceLock, sys::mman};

pub const PAGE_SIZE: usize = 1 << 12;
pub const CHUNK_SIZE: usize = 32;
pub const MAX_CHUNK_COUNT: usize = PAGE_SIZE / CHUNK_SIZE;
pub const BITMASK_BYTES: usize = MAX_CHUNK_COUNT.div_ceil(8);

pub const PAGE_ALIGN: usize = align_of::<Meta>();
pub const META_ALIGN: usize = if PAGE_ALIGN > CHUNK_SIZE {
    PAGE_ALIGN
} else {
    CHUNK_SIZE
};
pub const META_SIZE: usize = size_of::<Meta>().next_multiple_of(META_ALIGN);
pub const DATA_SIZE: usize = PAGE_SIZE - META_SIZE;
pub const CHUNK_COUNT: usize = DATA_SIZE / CHUNK_SIZE;

const _: () = {
    assert!(PAGE_SIZE > META_SIZE);
    assert!(META_SIZE.is_multiple_of(PAGE_ALIGN));
    assert!(PAGE_SIZE.is_multiple_of(PAGE_ALIGN));
    assert!(DATA_SIZE.is_multiple_of(CHUNK_SIZE));
    assert!(CHUNK_COUNT <= BITMASK_BYTES * 8);
};

static KING_PAGE: OnceLock<Page> = OnceLock::new();

pub fn king_page() -> &'static Page {
    KING_PAGE.get_or_init(Page::allocate_new)
}

#[repr(transparent)]
pub struct Page(NonNull<()>);

impl Page {
    pub fn allocate_new() -> Self {
        let ptr = match mman::mmap(None, PAGE_SIZE, 0, 0, -1, 0) {
            Ok(p) => p,
            Err(e) => panic!("Failed to allocate new Page: {e}"),
        };
        Self(ptr)
    }

    pub const fn meta(&self) -> &Meta {
        // SAFETY: According to the contract, Meta is always located at the
        // beginning
        unsafe { self.0.cast().as_ref() }
    }

    pub fn data_ptr(&self) -> *mut u8 {
        // SAFETY: According to the contract, after Meta, all subsequent bytes
        // on the page are allocated exclusively for data
        unsafe { self.0.cast::<u8>().as_ptr().add(META_SIZE) }
    }

    pub fn chunk_ptr(&self, idx: usize) -> NonNull<[u8; CHUNK_SIZE]> {
        assert!(idx < CHUNK_COUNT);
        // SAFETY: idx will never go beyond the bounds of allocated memory
        unsafe {
            let ptr = self.data_ptr().add(idx * CHUNK_SIZE);
            NonNull::new_unchecked(ptr.cast())
        }
    }

    #[allow(
        clippy::forget_non_drop,
        reason = "Do not free the page, the OS will consume it itself,
        core::mem::forget is equivalent to an empty body"
    )]
    pub const fn destruct(self) {
        core::mem::forget(self);
    }
}

unsafe impl Send for Page {}
unsafe impl Sync for Page {}

pub struct Meta {
    next: AtomicPtr<Page>,
    bitmask: [AtomicU8; BITMASK_BYTES],
}

impl Meta {
    pub const fn new() -> Self {
        Self {
            next: AtomicPtr::new(core::ptr::null_mut()),
            bitmask: [const { AtomicU8::new(0) }; BITMASK_BYTES],
        }
    }

    pub fn next(&self) -> Option<NonNull<Page>> {
        NonNull::new(self.next.load(Ordering::Acquire))
    }

    pub fn set_next(&self, ptr: NonNull<Page>) -> Result<(), NonNull<Page>> {
        self.next
            .compare_exchange(
                core::ptr::null_mut(),
                ptr.as_ptr(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map(|_| ())
            .map_err(|p| NonNull::new(p).unwrap())
    }

    pub fn try_acquire_chunk(&self, pos: usize) -> bool {
        let byte = pos >> 3;
        let bit = 1u8 << (pos & 7);
        let mut cur = self.bitmask[byte].load(Ordering::Relaxed);
        loop {
            if cur & bit != 0 {
                return false;
            }
            match self.bitmask[byte].compare_exchange_weak(
                cur,
                cur | bit,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(actual) => cur = actual,
            }
        }
    }

    pub fn release_chunk(&self, pos: usize) {
        let byte = pos >> 3;
        let bit = 1u8 << (pos & 7);
        self.bitmask[byte].fetch_and(!bit, Ordering::AcqRel);
    }

    pub fn is_free(&self, pos: usize) -> bool {
        let byte = pos >> 3;
        let bit = 1u8 << (pos & 7);
        self.bitmask[byte].load(Ordering::Acquire) & bit == 0
    }
}
