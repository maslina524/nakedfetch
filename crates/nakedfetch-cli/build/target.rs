use std::{
    env,
    ffi::{OsStr, OsString},
    sync::OnceLock,
};

static TARGET: OnceLock<Target> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    full_name: Box<OsStr>,
    arch: Box<OsStr>,
    os: Box<OsStr>,
}

impl Target {
    pub fn detect() -> Self {
        fn get_env<K: AsRef<OsStr>>(key: K) -> OsString {
            let key = key.as_ref();
            env::var_os(key)
                .unwrap_or_else(|| panic!("Env variable '{}' is not set!", key.display()))
        }

        let full_name = get_env("TARGET").into_boxed_os_str();
        let arch = get_env("CARGO_CFG_TARGET_ARCH").into_boxed_os_str();
        let os = get_env("CARGO_CFG_TARGET_OS").into_boxed_os_str();

        Self {
            full_name,
            arch,
            os,
        }
    }

    fn get_or_init() -> &'static Self {
        TARGET.get_or_init(Self::detect)
    }

    pub fn full_name() -> &'static OsStr {
        &Self::get_or_init().full_name
    }

    pub fn arch() -> &'static OsStr {
        &Self::get_or_init().arch
    }

    pub fn os() -> &'static OsStr {
        &Self::get_or_init().os
    }
}
