#![cfg_attr(unstable, feature(proc_macro_diagnostic))]
#![cfg_attr(unstable, feature(proc_macro_value))]

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod derive;
mod lower;
mod parser;
mod snake;
mod todo;

// TODO ATTRIBUTE
#[proc_macro_attribute]
pub fn todo(attr: TokenStream, item: TokenStream) -> TokenStream {
    todo::imp(attr.clone());
    item
}

// LOWER MACRO
#[proc_macro]
pub fn lower(item: TokenStream) -> TokenStream {
    lower::imp(item.into()).into()
}

// SNAKE MACRO
#[proc_macro]
pub fn snake_ident(item: TokenStream) -> TokenStream {
    snake::imp_macro(item.into()).into()
}

// SNAKE ATTRIBUTE
#[proc_macro_attribute]
pub fn snake_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    snake::imp_attribute(item.into()).into()
}

// DOC DERIVE
#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(Docs)]
pub fn docs_derive(input: TokenStream) -> TokenStream {
    derive::doc::imp(input)
}

// MODULE DERIVE
#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(Module)]
pub fn module_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = parser::get_fields(&input.data);
    derive::module::imp(&input.ident, &fields).into()
}
