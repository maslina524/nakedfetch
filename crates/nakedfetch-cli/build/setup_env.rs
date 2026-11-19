use crate::target::Target;

pub fn target() {
    set_env!("TARGET={}", Target::full_name().display());
}

pub fn target_os() {
    set_env!("TARGET_OS={}", Target::os().display());
}

pub fn target_arch() {
    set_env!("TARGET_ARCH={}", Target::arch().display());
}

pub fn compile_time() {
    let time = chrono::Local::now().format("%b %d %Y, %H:%M:%S");
    set_env!("COMPILE_TIME={}", time);
}

pub fn rustc_version() -> Result<(), rustc_version::Error> {
    let ver = rustc_version::version()?;
    set_env!("RUSTC_VERSION={}", ver);
    Ok(())
}

pub fn llvm_version() -> Result<(), rustc_version::Error> {
    let Some(ver) = rustc_version::version_meta()?.llvm_version else {
        set_env!("LLVM_VERSION=");
        return Ok(());
    };
    set_env!("LLVM_VERSION={}", ver);
    Ok(())
}
