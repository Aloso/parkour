extern crate proc_macro;

use std::fmt::Write;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

use syn::spanned::Spanned;
use syn::{Data, Ident, Variant};

#[proc_macro_derive(FromInputValue)]
pub fn from_input_value_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    derive_impl(ast)
}

fn derive_impl(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    if generics.type_params().next().is_some() {
        return compile_error(
            generics.span(),
            "The FromInputValue derive macro currently doesn't support generics",
        );
    }

    match ast.data {
        Data::Struct(s) => compile_error(
            s.struct_token.span(),
            "The FromInputValue derive macro only supports enums, not structs",
        ),
        Data::Union(u) => compile_error(
            u.union_token.span(),
            "The FromInputValue derive macro only supports enums, not unions",
        ),
        Data::Enum(e) => {
            let variants: Vec<Variant> = e.variants.into_iter().collect();
            let len = variants.len();

            for v in &variants {
                if v.fields.iter().next().is_some() {
                    return compile_error(
                        v.fields.span(),
                        "The FromInputValue derive macro doesn't support \
                        variants with fields",
                    );
                }
            }

            let idents_strs: Vec<String> = variants
                .iter()
                .map(|v| {
                    let mut s = format!("{}", v.ident);
                    s.make_ascii_lowercase();
                    s
                })
                .collect();

            let mut ident_strs_concat =
                variants.iter().enumerate().fold(String::new(), |mut acc, (i, v)| {
                    if i == 0 {
                        write!(&mut acc, "{}", v.ident).unwrap();
                    } else if i < len - 1 {
                        write!(&mut acc, ", {}", v.ident).unwrap();
                    } else {
                        write!(&mut acc, " or {}", v.ident).unwrap();
                    }
                    acc
                });
            ident_strs_concat.make_ascii_lowercase();

            let idents: Vec<Ident> = variants.into_iter().map(|v| v.ident).collect();

            let gen = quote! {
                impl parkour::FromInputValue for #name {
                    type Context = ();

                    fn from_input_value(value: &str, _: &Self::Context) -> Result<Self, parkour::Error> {
                        match value {
                            #(
                                v if v.eq_ignore_ascii_case(#idents_strs) => Ok(#name::#idents),
                            )*
                            v => Err(parkour::Error::unexpected_value(v, #ident_strs_concat)),
                        }
                    }
                }
            };
            gen.into()
        }
    }
}

fn compile_error(span: Span, message: impl std::fmt::Display) -> TokenStream {
    syn::Error::new(span, message).to_compile_error().into()
}
