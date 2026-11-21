use crate::{commit::Commit, target::Target};

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

pub fn libc_version() {
    set_env!("LIBC_VERSION=");
}

pub fn commit(commit: &Commit) {
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");
    println!("cargo:rerun-if-env-changed=GITHUB_REF_NAME");
    println!("cargo:rerun-if-env-changed=GITHUB_HEAD_REF");

    // Commit
    set_env!("COMMIT_AUTHOR={}", commit.author);
    set_env!("COMMIT_EMAIL={}", commit.email);
    set_env!("COMMIT_DATE={}", commit.date);
    set_env!("COMMIT_SHA={}", commit.hash);
    set_env!("COMMIT_SHA_SMALL={}", &commit.hash[..7]);
    set_env!("COMMIT_MESSAGE={}", commit.message);

    if let Some(index) = commit.date.find(',') {
        let smol = &commit.date[..index];
        set_env!("COMMIT_DATE_SMALL={}", smol);
    } else {
        set_env!("COMMIT_DATE_SMALL=");
    }

    // Diff
    set_env!("COMMIT_FILES={}", commit.diff.files);
    set_env!("COMMIT_ADDED={}", commit.diff.insertions);
    set_env!("COMMIT_DELETED={}", commit.diff.deletions);
    set_env!(
        "COMMIT_TOTAL={}",
        commit.diff.insertions + commit.diff.deletions
    );
}
