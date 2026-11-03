use alloc::{
    borrow::{Cow, ToOwned},
    collections::BTreeMap,
    string::String,
};

use common::sync::OnceLock;
use json::Value;
use proc::Docs;

use crate::{
    impl_display_for_module,
    modules::{self, Module, ModuleFormat},
};

static BREAK: OnceLock<Break> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Break;

impl Module for Break {
    fn new() -> Self {
        Self {}
    }

    fn get() -> &'static Self {
        BREAK.get_or_init(Self::new)
    }

    fn key(&self) -> &'static str {
        ""
    }

    fn title(&self) -> &'static str {
        "\n"
    }

    fn string_name(&self) -> &'static str {
        "break"
    }
}

impl ModuleFormat for Break {
    fn format(
        &self,
        _key: super::FormatValue<'_>,
        format: super::FormatValue<'_>,
        _map: Option<&BTreeMap<String, Value>>,
    ) -> Option<Cow<'_, str>> {
        let title_str = format.format.unwrap_or_else(|| self.title());
        Some(modules::expand_env_in_module(title_str.to_owned()))
    }
}

impl_display_for_module!(Break);
