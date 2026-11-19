use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .unwrap_or_else(|_| panic!("CARGO_CFG_TARGET_OS is not set"));

    if target_os == "windows" {
        println!("cargo:rustc-cfg=lua5_5");
    } else {
        println!("cargo:rustc-cfg=lua5_4");
    }
}
