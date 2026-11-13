use core::{
    ops::Range,
    ptr::NonNull,
    sync::atomic::{AtomicPtr, AtomicU64, Ordering},
};

use super::sync::OnceLock;
use crate::sys::mman;

pub const PAGE_SIZE: usize = 1 << 12;
pub const CHUNK_SIZE: usize = 64;

pub const PAGE_ALIGN: usize = align_of::<Meta>();
pub const META_ALIGN: usize = if PAGE_ALIGN > CHUNK_SIZE {
    PAGE_ALIGN
} else {
    CHUNK_SIZE
};
pub const META_SIZE: usize = size_of::<Meta>().next_multiple_of(META_ALIGN);
pub const DATA_SIZE: usize = PAGE_SIZE - META_SIZE;
pub const CHUNK_COUNT: usize = DATA_SIZE / CHUNK_SIZE;

pub const BITMASK_BITS: usize = 64;

const _: () = {
    assert!(PAGE_SIZE > META_SIZE);
    assert!(META_SIZE.is_multiple_of(PAGE_ALIGN));
    assert!(PAGE_SIZE.is_multiple_of(PAGE_ALIGN));
    assert!(DATA_SIZE.is_multiple_of(CHUNK_SIZE));
    assert!(CHUNK_COUNT <= BITMASK_BITS);
};

static KING_PAGE: OnceLock<Page> = OnceLock::new();

pub fn king_page() -> Page {
    *KING_PAGE.get_or_init(Page::allocate_new)
}

#[repr(transparent)]
#[derive(Clone, Copy)]
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

    pub const fn data_ptr(&self) -> *mut u8 {
        // SAFETY: According to the contract, after Meta, all subsequent bytes
        // on the page are allocated exclusively for data
        unsafe { self.0.cast::<u8>().as_ptr().add(META_SIZE) }
    }

    pub fn chunk_ptr(&self, idx: usize) -> NonNull<[u8; CHUNK_SIZE]> {
        assert!(idx < CHUNK_COUNT);
        // SAFETY: `idx` will never go beyond the bounds of allocated memory
        unsafe {
            let ptr = self.data_ptr().add(idx * CHUNK_SIZE);
            NonNull::new_unchecked(ptr.cast())
        }
    }

    pub(crate) const fn as_raw(self) -> *mut () {
        self.0.as_ptr()
    }

    /// # Safety
    ///
    /// Ptr must be non-null and point to a page previously produced by
    /// `Page::as_raw`
    pub(crate) const unsafe fn from_raw(ptr: *mut ()) -> Self {
        // SAFETY: caller guarantees `ptr` is non-null and valid
        Self(unsafe { NonNull::new_unchecked(ptr) })
    }
}

// SAFETY: The ptr in Page never changes
unsafe impl Send for Page {}
// SAFETY: The ptr in Page never changes
unsafe impl Sync for Page {}

pub struct Meta {
    next: AtomicPtr<()>,
    bitmask: AtomicU64,
}

impl Meta {
    pub const fn new() -> Self {
        Self {
            next: AtomicPtr::new(core::ptr::null_mut()),
            bitmask: AtomicU64::new(0),
        }
    }

    pub fn next(&self) -> Option<Page> {
        let ptr = self.next.load(Ordering::Acquire);
        if ptr.is_null() {
            None
        } else {
            // SAFETY: `set_next` only stores addresses obtained from
            // `Page::as_raw`, which are always valid page addresses
            Some(unsafe { Page::from_raw(ptr) })
        }
    }

    /// # Errors
    ///
    /// If `next` is already set.
    pub fn set_next(&self, page: Page) -> Result<(), Page> {
        match self.next.compare_exchange(
            core::ptr::null_mut(),
            page.as_raw(),
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => Ok(()),
            // SAFETY: any value stored in `next` came from `Page::as_raw`
            Err(existing) => Err(unsafe { Page::from_raw(existing) }),
        }
    }

    pub fn try_acquire_chunks(&self, range: Range<usize>) -> bool {
        let mask = chunk_mask(&range);
        if mask == 0 {
            return true;
        }
        let mut cur = self.bitmask.load(Ordering::Acquire);
        loop {
            if cur & mask != 0 {
                return false;
            }
            match self.bitmask.compare_exchange_weak(
                cur,
                cur | mask,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(actual) => cur = actual,
            }
        }
    }

    pub fn release_chunks(&self, range: Range<usize>) {
        let mask = chunk_mask(&range);
        if mask == 0 {
            return;
        }
        self.bitmask.fetch_and(!mask, Ordering::AcqRel);
    }

    pub fn is_free(&self, pos: usize) -> bool {
        self.is_free_range(pos..pos + 1)
    }

    pub fn is_free_range(&self, range: Range<usize>) -> bool {
        let mask = chunk_mask(&range);
        self.bitmask.load(Ordering::Acquire) & mask == 0
    }
}

fn chunk_mask(range: &Range<usize>) -> u64 {
    debug_assert!(range.start <= range.end);
    debug_assert!(range.end <= CHUNK_COUNT);
    let width = range.end - range.start;
    if width == 0 {
        return 0;
    }
    ((1u64 << width) - 1) << range.start
}
