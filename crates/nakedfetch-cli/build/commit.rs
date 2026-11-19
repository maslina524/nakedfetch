use std::{
    error::Error,
    fmt, io,
    process::{Command, Output},
    str::{Split, Utf8Error},
};

#[derive(Debug)]
pub enum ReadCommitError {
    CallGit(io::Error),
    NonUtf8Output(Utf8Error),
    FewElems,
}

impl From<io::Error> for ReadCommitError {
    fn from(e: io::Error) -> Self {
        Self::CallGit(e)
    }
}

impl From<Utf8Error> for ReadCommitError {
    fn from(e: Utf8Error) -> Self {
        Self::NonUtf8Output(e)
    }
}

impl fmt::Display for ReadCommitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CallGit(e) => write!(f, "Failed to correctly call git log: {e}"),
            Self::NonUtf8Output(e) => write!(f, "Git returned an invalid UTF-8 in stdout: {e}"),
            Self::FewElems => write!(
                f,
                "Git returned data that does not correspond to the required formatting"
            ),
        }
    }
}

impl Error for ReadCommitError {}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DiffStat {
    pub files: usize,
    pub insertions: usize,
    pub deletions: usize,
}

impl From<&str> for DiffStat {
    fn from(s: &str) -> Self {
        let mut stat = Self::default();
        for part in s.split(',') {
            let part = part.trim();
            let num: usize = part
                .split_whitespace()
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or(0);

            if part.contains("file") {
                stat.files = num;
            } else if part.contains("insertion") {
                stat.insertions = num;
            } else if part.contains("deletion") {
                stat.deletions = num;
            }
        }
        stat
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Commit {
    pub author: Box<str>,
    pub email: Box<str>,
    pub date: Box<str>,
    pub hash: Box<str>,
    pub message: Box<str>,
    pub diff: DiffStat,
}

impl Commit {
    fn call_git_log() -> io::Result<Output> {
        Command::new("git")
            .env("LC_ALL", "C")
            .args([
                "log",
                "--format=%an%n%ae%n%ad%n%H%n%s",
                "--date=format:%b %d %Y, %H:%M:%S",
                "-1",
            ])
            .output()
    }

    fn call_git_log_shortstat() -> io::Result<Output> {
        Command::new("git")
            .env("LC_ALL", "C")
            .args([
                "log",
                "--shortstat",
                "-1",
                "--first-parent",
                "-m",
                "--format=",
            ])
            .output()
    }

    pub fn read_git() -> Result<Self, ReadCommitError> {
        fn next<'a>(iter: &'a mut Split<'_, char>) -> Result<&'a str, ReadCommitError> {
            iter.next().ok_or(ReadCommitError::FewElems)
        }

        let log_vec = Self::call_git_log()?.stdout;
        let log = str::from_utf8(&log_vec)?;
        let mut parts = log.split('\n');

        let author = next(&mut parts)?.into();
        let email = next(&mut parts)?.into();
        let date = next(&mut parts)?.into();
        let hash = next(&mut parts)?.into();
        let message = next(&mut parts)?.into();

        let log_vec = Self::call_git_log_shortstat()?.stdout;
        let log = str::from_utf8(&log_vec)?;

        let diff = DiffStat::from(log);

        Ok(Self {
            author,
            email,
            date,
            hash,
            message,
            diff,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::commit::DiffStat;

    #[test]
    fn parse_shortstat() {
        let s = " 6 files changed, 218 insertions(+), 68 deletions(-)";
        let diff = DiffStat::from(s);
        assert_eq!(
            diff,
            DiffStat {
                files: 6,
                insertions: 218,
                deletions: 68
            }
        );

        let s = " 4 files changed, 45 insertions(+)";
        let diff = DiffStat::from(s);
        assert_eq!(
            diff,
            DiffStat {
                files: 4,
                insertions: 45,
                deletions: 0
            }
        );

        let s = " 1 files changed, 5 deletions(-)";
        let diff = DiffStat::from(s);
        assert_eq!(
            diff,
            DiffStat {
                files: 1,
                insertions: 0,
                deletions: 5
            }
        );
    }
}
