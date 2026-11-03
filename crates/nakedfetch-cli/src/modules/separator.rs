use alloc::{borrow::Cow, collections::BTreeMap, string::String};

use common::sync::OnceLock;
use json::Value;
use proc::Docs;

use crate::{
    formats, impl_display_for_module,
    modules::{self, FormatValue, Module, ModuleFormat, Title},
};

static SEPARATOR: OnceLock<Separator> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Separator;

impl Module for Separator {
    fn new() -> Self {
        Self {}
    }

    fn get() -> &'static Self {
        SEPARATOR.get_or_init(Self::new)
    }

    fn key(&self) -> &'static str {
        ""
    }

    fn title(&self) -> &'static str {
        ""
    }

    fn string_name(&self) -> &'static str {
        "separator"
    }
}

impl ModuleFormat for Separator {
    fn format(
        &self,
        _key: FormatValue<'_>,
        _format: FormatValue<'_>,
        _map: Option<&BTreeMap<String, Value>>,
    ) -> Option<Cow<'_, str>> {
        let string = Title::get()
            .format(FormatValue::default(), FormatValue::default(), None)
            .map(|title| "-".repeat(formats::visible_len(&title)));

        string.map(modules::expand_env_in_module)
    }
}

impl_display_for_module!(Separator);
