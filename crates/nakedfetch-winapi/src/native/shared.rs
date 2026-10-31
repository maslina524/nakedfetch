use crate::types::{
    AlternativeArchitectureType, Boolean, KSystemTime, MitigationPolicies, NtProductType,
    VirtualizationFlags,
    chars::{UChar, WChar},
    primitives::{LargeInteger, Long, LongLong, ULong, ULongLong, UShort},
};

const PROCESSOR_FEATURE_MAX: usize = 64;
const KUSER_SHARED_DATA_PTR: *mut KUserSharedData = 0x7FFE_0000 as *mut _;

STRUCT! {
    struct KUserSharedData {
        tick_count_low_deprecated: ULong,
        tick_count_multiplier: ULong,
        interrupt_time: KSystemTime,
        system_time: KSystemTime,
        time_zone_bias: KSystemTime,
        image_number_low: UShort,
        image_number_high: UShort,
        nt_system_root: [WChar; 260],
        max_stack_trace_depth: ULong,
        crypto_exponent: ULong,
        time_zone_id: ULong,
        large_page_minimum: ULong,
        ait_sampling_value: ULong,
        app_compat_flag: ULong,
        rng_seed_version: ULongLong,
        global_validation_runlevel: ULong,
        time_zone_bias_stamp: Long,
        nt_build_number: ULong,
        nt_product_type: NtProductType,
        product_type_is_valid: Boolean,
        reserved0: [Boolean; 1],
        native_processor_architecture: UShort,
        nt_major_version: ULong,
        nt_minor_version: ULong,
        processor_features: [Boolean; PROCESSOR_FEATURE_MAX],
        reserved1: ULong,
        reserved3: ULong,
        time_slip: ULong,
        alternative_architecture: AlternativeArchitectureType,
        boot_id: UShort,
        system_expiration_date: LargeInteger,
        suite_mask: ULong,
        kd_debugger_enabled: Boolean,
        mitigation_policies: MitigationPolicies,
        cycles_per_yield: UShort,
        active_console_id: ULong,
        dismount_count: ULong,
        com_plus_package: ULong,
        last_system_rit_event_tick_count: ULong,
        number_of_physical_pages: ULong,
        safe_boot_mode: Boolean,
        virtualization_flags: VirtualizationFlags,
        reserved12: [UChar; 2],
        share_data_flags: ULong, // FIXME: Replace to bitflag struct
        data_flags_pad: [ULong; 1],
        test_ret_instruction: ULongLong,
        qpc_frequency: LongLong,
        system_call: ULong,
        reserved2: ULong,
        full_number_of_physical_pages: ULongLong,
        system_call_pad: [ULongLong; 1],
        tick_count: KSystemTime
    }
}

pub fn kuser_share_data() -> &'static KUserSharedData {
    // SAFETY: The ptr is static for all versions of Windows, safe
    unsafe { &*KUSER_SHARED_DATA_PTR }
}
