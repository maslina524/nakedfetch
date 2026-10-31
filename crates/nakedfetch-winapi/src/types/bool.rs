#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Boolean(u8);

impl From<Boolean> for bool {
    fn from(val: Boolean) -> Self {
        val.0 > 0
    }
}
