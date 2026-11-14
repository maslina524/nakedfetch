use core::ffi::{c_char, CStr};

use alloc::string::{String, ToString};

use path::{Path, SmolPath};
use common::{abort, warning, formatf};

use crate::{
    detect::title::TitleInfo,
    imp::fs,
    imp::libc::{getcwd, gethostname, getpid, getpwuid, getuid},
    unix::error::ErrorCode,
    logo::LogoInfo,
};

const HOST_NAME_MAX: usize = 64;
const PATH_MAX: usize = 4096;

impl TitleInfo {
    pub fn new() -> Self {
        let uid = getuid();
        let pw_ptr = getpwuid(uid);
        if pw_ptr.is_null() {
            abort!("Failed to call `getpwuid`: {}", ErrorCode::last());
        }
        // SAFETY: Libc always returns a valid pointer
        let pw = unsafe { &*pw_ptr };

        // SAFETY: Libs are guaranteed to store a valid cstr
        let user_name_c_str = unsafe { CStr::from_ptr(pw.pw_name) };
        let user_name = user_name_c_str.to_string_lossy().into_owned();

        let user_id = pw.pw_uid.to_string();
        let pid = getpid() as u32;

        // SAFETY: Libs are guaranteed to store a valid cstr
        let home_dir_c_str = unsafe { CStr::from_ptr(pw.pw_dir) };
        let home_dir = SmolPath::from(home_dir_c_str);

        // SAFETY: Libs are guaranteed to store a valid cstr
        let user_shell_c_str = unsafe { CStr::from_ptr(pw.pw_shell) };
        let user_shell = SmolPath::from(user_shell_c_str);

        let host_name = Self::host_name();
        let exe_path = Self::exe_path();

        let user_name_colored = Self::colored(&user_name);
        let host_name_colored = Self::colored(&host_name);

        let cwd = Self::cwd();

        Self {
            user_name,
            host_name,
            home_dir,
            exe_path,
            user_shell,
            user_name_colored,
            host_name_colored,
            full_user_name: String::new(),
            user_id,
            pid,
            cwd,
        }
    }

    fn cwd() -> SmolPath {
        let mut buf = [0i8; PATH_MAX];
        let ret = getcwd(buf.as_mut_ptr().cast(), PATH_MAX);
        if ret.is_null() {
            warning!("Failed to call `getcwd`");
            return SmolPath::empty();
        }

        // SAFETY: Libs are guaranteed to store a valid cstr
        let c_str = unsafe { CStr::from_ptr(buf.as_ptr().cast()) };
        SmolPath::from(c_str)
    }

    fn colored(s: &str) -> String {
        formatf!(
            "\x1b[1;{}m{}\x1b[0m",
            LogoInfo::get().unwrap().color_title,
            s
        )
    }

    fn host_name() -> String {
        let mut buf = [c_char::default(); HOST_NAME_MAX + 1];
        let ret = gethostname(buf.as_mut_ptr(), HOST_NAME_MAX + 1);
        if ret == 0 {
            // SAFETY: Libs are guaranteed to store a valid cstr
            let c_str = unsafe { CStr::from_ptr(buf.as_mut_ptr()) };
            c_str.to_string_lossy().into_owned()
        } else {
            warning!("Failed to get host name (gethostname)");
            String::new()
        }
    }

    fn exe_path() -> SmolPath {
        let Some(path_string) = fs::read_link("/proc/self/exe", PATH_MAX) else {
            warning!("Failed to get exe path");
            return SmolPath::empty();
        };
        SmolPath::from(path_string)
    }
}

#[cfg(test)]
mod tests {
    use crate::detect::title::TitleInfo;

    extern crate std;

    #[test]
    fn cwd_test() {
        let cwd = TitleInfo::cwd();
        println!("{cwd}");
    }

    #[test]
    fn host_test() {
        let cwd = TitleInfo::host_name();
        println!("{cwd}");
    }
}
