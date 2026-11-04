use alloc::string::String;

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
}
