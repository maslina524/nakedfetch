use core::mem;

use crate::types::{
    Boolean,
    primitives::{LargeInteger, ULong},
};

// SAFETY: It runs only at compile time, the layout of u32 and FileInformation
// is the same, check whether the WinAPI function will perceive the enum
// class as a number
const _: () = assert!(
    5u32 == unsafe {
        mem::transmute::<FileInformation, u32>(FileInformation::FileStandardInformation)
    }
);

#[repr(u32)]
pub enum FileInformation {
    FileStandardInformation = 5,
}

STRUCT! {
    struct FileStandartInformation {
        allocation_size: LargeInteger,
        end_of_file: LargeInteger,
        number_of_links: ULong,
        delete_pending: Boolean,
        directory: Boolean,
    }
}
