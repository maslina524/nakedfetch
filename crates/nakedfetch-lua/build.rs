use std::{env, ffi::OsString};

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .unwrap_or_else(|_| panic!("CARGO_CFG_TARGET_OS is not set"));
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or_else(|_| panic!("CARGO_CFG_TARGET_ARCH is not set"));

    match (target_os.as_str(), target_arch.as_str()) {
        ("linux", "x86_64") => {
            println!(
                "cargo:rustc-link-search=native={}",
                env::var_os("LUA_LIB_DIR")
                    .unwrap_or_else(|| OsString::from("/usr/lib/"))
                    .display()
            );
            println!("cargo:rustc-link-lib=static=lua5.4");
            println!("cargo:rustc-link-lib=dylib=dl");
            println!("cargo:rustc-link-lib=dylib=m");
            // println!("cargo:rustc-link-arg=-pthread");
            // println!("cargo:rustc-link-arg=-lc");
        },
        ("android", "aarch64") => {
            let lua_dir =
                env::var("LUA_LIB_DIR").unwrap_or_else(|_| "bin/android-aarch64".to_owned());
            println!("cargo:rustc-link-search=native={lua_dir}");
            println!("cargo:rustc-link-lib=static=lua5.4");
            println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
            println!("cargo:rustc-link-arg=-lc");
            println!("cargo:rustc-link-lib=dylib=m");
            println!("cargo:rustc-link-lib=dylib=dl");
        },
        ("windows", _) => {
            println!("cargo:rustc-link-search=native=bin/windows");
            println!("cargo:rustc-link-lib=static=lua55");

            // FIXME: This just suppresses the error rather than solving it,
            // in the future Lua should be built manually for the linker
            // that Rust uses
            println!("cargo:rustc-link-arg=/NODEFAULTLIB:LIBCMT");
        },
        _ => {},
    }

    if target_os == "windows" {
        println!("cargo:rustc-cfg=lua5_5");
    } else {
        println!("cargo:rustc-cfg=lua5_4");
    }
}
