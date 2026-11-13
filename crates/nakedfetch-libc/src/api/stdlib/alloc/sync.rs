use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicU8, Ordering},
};

const INCOMPLETE: u8 = 0;
const INITIALIZING: u8 = 1;
const READY: u8 = 2;

#[derive(Debug)]
pub struct OnceLock<T> {
    state: AtomicU8,
    value: UnsafeCell<MaybeUninit<T>>,
}

impl<T> OnceLock<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(INCOMPLETE),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn get_or_init(&self, f: impl FnOnce() -> T) -> &T {
        if self.state.load(Ordering::Acquire) == READY {
            // SAFETY: The value in `MaybeUninit` is guaranteed to be
            // initialized
            unsafe {
                return (&*self.value.get()).assume_init_ref();
            }
        }

        if self
            .state
            .compare_exchange(
                INCOMPLETE,
                INITIALIZING,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
        {
            let value = f();

            // SAFETY: In write we initialize the value, and
            // then call an unsafe function that
            // is completely safe after initialization
            unsafe {
                (*self.value.get()).write(value);
            }
            self.state.store(READY, Ordering::Release);

            // SAFETY: The value in `MaybeUninit` is guaranteed to be
            // initialized
            unsafe { (&*self.value.get()).assume_init_ref() }
        } else {
            while self.state.load(Ordering::Acquire) != READY {
                core::hint::spin_loop();
            }

            // SAFETY: The value in `MaybeUninit` is guaranteed to be
            // initialized
            unsafe { (&*self.value.get()).assume_init_ref() }
        }
    }
}

impl<T> Default for OnceLock<T> {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: trait is empty
unsafe impl<T: Sync> Sync for OnceLock<T> {}
