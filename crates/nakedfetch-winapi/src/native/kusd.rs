use crate::types::{
    primitives::{ULong, UShort},
    chars::WChar,
    KSystemTime
};

STRUCT! {
    struct _KUSER_SHARED_DATA {
        TickCountLowDeprecated: ULong,
        TickCountMultiplier: ULong,
        InterruptTime: KSystemTime,
        SystemTime: KSystemTime,
        TimeZoneBias: KSystemTime,
        ImageNumberLow: UShort,
        ImageNumberHigh: UShort,
        NtSystemRoot: [WChar; 260],
        MaxStackTraceDepth: ULong,
        CryptoExponent: ULong,
        TimeZoneId: ULong,
        LargePageMinimum: ULong,
        AitSamplingValue: ULong,
        AppCompatFlag: ULong,
        RNGSeedVersion: ULongLong,
        GlobalValidationRunlevel: ULong,
        TimeZoneBiasStamp: Long,
        NtBuildNumber: ULong,
        NT_PRODUCT_TYPE               NtProductType,
        BOOLEAN                       ProductTypeIsValid,
        BOOLEAN                       Reserved0[1],
        NativeProcessorArchitecture: UShort,
        NtMajorVersion: ULong,
        NtMinorVersion: ULong,
        BOOLEAN                       ProcessorFeatures[PROCESSOR_FEATURE_MAX],
        Reserved1: ULong,
        Reserved3: ULong,
        TimeSlip: ULong,
        ALTERNATIVE_ARCHITECTURE_TYPE AlternativeArchitecture,
        BootId: UShort,
        LARGE_INTEGER                 SystemExpirationDate,
        SuiteMask: ULong,
        BOOLEAN                       KdDebuggerEnabled,
        union {
            UCHAR MitigationPolicies;
            struct {
                UCHAR NXSupportPolicy : 2;
                UCHAR SEHValidationPolicy : 2;
                UCHAR CurDirDevicesSkippedForDlls : 2;
                UCHAR Reserved : 2;
            };
        };
        USHORT                        CyclesPerYield;
        ULONG                         ActiveConsoleId;
        ULONG                         DismountCount;
        ULONG                         ComPlusPackage;
        ULONG                         LastSystemRITEventTickCount;
        ULONG                         NumberOfPhysicalPages;
        BOOLEAN                       SafeBootMode;
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
};