use core::ptr::NonNull;

use crate::sys::mman;

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

    pub const fn meta_mut(&mut self) -> &mut Meta {
        // SAFETY: According to the contract, Meta is always located at the
        // beginning
        unsafe { self.0.cast().as_mut() }
    }

    pub const fn data(&self) -> &[u8; DATA_SIZE] {
        // SAFETY: According to the contract, after Meta, all subsequent bytes
        // on the page are allocated exclusively for data
        unsafe { self.0.cast().add(META_SIZE).as_ref() }
    }

    pub const fn data_mut(&mut self) -> &mut [u8; DATA_SIZE] {
        // SAFETY: According to the contract, after Meta, all subsequent bytes
        // on the page are allocated exclusively for data
        unsafe { self.0.cast().add(META_SIZE).as_mut() }
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

#[repr(transparent)]
pub struct BitMask<const BYTES: usize> {
    inner: [u8; BYTES],
}

impl<const BYTES: usize> BitMask<BYTES> {
    pub const fn new() -> Self {
        Self {
            inner: [0u8; BYTES],
        }
    }

    pub const fn get(&self, pos: usize) -> bool {
        let byte = pos >> 3;
        let bit = pos & 7;
        (self.inner[byte] >> bit) & 1 == 1
    }

    pub const fn set(&mut self, pos: usize) {
        let byte = pos >> 3;
        let bit = pos & 7;
        let mask = 1u8 << bit;
        self.inner[byte] |= mask;
    }

    pub const fn unset(&mut self, pos: usize) {
        let byte = pos >> 3;
        let bit = pos & 7;
        let mask = 1u8 << bit;
        self.inner[byte] &= !mask;
    }
}

impl<const BYTES: usize> Default for BitMask<BYTES> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Meta {
    next: Option<NonNull<Page>>,
    bitmask: BitMask<BITMASK_BYTES>,
}

impl Meta {
    pub const fn next(&self) -> Option<NonNull<Page>> {
        self.next
    }

    /// # Errors
    ///
    /// If the pointer to the next page is already set
    pub const fn set_next(&mut self, ptr: NonNull<Page>) -> Result<(), NonNull<Page>> {
        if let Some(p) = self.next {
            Err(p)
        } else {
            self.next = Some(ptr);
            Ok(())
        }
    }

    pub const fn bitmask(&self) -> &BitMask<BITMASK_BYTES> {
        &self.bitmask
    }

    pub const fn bitmask_mut(&mut self) -> &mut BitMask<BITMASK_BYTES> {
        &mut self.bitmask
    }
}
