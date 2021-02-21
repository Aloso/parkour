use proc_macro2::Span;
use syn::{Fields, Ident, Result};

macro_rules! bail_main {
    ($span:expr, $s:literal $(,)?) => {{
        return syn::Error::new($span, $s).into_compile_error().into();
    }};
    ($span:expr, $($rest:tt)*) => {{
        return syn::Error::new($span, format!($($rest)*))
            .into_compile_error()
            .into();
    }};
}

macro_rules! bail {
    ($span:expr, $s:literal $(,)?) => {{
        return Err(syn::Error::new($span, $s));
    }};
    ($span:expr, $($rest:tt)*) => {{
        return Err(syn::Error::new($span, format!($($rest)*)));
    }};
}

pub fn field_len(fields: &Fields) -> usize {
    match fields {
        Fields::Named(n) => n.named.len(),
        Fields::Unnamed(n) => n.unnamed.len(),
        Fields::Unit => 0,
    }
}

pub fn first_char(span: Span, s: &str) -> Result<&str> {
    match s.char_indices().nth(1) {
        Some((i, _)) => Ok(&s[0..i]),
        None if !s.is_empty() => Ok(s),
        None => bail!(span, "flag is empty"),
    }
}

pub fn ident_to_flag_string(ident: &Ident) -> String {
    ident.to_string().trim_matches('_').replace('_', "-")
}
