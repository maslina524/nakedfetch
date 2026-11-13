use core::{alloc::Layout, ptr::NonNull};

use crate::sys::mman;

pub const PAGE_SIZE: usize = 1 << 12;
pub const PAGE_ALIGN: usize = align_of::<Meta>();
pub const META_SIZE: usize = size_of::<Meta>().next_multiple_of(PAGE_ALIGN);
pub const DATA_SIZE: usize = PAGE_SIZE - META_SIZE;

const _: () = {
    assert!(PAGE_SIZE > META_SIZE);
    assert!(META_SIZE.is_multiple_of(PAGE_ALIGN));
    assert!(PAGE_SIZE.is_multiple_of(PAGE_ALIGN));
};

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

pub struct Meta {
    next: Option<NonNull<Page>>,
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
}

#[must_use]
pub const fn malloc(_layout: Layout) -> NonNull<()> {
    NonNull::dangling()
}
