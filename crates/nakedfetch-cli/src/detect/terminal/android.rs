use core::ffi::CStr;

use common::env;
use path::SmolPath;

use crate::{modules::Terminal, str::SmolStr, warning};

static ENV_NAMES: [&[u8]; 6] = [
    b"TERMUX_VERSION",
    b"TERMUX_APP_PACKAGE_MANAGER",
    b"TERMUX_APP__PACKAGE_NAME",
    b"TERMUX_MAIN_PACKAGE_FORMAT",
    b"TERMUX_APP__VERSION_NAME",
    b"TERMUX_APP__VERSION_CODE",
];

const TERMUX: &[u8] = b"Termux";

fn get_version() -> Option<SmolStr> {
    for env in ENV_NAMES {
        let Ok(val) = env::var(env) else {
            continue;
        };

        match SmolStr::try_from(val) {
            Ok(s) => return Some(s),
            Err(e) => {
                warning!("Failed to parse CStr: {e}");
            },
        }
    }

    None
}

pub fn get() -> Terminal {
    let version = get_version().unwrap_or_default();

    Terminal {
        process_name: SmolPath::from_static(TERMUX),
        exe: SmolPath::empty(),
        exe_name: SmolPath::empty(),
        version,
        pid: 0,
        pretty_name: SmolPath::from_static(TERMUX),
        exe_path: SmolPath::empty(),
        tty: -1,
    }
}
