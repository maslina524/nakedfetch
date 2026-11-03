use core::{
    ffi::{CStr, c_void},
    sync::atomic::Ordering::Relaxed,
};

use path::SmolPath;

use crate::{
    detect::shell::SHELL_PID,
    modules::Shell,
    str::SmolStr,
    warning,
    windows::{
        encoding::{Utf16Len, utf16le_to_utf8},
        env,
        error::ErrorCode,
        link::{OpenProcess, QueryFullProcessImageNameW},
    },
};

const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
const INVALID_HANDLE: *mut c_void = (-1isize).cast_unsigned() as *mut c_void;

static BLACKLIST: [&CStr; 6] = [
    c"python.exe",
    c"python3.exe",
    c"nakedfetch.exe",
    c"fastfetch.exe",
    c"neofetch.exe",
    c"cargo.exe",
];

#[derive(Default)]
struct Process {
    pid: u32,
    exe_path: SmolPath,
}

impl Process {
    pub fn new(pid: u32) -> Self {
        if pid == 0 {
            return Self::default();
        }

        // SAFETY: Completely safe
        let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };

        if handle == INVALID_HANDLE {
            warning!("Failed to get shell process handle: {}", ErrorCode::last());
            return Self::default();
        }
        let mut size = 260;
        let mut buf = [0u16; 260];
        // SAFETY: Completely safe
        let ret = unsafe { QueryFullProcessImageNameW(handle, 0, buf.as_mut_ptr(), &raw mut size) };

        let exe_path = if ret == 1 {
            // SAFETY: WinAPI returns a valid C string and always leaves 261
            // bytes zeroed
            match utf16le_to_utf8(&buf, Utf16Len::Len(size as usize)) {
                Ok(s) => SmolPath::from(s),
                Err(e) => {
                    warning!("Failed to convert Utf16 to Utf8 in shell exe path: {e}");
                    SmolPath::default()
                },
            }
        } else {
            warning!("Failed to get shell exe path: {}", ErrorCode::last());
            SmolPath::default()
        };

        Self { pid, exe_path }
    }
}

pub fn get_shell_pid() -> u32 {
    let loaded = SHELL_PID.load(Relaxed);
    if loaded != 0 {
        return loaded;
    }

    let mut pid = env::get_my_pid();
    loop {
        pid = if let Some(p) = env::get_ppid_by_pid(pid) {
            p
        } else {
            warning!("Process ppid not foind (pid: {pid})");
            return 0;
        };
        let name = env::get_name_by_pid(pid).expect("Unreachable");

        if !BLACKLIST.contains(&name) {
            break;
        }
    }

    SHELL_PID.store(pid, Relaxed);
    pid
}

pub fn get() -> Shell {
    let info = Process::new(get_shell_pid());

    let exe_path = info.exe_path.clone();
    let process_name = info.exe_path.last().map_or_default(SmolPath::from);

    let version = match env::get_file_product_version(&exe_path) {
        Ok(v) => v,
        Err(e) => {
            warning!("Failed to get file version: {e} (shell)");
            SmolStr::from_static("0.0.0.0")
        },
    };

    let pid = info.pid;
    let exe = process_name.clone();
    let exe_name = process_name.clone();

    let pretty_name_path = exe_name.trim_extension().unwrap_or(&exe_name);
    let pretty_name = SmolPath::from(pretty_name_path);

    Shell {
        process_name,
        exe,
        exe_name,
        version,
        pid,
        pretty_name,
        exe_path,
        tty: -1,
    }
}
