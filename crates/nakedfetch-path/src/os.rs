#[cfg(target_os = "windows")]
mod windows {
    pub type PathInner = wtf8::Wtf8;

    pub static SEPARATOR: [char; 2] = ['/', '\\'];
    pub static INVALID_SYMBOLS: [char; 5] = ['<', '>', '|', '?', '*'];
    pub static INVALID_NAMES: [&str; 22] = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
}
pub use windows::*;
