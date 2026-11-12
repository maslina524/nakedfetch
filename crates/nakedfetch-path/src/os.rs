#[cfg(target_os = "windows")]
mod windows {
    pub type OsStr = wtf8::Wtf8;
    pub type OsString = wtf8::Wtf8Buf;
    pub type SmolOsInner = wtf8::SmolWtf8;

    pub static SEPARATOR: [char; 2] = ['/', '\\'];
    pub static INVALID_SYMBOLS: &[char] = &['<', '>', '|', '?', '*'];
    pub static INVALID_NAMES: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
}
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_family = "unix")]
mod unix {
    use alloc::vec::Vec;

    pub type OsStr = [u8];
    pub type OsString = Vec<u8>;

    pub static SEPARATOR: [char; 1] = ['/'];
    pub static INVALID_SYMBOLS: &[char] = &[];
    pub static INVALID_NAMES: &[&str] = &[];
}
#[cfg(target_family = "unix")]
pub use unix::*;
