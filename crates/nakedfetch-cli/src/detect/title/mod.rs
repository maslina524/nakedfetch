use alloc::string::String;

use path::SmolPath;

crate::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_family = "unix")] {
        mod unix;
    }
}

pub struct TitleInfo {
    pub user_name: String,
    pub host_name: String,
    pub home_dir: SmolPath,
    pub exe_path: SmolPath,
    pub user_shell: SmolPath,
    pub user_name_colored: String,
    pub host_name_colored: String,
    pub full_user_name: String,
    pub user_id: String,
    pub pid: u32,
    pub cwd: SmolPath,
}
