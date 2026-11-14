use alloc::borrow::{Cow, ToOwned};

use common::{fs as fsc, warning};
use path::SmolPath;

use crate::{detect::initsystem::InitSystemInfo, str::SmolStr, unix::fs};

impl InitSystemInfo {
    #[proc::todo("Too time-consuming to implement for every distribution and system")]
    pub fn new() -> Self {
        let name = fsc::read_to_string("/proc/1/comm")
            .unwrap_or_else(|_| "Unknown".to_owned())
            .trim()
            .to_owned();

        let exe = fsc::read_to_string("/proc/1/cmdline").map_or_else(
            |_| {
                warning!("Failed to read /proc/1/cmdline");
                SmolPath::empty()
            },
            |cmdline| {
                let first_arg = cmdline.split('\0').next().unwrap_or("");
                if first_arg.is_empty() {
                    SmolPath::empty()
                } else {
                    fs::read_link(first_arg, 64).map_or(SmolPath::empty(), SmolPath::from)
                }
            },
        );

        let version = SmolStr::from_static("0.0.0.0");

        Self {
            exe,
            name: Cow::Owned(name),
            version,
        }
    }
}
