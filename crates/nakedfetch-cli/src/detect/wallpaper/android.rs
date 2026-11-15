use path::SmolPath;

use crate::detect::wallpaper::WallpaperInfo;

impl WallpaperInfo {
    pub const fn new() -> Self {
        Self {
            full_path: SmolPath::empty(),
        }
    }
}
