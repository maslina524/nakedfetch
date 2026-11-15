use common::{
    fs::{self, FileCreateError, FileReadError, FileReadToStringError, FileSizeError},
    warning,
};
use libc::errno::Errno;
use path::{Path, PathBuf, SmolPath};

use crate::{modules::Board, str::SmolStr};

fn read_file<T: AsRef<Path>>(path: T) -> SmolStr {
    match fs::read_to_string(path.as_ref()) {
        Ok(s) => SmolStr::from(s.trim()),
        Err(e) => {
            if !is_access_denied(&e) {
                warning!("Failed to read file: {e}");
            }
            SmolStr::empty()
        },
    }
}

const fn is_access_denied(err: &FileReadToStringError) -> bool {
    match err {
        FileReadToStringError::Create(FileCreateError::AccessDenied)
        | FileReadToStringError::Read(FileReadError::AccessDenied) => true,
        FileReadToStringError::Create(FileCreateError::Unknown(e))
        | FileReadToStringError::Size(FileSizeError::Unknown(e))
        | FileReadToStringError::Read(FileReadError::Unknown(e)) => {
            matches!(*e, Errno::EACCES)
        },
        _ => false,
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
