use crate::types::chars;

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

#[derive(Debug, Clone, Copy)]
pub struct VirtualizationFlags(chars::UChar);

impl VirtualizationFlags {
    pub fn arch_started_in_el2(&self) -> u8 {
        self.0 & 0b1
    }

    pub fn qc_sl_is_supported(&self) -> u8 {
        (self.0 >> 1) & 0b1
    }
}
