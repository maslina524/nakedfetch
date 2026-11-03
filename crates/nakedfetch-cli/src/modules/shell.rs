use common::sync::OnceLock;
use path::SmolPath;
use proc::{Docs, Module};

use crate::{detect::shell, impl_display_for_module, modules::Module, str::SmolStr};

static SHELL: OnceLock<Shell> = OnceLock::new();

#[derive(Debug, Default, Docs, Module)]
pub struct Shell {
    pub process_name: SmolPath,
    pub exe: SmolPath,
    pub exe_name: SmolPath,
    pub version: SmolStr,
    pub pid: u32,
    pub pretty_name: SmolPath,
    pub exe_path: SmolPath,
    pub tty: i32,
}

impl Module for Shell {
    fn new() -> Self {
        shell::get()
    }

    fn get() -> &'static Self {
        SHELL.get_or_init(Self::new)
    }

    fn key(&self) -> &'static str {
        "Shell"
    }

    fn title(&self) -> &'static str {
        "{pretty-name} {version}"
    }

    fn string_name(&self) -> &'static str {
        "shell"
    }
}

impl_display_for_module!(Shell);
