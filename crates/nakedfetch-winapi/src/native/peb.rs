use super::{BOOLEAN, HANDLE, PHEAP, PPEB_LDR_DATA, PRTL_USER_PROCESS_PARAMETERS, PVOID};

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
