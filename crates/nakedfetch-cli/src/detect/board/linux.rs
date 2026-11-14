use common::{
    fs::{self, FileReadError, FileReadToStringError},
    warning,
};
use path::{Path, PathBuf, SmolPath};

use crate::{modules::Board, str::SmolStr};

const EACCES: i32 = 13;

fn read_file<T: AsRef<Path>>(path: T) -> SmolStr {
    match fs::read_to_string(path.as_ref()) {
        Ok(s) => SmolStr::from(s.trim()),
        Err(e) => {
            // if e.code() != EACCES {
            warning!("Failed to read file: {e}");
            SmolStr::empty()
        },
    }
}

pub fn get() -> Board {
    let root_path = PathBuf::from("/sys/devices/virtual/dmi/id/");

    let name = read_file(root_path.clone().join("board_name"));
    let vendor = read_file(root_path.clone().join("board_vendor"));
    let version = read_file(root_path.clone().join("board_version"));
    let serial = read_file(root_path.join("board_serial"));

    Board {
        name,
        vendor,
        version,
        serial,
    }
}
