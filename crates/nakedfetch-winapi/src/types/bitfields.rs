use crate::types::chars;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct MitigationPolicies(chars::UCHAR);

impl MitigationPolicies {
    #[must_use]
    pub const fn nx_support_policy(self) -> u8 {
        self.0 & 0b11
    }

    #[must_use]
    pub const fn seh_validation_policy(self) -> u8 {
        (self.0 >> 2) & 0b11
    }

    #[must_use]
    pub const fn cur_dir_devices_skipped_for_dlls(self) -> u8 {
        (self.0 >> 4) & 0b11
    }

    #[must_use]
    pub const fn reserved(self) -> u8 {
        (self.0 >> 6) & 0b11
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct VirtualizationFlags(chars::UCHAR);

impl VirtualizationFlags {
    #[must_use]
    pub const fn arch_started_in_el2(self) -> u8 {
        self.0 & 0b1
    }

    #[must_use]
    pub const fn qc_sl_is_supported(self) -> u8 {
        (self.0 >> 1) & 0b1
    }
}
