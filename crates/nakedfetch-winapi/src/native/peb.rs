#![allow(
    clippy::upper_case_acronyms,
    non_camel_case_types,
    reason = "Windows style"
)]

use crate::{
    native::rtl::RtlUserProcessParameters,
    types::{Boolean, Handle, ptr},
};

type BOOLEAN = Boolean;
type HANDLE = Handle;
type PVOID = ptr::VoidPtr;
type PPEB_LDR_DATA = &'static core::ffi::c_void;
type PHEAP = *mut core::ffi::c_void;
type PRTL_USER_PROCESS_PARAMETERS = &'static RtlUserProcessParameters;
define_struct!(
    typedef struct Peb {
        //
        // The process was cloned with an inherited address space.
        //
        BOOLEAN InheritedAddressSpace;

        //
        // The process has image file execution options (IFEO).
        //
        BOOLEAN ReadImageFileExecOptions;

        //
        // The process has a debugger attached.
        //
        BOOLEAN BeingDebugged;

        u8 BitField;

        //
        // Handle to a mutex for synchronization.
        //
        HANDLE Mutant;

        //
        // Pointer to the base address of the process image.
        //
        PVOID ImageBaseAddress;

        //
        // Pointer to the process loader data.
        //
        PPEB_LDR_DATA Ldr;

        //
        // Pointer to the process parameters.
        //
        PRTL_USER_PROCESS_PARAMETERS ProcessParameters;

        //
        // Reserved.
        //
        PVOID SubSystemData;

        //
        // Pointer to the process default heap.
        //
        PHEAP ProcessHeap;
    };
);
