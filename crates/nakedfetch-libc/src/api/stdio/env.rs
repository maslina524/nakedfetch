use alloc::vec::Vec;
use core::{ffi::CStr, sync::atomic::Ordering};

use crate::unistd;

pub fn getenv(name: &CStr) -> Option<Vec<u8>> {
    let name_bytes = name.to_bytes();
    let name_len = name.count_bytes();
    if name_len == 0 {
        return None;
    }

    let mut pairs = unistd::ENVIRON.load(Ordering::Acquire);
    loop {
        // SAFETY: According to POSIX, the pointer is always valid
        let pair_ptr = unsafe { pairs.read() };
        if pair_ptr.is_null() {
            return None;
        }

        // SAFETY: A valid `CStr` is always stored.
        let c_pair = unsafe { CStr::from_ptr(pair_ptr) };
        let pair_bytes = c_pair.to_bytes();

        if pair_bytes.len() <= name_len
            || name_bytes != &pair_bytes[..name_len]
            || pair_bytes[name_len] != b'='
        {
            // SAFETY: Search in the next pair, if the pairs are exhausted, then
            // according to POSIX the next one is NUL, we exit the loop.
            pairs = unsafe { pairs.add(1) };
            continue;
        }

        let slice = &pair_bytes[name_len + 1..];
        return Some(Vec::from(slice));
    }
}
