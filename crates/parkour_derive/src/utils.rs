use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{Field, Fields, Ident, Result, Type, Variant};

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

pub fn concat_strings_human_readable(idents: &[String]) -> String {
    let mut result = String::new();
    let len = idents.len();
    for (i, s) in idents.iter().enumerate() {
        if i != 0 {
            if i < len - 1 {
                result.push_str(", ");
            } else {
                result.push_str(" or ");
            }
        }
        result.push_str(&s);
    }
    result
}

pub fn get_empty_variant_idents(variants: &[Variant]) -> Vec<&Ident> {
    variants.iter().filter(|&v| field_len(&v.fields) == 0).map(|v| &v.ident).collect()
}

pub fn get_lowercase_ident_strs(idents: &[&Ident]) -> Vec<String> {
    idents
        .iter()
        .map(|&i| {
            let mut s = format!("{}", i);
            s.make_ascii_lowercase();
            s
        })
        .collect()
}

pub fn get_field(variant: &Variant) -> Option<&Field> {
    match &variant.fields {
        Fields::Named(f) => f.named.first(),
        Fields::Unnamed(f) => f.unnamed.first(),
        Fields::Unit => None,
    }
}

pub fn get_variant_types_and_ctors(
    variants: &[Variant],
) -> Result<(Vec<&Type>, Vec<TokenStream>)> {
    let mut inner_types = Vec::new();
    let mut inner_types_string = Vec::new();
    let mut inner_type_ctors = Vec::new();

    for v in variants {
        if let Some(field) = get_field(v) {
            let var_name = &v.ident;
            let field_ty_string = field.ty.to_token_stream().to_string();
            if inner_types_string.contains(&field_ty_string) {
                bail!(
                    field.span(),
                    "The FromInputValue derive macro doesn't support multiple variants \
                     with the same type",
                );
            }
            inner_types_string.push(field_ty_string);
            inner_types.push(&field.ty);

            if let Some(ident) = &field.ident {
                inner_type_ctors.push(quote! { #var_name { #ident: __v } });
            } else {
                inner_type_ctors.push(quote! { #var_name(__v) });
            }
        }
    }

    Ok((inner_types, inner_type_ctors))
}
