use crate::types::{
    Boolean, Handle,
    ptr::{HeapPtr, PebLdrDataRef, RtlUserProcessParametersRef, VoidPtr},
};

STRUCT! {
    Naked
    struct Peb {
        inherited_address_space: Boolean,
        read_image_file_exec_options: Boolean,
        being_debugged: Boolean,
        bit_field: Boolean, // FIXME: Replace to BitFiled
        mutant: Handle,
        image_base_address: VoidPtr,
        ldr: PebLdrDataRef,
        process_parameters: RtlUserProcessParametersRef,
        sub_system_data: VoidPtr,
        process_heap: HeapPtr,
    }
}
