#[derive(Debug, Clone, Copy)]
pub struct Boolean(u8);

impl Into<bool> for Boolean {
    fn into(self) -> bool {
        self.0 > 0
    }
}
