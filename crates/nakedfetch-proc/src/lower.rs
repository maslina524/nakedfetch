use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

pub fn imp(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let first_token = match iter.next() {
        Some(token) => token,
        None => {
            let out = LitStr::new("", Span::call_site());
            return quote! { #out };
        },
    };
    if let Some(next_token) = iter.next() {
        return syn::Error::new(next_token.span(), "the macro accepts only one token.")
            .to_compile_error();
    }

    let text = if let Ok(lit_str) = syn::parse2::<LitStr>(quote! { #first_token }) {
        lit_str.value()
    } else {
        first_token.to_string()
    };

    let lower = text.to_lowercase();
    let out = LitStr::new(&lower, Span::call_site());
    quote! { #out }
}
