use crate::types::primitives::{Long, ULong};

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
