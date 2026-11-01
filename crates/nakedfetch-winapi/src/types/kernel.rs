use crate::{
    ntstatus::Ntstatus,
    types::{
        primitives::{Long, ULong, UShort},
        ptr::{ULongPtr, VoidPtr, WideStrPtr},
    },
};

pub type Handle = VoidPtr;

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

STRUCT! {
    #[derive(Debug)]
    struct IoStatusBlock {
        union: IoStatusBlockInner,
        information: ULongPtr,
    }
}

STRUCT! {
    #[derive(Debug)]
    struct RawUnicodeString {
        length: UShort,
        maximum_length: UShort,
        buffer: WideStrPtr
    }
}

UNION! {
    #[derive(Debug)]
    union IoStatusBlockInner {
        status: Ntstatus,
        pointer: VoidPtr,
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
