use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{DataEnum, Ident, Result, Variant};

use crate::utils;

pub fn enums(name: &Ident, e: DataEnum) -> Result<TokenStream> {
    let variants: Vec<Variant> = e.variants.into_iter().collect();

    if let Some(v) = variants.iter().find(|&v| utils::field_len(&v.fields) > 1) {
        bail!(
            v.fields.span(),
            "The FromInput derive macro doesn't support variants with more than 1 field",
        )
    }

    let empty_idents = utils::get_empty_variant_idents(&variants);
    let empty_ident_strs = utils::get_lowercase_ident_strs(&empty_idents);
    let ident_strs_concat = utils::concat_strings_human_readable(&empty_ident_strs);
    let (inner_types, inner_type_ctors) = utils::get_variant_types_and_ctors(&variants)?;

    let gen = quote! {
        impl parkour::FromInputValue for #name {
            type Context = ();

            fn from_input_value(value: &str, _: &Self::Context) -> parkour::Result<Self> {
                match value {
                    #(
                        v if v.eq_ignore_ascii_case(#empty_ident_strs) =>
                            Ok(#name::#empty_idents {}),
                    )*
                    v => {
                        #[allow(unused_mut)]
                        let mut source = None::<parkour::Error>;
                        #(
                            match <#inner_types>::from_input_value(value, &Default::default()) {
                                Ok(__v) => return Ok( #name::#inner_type_ctors ),
                                Err(e) if e.is_no_value() => {},
                                Err(e) => {
                                    source = Some(e);
                                },
                            }
                        )*
                        match source {
                            Some(s) => Err(
                                parkour::Error::unexpected_value(v, #ident_strs_concat)
                                    .with_source(s),
                            ),
                            None => Err(parkour::Error::unexpected_value(v, #ident_strs_concat)),
                        }
                    }
                }
            }
        }
    };
    Ok(gen)
}
