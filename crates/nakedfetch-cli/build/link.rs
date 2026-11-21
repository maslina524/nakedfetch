use std::{
    env,
    error::Error,
    ffi::OsStr,
    fmt, fs,
    path::{Path, PathBuf},
};

use crate::target::Target;

#[derive(Debug)]
pub struct LibcLinkError {
    os: &'static OsStr,
    arch: &'static OsStr,
}

impl LibcLinkError {
    pub const fn new(os: &'static OsStr, arch: &'static OsStr) -> Self {
        Self { os, arch }
    }
}

impl fmt::Display for LibcLinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Libc is not supported on your platform ({} {})",
            self.os.display(),
            self.arch.display()
        )
    }
}

impl Error for LibcLinkError {}

pub fn libc() -> Result<(), LibcLinkError> {
    let os = Target::os();
    let arch = Target::arch();

    let os_bytes = os.as_encoded_bytes();
    let arch_bytes = arch.as_encoded_bytes();

    match (os_bytes, arch_bytes) {
        (b"linux", b"x86_64") => {
            link_lib!("dylib=dl");
            link_lib!("dylib=m");
            // link_arg!("-pthread");
            // link_arg!("-lc");
        },
        (b"android", b"aarch64") => {
            link_arg!("-Wl,--no-as-needed");
            link_arg!("-lc");
            link_lib!("dylib=m");
            link_lib!("dylib=dl");
        },
        (b"windows", _) => {
            // FIXME: This just suppresses the error rather than solving it,
            // in the future Lua should be built manually for the linker
            // that Rust uses
            link_arg!("/NODEFAULTLIB:LIBCMT");
        },
        _ => return Err(LibcLinkError::new(os, arch)),
    }
    Ok(())
}

#[derive(Debug)]
pub enum LuaLinkError {
    NotADir(PathBuf),
    PathNotFound(PathBuf),
    PlatformNotSupport {
        os: &'static OsStr,
        arch: &'static OsStr,
    },
}

impl LuaLinkError {
    pub const fn not_found(path: PathBuf) -> Self {
        Self::PathNotFound(path)
    }

    pub const fn not_dir(path: PathBuf) -> Self {
        Self::NotADir(path)
    }

    pub const fn not_support(os: &'static OsStr, arch: &'static OsStr) -> Self {
        Self::PlatformNotSupport { os, arch }
    }
}

impl fmt::Display for LuaLinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const NAME: &str = "\x1b[1mLUA_LIB_DIR\x1b[0m";

        fn outro(f: &mut fmt::Formatter<'_>, path: &Path) -> fmt::Result {
            writeln!(
                f,
                "\nSet the env variable {NAME} to the path where the linker needs to find the Lua lib"
            )?;
            write!(f, "\n{NAME}={}", path.display())?;

            write!(f, " {:?}", fs::canonicalize(path))
            // fs::canonicalize(path).map_or(Ok(()), |expanded| write!(f, " {}",
            // expanded.display()))
        }

        match self {
            Self::NotADir(path) => {
                write!(f, "The path passed as {NAME} is not a directory.")?;
                outro(f, path)
            },
            Self::PathNotFound(path) => {
                write!(f, "The path passed as {NAME} does not exists.")?;
                outro(f, path)
            },
            Self::PlatformNotSupport { os, arch } => write!(
                f,
                "Lua is not supported on your platform ({} {})",
                os.display(),
                arch.display()
            ),
        }
    }
}

impl Error for LuaLinkError {}

pub fn lua_lib() -> Result<(), LuaLinkError> {
    fn lib_dir<T: AsRef<OsStr>>(default: T) -> Result<PathBuf, LuaLinkError> {
        let env = env::var_os("LUA_LIB_DIR").unwrap_or_else(|| default.as_ref().to_os_string());
        let path = PathBuf::from(env);

        let path = if path.is_absolute() {
            path
        } else {
            let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
            manifest_dir
                .ancestors()
                .map(|ancestor| ancestor.join(&path))
                .find(|candidate| candidate.exists())
                .unwrap_or_else(|| manifest_dir.join(&path))
        };

        if !path.exists() {
            return Err(LuaLinkError::not_found(path));
        }
        if !path.is_dir() {
            return Err(LuaLinkError::not_dir(path));
        }

        Ok(path)
    }

    let os = Target::os();
    let arch = Target::arch();

    let os_bytes = os.as_encoded_bytes();
    let arch_bytes = arch.as_encoded_bytes();

    match (os_bytes, arch_bytes) {
        (b"linux", b"aarch64") => {
            let dir = lib_dir("/usr/lib/")?;
            link_search!("native={}", dir.display());
            link_lib!("static=lua5.4");
            link_lib!("dylib=dl");
            link_lib!("dylib=m");
        },
        (b"android", b"aarch64") => {
            let dir = lib_dir("bin/android-aarch64")?;
            link_search!("native={}", dir.display());
            link_lib!("static=lua5.4");
            link_arg!("-Wl,--no-as-needed");
            link_arg!("-lc");
            link_lib!("dylib=m");
            link_lib!("dylib=dl");
        },
        (b"windows", _) => {
            let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
            let dir = manifest_dir
                .ancestors()
                .map(|ancestor| ancestor.join("bin\\windows"))
                .find(|candidate| candidate.exists())
                .unwrap_or_else(|| manifest_dir.join("bin\\windows"));

            link_search!("native={}", dir.display());
            link_lib!("static=lua55");
        },
        _ => return Err(LuaLinkError::not_support(os, arch)),
    }

    Ok(())
}
