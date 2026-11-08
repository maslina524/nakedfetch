#[allow(
    clippy::wildcard_imports,
    reason = "There's no point in importing them separately"
)]
use super::*;

const PROCESSOR_FEATURE_MAX: usize = 64;
const KUSER_SHARED_DATA_PTR: *mut KUSER_SHARED_DATA = 0x7FFE_0000 as *mut _;

STRUCT! {
    Naked
    struct KUSER_SHARED_DATA {
        tick_count_low_deprecated: ULONG,
        tick_count_multiplier: ULONG,
        interrupt_time: KSYSTEM_TIME,
        system_time: KSYSTEM_TIME,
        time_zone_bias: KSYSTEM_TIME,
        image_number_low: USHORT,
        image_number_high: USHORT,
        nt_system_root: [WCHAR; 260],
        max_stack_trace_depth: ULONG,
        crypto_exponent: ULONG,
        time_zone_id: ULONG,
        large_page_minimum: ULONG,
        ait_sampling_value: ULONG,
        app_compat_flag: ULONG,
        rng_seed_version: ULONGLONG,
        global_validation_runlevel: ULONG,
        time_zone_bias_stamp: LONG,
        nt_build_number: ULONG,
        nt_product_type: NtProductType,
        product_type_is_valid: BOOLEAN,
        reserved0: [BOOLEAN; 1],
        native_processor_architecture: USHORT,
        nt_major_version: ULONG,
        nt_minor_version: ULONG,
        processor_features: [BOOLEAN; PROCESSOR_FEATURE_MAX],
        reserved1: ULONG,
        reserved3: ULONG,
        time_slip: ULONG,
        alternative_architecture: AlternativeArchitectureType,
        boot_id: USHORT,
        system_expiration_date: i64,
        suite_mask: ULONG,
        kd_debugger_enabled: BOOLEAN,
        mitigation_policies: MitigationPolicies,
        cycles_per_yield: USHORT,
        active_console_id: ULONG,
        dismount_count: ULONG,
        com_plus_package: ULONG,
        last_system_rit_event_tick_count: ULONG,
        number_of_physical_pages: ULONG,
        safe_boot_mode: BOOLEAN,
        virtualization_flags: VirtualizationFlags,
        reserved12: [UCHAR; 2],
        share_data_flags: ULONG, // FIXME: Replace to bitflag struct
        data_flags_pad: [ULONG; 1],
        test_ret_instruction: ULONGLONG,
        qpc_frequency: LONGLONG,
        system_call: ULONG,
        reserved2: ULONG,
        full_number_of_physical_pages: ULONGLONG,
        system_call_pad: [ULONGLONG; 1],
        tick_count: KSYSTEM_TIME
    }
}

pub type PKUSER_SHARED_DATA = &'static KUSER_SHARED_DATA;

#[must_use]
pub fn kuser_share_data() -> PKUSER_SHARED_DATA {
    // SAFETY: The ptr is static for all versions of Windows, safe
    unsafe { &*KUSER_SHARED_DATA_PTR }
}
