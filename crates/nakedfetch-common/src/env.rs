use alloc::string::String;

use path::PathBuf;
use wtf8::{Wtf8, Wtf8Buf};

use crate::imp::env;

/// # Errors
///
/// If the system failed to correctly retrieve the Env variable or it was not
/// found.
pub fn var_os<T: AsRef<Wtf8>>(name: T) -> Result<Wtf8Buf, env::EnvVarError> {
    env::var(name.as_ref())
}

/// # Errors
///
/// If the system failed to correctly retrieve the Env variable or it was not
/// found.
pub fn var<T: AsRef<Wtf8>>(name: T) -> Result<String, env::EnvVarError> {
    let s = env::var(name.as_ref())?.to_string_lossy();
    Ok(s)
}

#[derive(Debug, Clone)]
pub struct Args {
    inner: env::ArgsInner,
}

impl Iterator for Args {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[must_use]
pub fn args() -> Args {
    Args {
        inner: env::ArgsInner::new(),
    }
}

#[must_use]
pub fn cache_dir() -> PathBuf {
    env::cache()
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "windows")]
    pub const ENV_SEP: char = ';';

    #[test]
    fn env_var_test() {
        let var = super::var("PATH").expect("Failed to get env var");
        assert_ne!(var, "");
        assert!(var.contains(ENV_SEP));
    }

    #[test]
    fn args_test() {
        let mut args = super::args();

        let crate_name = args.next().expect("args in empty");
        assert!(crate_name.contains(env!("CARGO_CRATE_NAME")));

        let test_name = args.next().expect("Test name not exists (args[1])");
        let target = "env::tests::args_test";
        assert!(
            test_name.contains(target),
            "Target: {target}, Arg: {test_name}"
        );
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn cache_dir_test() {
        let mut std = std::env::home_dir().expect("std::env::home_dir failed");
        std.push("AppData/Local/nakedfetch/cache");

        let std_string = std.to_string_lossy().replace('\\', "/");
        let my_string = super::cache_dir().to_string_lossy().replace('\\', "/");
        assert_eq!(std_string, my_string);
    }
}
