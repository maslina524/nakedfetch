#[cfg(target_os = "windows")]
mod windows {
    pub type PathInner = wtf8::Wtf8;

    pub static SEPARATOR: [u8; 2] = *b"/\\";
}
pub use windows::*;
