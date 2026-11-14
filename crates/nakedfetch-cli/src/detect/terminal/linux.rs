use core::convert::Into;

use common::{
    formatf,
    fs::{self as fsc, FileReadError, FileReadToStringError},
    warning,
};
use libc::errno::Errno;
use path::{Path, SmolPath};

use crate::{
    detect::shell::get_shell_pid,
    modules::Terminal,
    str::SmolStr,
    unix::{error, fs, libc::c_pid},
};

static BLACKLIST: [&str; 6] = [
    "python",
    "python3",
    "nakedfetch",
    "fastfetch",
    "neofetch",
    "cargo",
];

#[derive(Default)]
struct Process {
    pid: c_pid,
    name: SmolPath,
    arg: SmolPath,
    exe_path: SmolPath,
    tty: i32,
}

impl Process {
    pub fn new(pid: c_pid, name: &str) -> error::Result<Self> {
        let cmdline =
            fsc::read(formatf!("/proc/{pid}/cmdline")).map_err(|_| error::ErrorCode::new(999))?;

        let arg = if cmdline.len() > 1 {
            let mut null_idx = 0;
            while null_idx < cmdline.len() && cmdline[null_idx] != 0 {
                null_idx += 1;
            }

            let slice = &cmdline[..null_idx];
            SmolPath::from(str::from_utf8(slice).unwrap_or_default())
        } else {
            SmolPath::default()
        };

        let exe = fs::read_link(formatf!("/proc/{pid}/exe"), 1024).unwrap_or_default();
        let exe_path = SmolPath::from(exe);

        let tty_path = fs::read_link(formatf!("/proc/{pid}/fd/0"), 512).unwrap_or_default();
        let tty = SmolPath::from(tty_path)
            .last()
            .and_then(|s| s.to_str())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(-1);

        Ok(Self {
            pid,
            name: SmolPath::from(name),
            arg,
            exe_path,
            tty,
        })
    }
}

fn get_terminal_process(shell_pid: u32) -> Result<Process, FileReadToStringError> {
    let state = fsc::read_to_string(formatf!("/proc/{shell_pid}/stat"))?;
    let item_iter = state.split(' ');

    let mut item_iter = item_iter.skip(3); // pid comm state
    let terminal_pid = item_iter
        .next()
        .and_then(|s| s.parse::<i32>().ok())
        .expect("Strange unix /proc/pid/state");

    // Terminal process
    let state = fsc::read_to_string(formatf!("/proc/{terminal_pid}/stat"))?;
    let mut item_iter = state.split(' ').skip(1); // pid
    let raw = item_iter.next().expect("Strange unix /proc/pid/stat");
    let name = if raw.starts_with('(') && raw.ends_with(')') {
        // Valid unix format
        &raw[1..raw.len() - 1]
    } else {
        raw
    };

    Process::new(terminal_pid, name)
        .map_err(|e| FileReadToStringError::Read(FileReadError::Unknown(Errno::EACCES)))
}

pub fn get() -> Terminal {
    let info = match get_terminal_process(get_shell_pid()) {
        Ok(p) => p,
        Err(e) => {
            warning!("Failed to get terminal process: {e}");
            return Terminal::default();
        },
    };

    let exe_name = info.arg.last().map(SmolPath::from).unwrap_or_default();
    let pretty_name = info.name.clone();

    Terminal {
        process_name: info.name,
        exe: info.arg,
        exe_name,
        version: SmolStr::default(),
        pid: info.pid as u32,
        pretty_name,
        exe_path: info.exe_path,
        tty: info.tty,
    }
}
