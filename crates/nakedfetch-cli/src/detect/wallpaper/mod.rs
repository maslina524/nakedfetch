use path::SmolPath;

use crate::cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
    } else if #[cfg(target_os = "android")] {
        mod android;
    }
}

pub struct WallpaperInfo {
    pub full_path: SmolPath,
}
