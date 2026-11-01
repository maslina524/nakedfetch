pub struct Utf16Iter<'wtf8> {
    bytes: &'wtf8 [u8],
    pos: usize,
    zero: bool
}

