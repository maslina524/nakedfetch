use core::mem;

use crate::types::{BOOLEAN, LARGE_INTEGER, ULONG};

// SAFETY: It runs only at compile time, the layout of u32 and FileInformation
// is the same, check whether the WinAPI function will perceive the enum
// class as a number
const _: () = assert!(
    5u32 == unsafe {
        mem::transmute::<FILE_INFORMATION_CLASS, u32>(
            FILE_INFORMATION_CLASS::FileStandardInformation,
        )
    }
);

#[repr(u32)]
pub enum FILE_INFORMATION_CLASS {
    FileStandardInformation = 5,
}

define_struct!(
    typedef struct _FILE_STANDARD_INFORMATION {
        LARGE_INTEGER AllocationSize;
        LARGE_INTEGER EndOfFile;
        ULONG NumberOfLinks;
        BOOLEAN DeletePending;
        BOOLEAN Directory;
    } FILE_STANDARD_INFORMATION, PFILE_STANDARD_INFORMATION;
);
