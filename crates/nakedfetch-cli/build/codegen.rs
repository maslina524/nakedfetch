use core::fmt;
use std::{
    collections::HashMap,
    env,
    error::Error,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::Ident;

#[derive(Debug)]
pub enum CodegenError {
    FileSystem(io::Error),
    Serde(serde_json::Error),
    Syn(syn::Error),
}

impl From<io::Error> for CodegenError {
    fn from(e: io::Error) -> Self {
        Self::FileSystem(e)
    }
}

impl From<serde_json::Error> for CodegenError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}

impl From<syn::Error> for CodegenError {
    fn from(e: syn::Error) -> Self {
        Self::Syn(e)
    }
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileSystem(err) => write!(f, "Filesystem error: {err}"),
            Self::Serde(err) => write!(f, "Serde Json error: {err}"),
            Self::Syn(err) => write!(f, "Syn parser error: {err}"),
        }
    }
}

impl Error for CodegenError {}

#[derive(Debug)]
pub enum GenLogoError {
    EnvIsNotSet(OsString),
    FileSystem(io::Error),
    Codegen(CodegenError),
}

impl From<io::Error> for GenLogoError {
    fn from(e: io::Error) -> Self {
        Self::FileSystem(e)
    }
}

impl From<CodegenError> for GenLogoError {
    fn from(e: CodegenError) -> Self {
        Self::Codegen(e)
    }
}

impl fmt::Display for GenLogoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnvIsNotSet(var) => write!(f, "Env variable {} is not set", var.display()),
            Self::FileSystem(err) => write!(f, "Filesystem error: {err}"),
            Self::Codegen(err) => write!(f, "Codegen: {err}"),
        }
    }
}

impl Error for GenLogoError {}

#[derive(Deserialize)]
struct LogoEntry {
    names: Vec<Box<str>>,
    lines: Box<str>,
    colors: Vec<Box<str>>,
    color_keys: Box<str>,
    color_title: Box<str>,
}

fn find_logo_path() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .map(|ancestor| ancestor.join("logo.json"))
        .find(|candidate| candidate.exists())
        .unwrap_or_else(|| manifest_dir.join("logo.json"))
}

pub fn generate_logo() -> Result<(), GenLogoError> {
    let logo_path = find_logo_path();
    println!("cargo:rerun-if-changed={}", logo_path.display());

    let env = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .ok_or_else(|| GenLogoError::EnvIsNotSet(OsString::from("OUT_DIR")))?;

    let out_dir = env.join("logo");
    fs::create_dir_all(&out_dir)?;
    generate(&logo_path, &out_dir)?;
    Ok(())
}

fn generate(input_json: &Path, out_dir: &Path) -> Result<(), CodegenError> {
    let content = fs::read_to_string(input_json)?;
    let mut map: HashMap<Box<str>, Vec<LogoEntry>> = serde_json::from_str(&content)?;

    for v in map.values_mut() {
        for entry in v {
            let Some(first) = entry.colors.first() else {
                break;
            };

            if entry.color_keys.as_ref() == "color::FG_DEFAULT" {
                entry.color_keys = first.clone();
            }

            if entry.color_title.as_ref() == "color::FG_DEFAULT" {
                entry.color_title = first.clone();
            }
        }
    }

    for (letter, entries) in &map {
        let code = generate_module(letter, entries)?;
        fs::write(out_dir.join(format!("{letter}.rs")), code.to_string())?;
    }
    Ok(())
}

fn generate_module(letter: &str, entries: &[LogoEntry]) -> Result<TokenStream, CodegenError> {
    let static_name = Ident::new(&letter.to_ascii_uppercase(), Span::call_site());
    let logo_infos = entries
        .iter()
        .map(generate_logo_info)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(quote! {
        pub static #static_name: &[crate::logo::LogoInfo] = &[
            #(#logo_infos),*
        ];
    })
}

fn generate_logo_info(entry: &LogoEntry) -> Result<TokenStream, CodegenError> {
    let names: Vec<_> = entry.names.iter().collect();
    let path_lit = &entry.lines;

    let colors: Vec<syn::Path> = entry
        .colors
        .iter()
        .map(|c| syn::parse_str::<syn::Path>(c))
        .collect::<Result<Vec<_>, _>>()?;

    let color_keys: syn::Path = syn::parse_str(&entry.color_keys)?;
    let color_title: syn::Path = syn::parse_str(&entry.color_title)?;

    let q = quote! {
        crate::logo::LogoInfo {
            names: &[#(#names),*],
            encoded: include_bytes!(concat!(env!("OUT_DIR"), #path_lit)),
            colors: &[
                #(#colors),*
            ],
            color_keys: #color_keys,
            color_title: #color_title,
        }
    };
    Ok(q)
}
