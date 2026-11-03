use common::sync::OnceLock;
use path::SmolPath;
use proc::{Docs, Module};

use crate::{detect::terminal, impl_display_for_module, modules::Module, str::SmolStr};

static TERMINAL: OnceLock<Terminal> = OnceLock::new();

#[derive(Debug, Default, Docs, Module)]
pub struct Terminal {
    pub process_name: SmolPath,
    pub exe: SmolPath,
    pub exe_name: SmolPath,
    pub version: SmolStr,
    pub pid: u32,
    pub pretty_name: SmolPath,
    pub exe_path: SmolPath,
    pub tty: i32,
}

impl Module for Terminal {
    fn new() -> Self {
        terminal::get()
    }

    fn get() -> &'static Self {
        TERMINAL.get_or_init(Self::new)
    }

    fn key(&self) -> &'static str {
        "Terminal"
    }

    fn title(&self) -> &'static str {
        "{pretty-name} {version}"
    }

    fn string_name(&self) -> &'static str {
        "terminal"
    }
}

impl_display_for_module!(Terminal);
