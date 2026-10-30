#[derive(Debug, Clone, Copy)]
pub struct MitigationPolicies(chars::UChar);

impl MitigationPolicies {
    pub fn nx_support_policy(&self) -> u8 {
        self.0 & 0b11
    }

    pub fn seh_validation_policy(&self) -> u8 {
        (self.0 >> 2) & 0b11
    }

    pub fn cur_dir_devices_skipped_for_dlls(&self) -> u8 {
        (self.0 >> 4) & 0b11
    }

    pub fn reserved(&self) -> u8 {
        (self.0 >> 6) & 0b11
    }
}

STRUCT! {
    struct KSystemTime {
        low_part: ULong,
        high1_time: Long,
        high2_time: Long,
    }
}

ENUM! {
    enum NtProductType {
        WinNt = 1,
        LanManNt = 2,
        Server = 3,
    }
}

ENUM! {
    enum AlternativeArchitectureType {
        StandardDesign,
        NEC98x86,
        EndAlternatives
    }
}
