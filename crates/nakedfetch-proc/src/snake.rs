use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, LitStr, parse2};

fn camel_to_snake(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len() + 4);

    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                let prev = chars[i - 1];
                let next_is_lower = chars.get(i + 1).is_some_and(|n| n.is_ascii_lowercase());

                if prev.is_ascii_lowercase()
                    || prev.is_ascii_digit()
                    || (prev.is_ascii_uppercase() && next_is_lower)
                {
                    out.push('_');
                }
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }

    out
}

pub fn imp_macro(input: TokenStream) -> TokenStream {
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

    let camel = camel_to_snake(&text);
    let out = Ident::new(&camel, Span::call_site());
    quote! { #out }
}

pub fn imp_attribute(item: proc_macro2::TokenStream) -> TokenStream {
    let mut input: DeriveInput = match parse2(item) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    if let Data::Struct(data) = &mut input.data
        && let Fields::Named(fields) = &mut data.fields
    {
        for field in fields.named.iter_mut() {
            if let Some(ident) = &field.ident {
                let raw = ident.to_string();
                let name = raw.strip_prefix("r#").unwrap_or(&raw);

                let snake = camel_to_snake(name);

                if snake != name {
                    field.ident = Some(Ident::new(&snake, ident.span()));
                }
            }
        }
    }

    quote! { #input }
}

#[cfg(test)]
mod tests {
    use super::camel_to_snake;

    #[test]
    fn cases() {
        assert_eq!(camel_to_snake("userName"), "user_name");
        assert_eq!(camel_to_snake("UserName"), "user_name");
        assert_eq!(camel_to_snake("user_name"), "user_name");
        assert_eq!(camel_to_snake("HTTPHeader"), "http_header");
        assert_eq!(camel_to_snake("XMLHttp"), "xml_http");
        assert_eq!(camel_to_snake("userIDCard"), "user_id_card");
        assert_eq!(camel_to_snake("x"), "x");
        assert_eq!(camel_to_snake(""), "");
    }
}
