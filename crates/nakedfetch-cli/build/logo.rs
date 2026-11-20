use std::{
    env,
    error::Error,
    ffi::OsString,
    fmt, fs, io,
    iter::once,
    path::{Path, PathBuf},
};

use zlib_rs::{DeflateConfig, ReturnCode};

#[derive(Debug)]
pub enum CompressError {
    EnvIsNotSet(OsString),
    FileSystem(io::Error),
    Zlib(ReturnCode),
    InvalidPath,
}

impl From<io::Error> for CompressError {
    fn from(e: io::Error) -> Self {
        Self::FileSystem(e)
    }
}

impl fmt::Display for CompressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnvIsNotSet(var) => write!(f, "Env variable {} is not set", var.display()),
            Self::FileSystem(err) => write!(f, "Filesystem error: {err}"),
            Self::Zlib(code) => write!(f, "Zlib compression failed with code: {code:?}"),
            Self::InvalidPath => write!(f, "Failed to extract directory or file name from path"),
        }
    }
}

impl Error for CompressError {}

fn valid_chars() -> impl Iterator<Item = char> {
    ('a'..='z').chain(once('_'))
}

fn find_logo_dir() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .map(|ancestor| ancestor.join("logo"))
        .find(|candidate| candidate.exists())
        .unwrap_or_else(|| manifest_dir.join("logo"))
}

fn compress_bytes(input: &[u8]) -> Result<Vec<u8>, CompressError> {
    let cap = zlib_rs::compress_bound(input.len());
    let mut out = vec![0u8; cap];

    let (compressed, rc) = zlib_rs::compress_slice(&mut out, input, DeflateConfig::default());

    if rc != ReturnCode::Ok {
        return Err(CompressError::Zlib(rc));
    }

    let compressed_len = compressed.len();
    // SAFETY: zlib_rs guarantees that the first `compressed_len` bytes are
    // initialized
    unsafe {
        out.set_len(compressed_len);
    }

    Ok(out)
}

pub fn compress() -> Result<(usize, usize), CompressError> {
    let base_dir = find_logo_dir();
    let out_dir = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .ok_or_else(|| CompressError::EnvIsNotSet(OsString::from("OUT_DIR")))?;

    let temp_root = out_dir.join("temp");
    let mut raw_bytes = 0;
    let mut enc_bytes = 0;

    for ch in valid_chars() {
        let dir_name = ch.to_string();
        let src_dir = base_dir.join(&dir_name);

        let Ok(entries) = fs::read_dir(&src_dir) else {
            continue;
        };

        let dest_dir = temp_root.join(&dir_name);
        let mut dir_created = false;

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let content = fs::read(&path)?;
            raw_bytes += content.len();

            let compressed = compress_bytes(&content)?;
            enc_bytes += compressed.len();

            if !dir_created {
                fs::create_dir_all(&dest_dir)?;
                dir_created = true;
            }

            let file_name = path.file_name().ok_or(CompressError::InvalidPath)?;
            fs::write(dest_dir.join(file_name), &compressed)?;
        }
    }

    Ok((raw_bytes, enc_bytes))
}

#[cfg(test)]
mod tests {
    use super::valid_chars;

    #[test]
    fn valid_chars_iter() {
        let collected: Vec<char> = valid_chars().collect();
        assert_eq!(
            collected,
            [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_'
            ]
        );
    }
}
