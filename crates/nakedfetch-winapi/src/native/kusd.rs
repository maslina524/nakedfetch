use crate::types::{
    KSystemTime, NtProductType,
    chars::WChar,
    primitives::{ULong, UShort},
};

const PROCESSOR_FEATURE_MAX: usize = 64;

STRUCT! {
    struct _KUSER_SHARED_DATA {
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
        union {
            UCHAR VirtualizationFlags;
            struct {
                UCHAR ArchStartedInEl2 : 1;
                UCHAR QcSlIsSupported : 1;
            };
        };
        UCHAR                         Reserved12[2];
        union {
            ULONG SharedDataFlags;
            struct {
                ULONG DbgErrorPortPresent : 1;
                ULONG DbgElevationEnabled : 1;
                ULONG DbgVirtEnabled : 1;
                ULONG DbgInstallerDetectEnabled : 1;
                ULONG DbgLkgEnabled : 1;
                ULONG DbgDynProcessorEnabled : 1;
                ULONG DbgConsoleBrokerEnabled : 1;
                ULONG DbgSecureBootEnabled : 1;
                ULONG DbgMultiSessionSku : 1;
                ULONG DbgMultiUsersInSessionSku : 1;
                ULONG DbgStateSeparationEnabled : 1;
                ULONG SpareBits : 21;
            } DUMMYSTRUCTNAME2;
        } DUMMYUNIONNAME2;
        ULONG                         DataFlagsPad[1];
        ULONGLONG                     TestRetInstruction;
        LONGLONG                      QpcFrequency;
        ULONG                         SystemCall;
        ULONG                         Reserved2;
        ULONGLONG                     FullNumberOfPhysicalPages;
        ULONGLONG                     SystemCallPad[1];
        union {
            KSYSTEM_TIME TickCount;
            ULONG64      TickCountQuad;
            struct {
                ULONG ReservedTickCountOverlay[3];
                ULONG TickCountPad[1];
            } DUMMYSTRUCTNAME;
        }
    }
}
