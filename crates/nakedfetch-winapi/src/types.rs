pub mod primitives {
    pub type Short = i16;
    pub type UShort = u16;

    pub type Long = i32;
    pub type ULong = u32;

    pub type LongLong = i64;
    pub type ULongLong = u64;
}

pub mod chars {
    pub type WChar = u16;
}

STRUCT! {
    struct KSystemTime {
        low_part: ULong,
        high1_time: Long,
        high2_time: Long,
    }
}
