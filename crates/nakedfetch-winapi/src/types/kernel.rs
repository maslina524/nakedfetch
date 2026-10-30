use crate::types::primitives::{Long, ULong};

STRUCT! {
    struct KSystemTime {
        low_part: ULong,
        high1_time: Long,
        high2_time: Long,
    }
}

impl KSystemTime {
    #[must_use]
    pub const fn quad(&self) -> u64 {
        ((self.high1_time as u64) << 32) | (self.low_part as u64)
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
