use crate::{
    ntstatus::Ntstatus,
    types::{
        primitives::{LargeInteger, Long, ULong, UShort},
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

STRUCT! {
    #[derive(Debug)]
    struct ObjectAttributes {
        length: ULong,
        root_directory: Handle,
        object_name: *const RawUnicodeString,
        attributes: ULong,
        security_descriptor: *mut (),
        security_quality_of_service: *mut ()
    }
}

STRUCT! {
    #[derive(Debug)]
    struct FileBasicInfo {
        creation_time: LargeInteger,
        last_access_time: LargeInteger,
        last_write_time: LargeInteger,
        change_time: LargeInteger,
        file_attributes: ULong,
    }
}

UNION! {
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
