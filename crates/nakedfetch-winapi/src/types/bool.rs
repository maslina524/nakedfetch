#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct BOOLEAN(u8);

impl From<BOOLEAN> for bool {
    fn from(val: BOOLEAN) -> Self {
        val.0 > 0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct LOGICAL(u32);

impl From<LOGICAL> for bool {
    fn from(val: LOGICAL) -> Self {
        val.0 > 0
    }
}
