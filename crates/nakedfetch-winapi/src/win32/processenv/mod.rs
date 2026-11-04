mod GetEnvironmentVariable;
pub use GetEnvironmentVariable::*;

#[cfg(test)]
mod tests {
    use path::Path;

    #[test]
    fn get_env_test() {
        let wtf8 = super::GetEnvironmentVariable("APPDATA").expect("Failed to get env var");
        let path = Path::from_os_str(&wtf8);
        let appadata = path
            .parent()
            .expect("Windows returns empty path")
            .as_pathbuf();
        let name = appadata.last().expect("Windows returns small path");

        assert_eq!(name.to_str(), Some("AppData"));
    }
}
