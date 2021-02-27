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
    let (inner_types, inner_type_ctors) = utils::get_variant_types_and_ctors(&variants)?;

    let empty_ident_comparisons = empty_ident_strs.iter().map(|s| {
        if s.chars().all(|c| c.is_ascii()) {
            quote! { v if v.eq_ignore_ascii_case(#s) }
        } else {
            quote! { v if v.to_lowercase() == #s }
        }
    });

    let from_input_value = quote! {
        fn from_input_value(value: &str, context: &Self::Context) -> parkour::Result<Self> {
            match value {
                #(
                    #empty_ident_comparisons => Ok(#name::#empty_idents {}),
                )*
                v => {
                    #[allow(unused_mut)]
                    let mut source = None::<parkour::Error>;
                    #(
                        match <#inner_types as parkour::FromInputValue>::from_input_value(
                            value,
                            &Default::default()
                        ) {
                            Ok(__v) => return Ok( #name::#inner_type_ctors ),
                            Err(e) if e.is_no_value() => {},
                            Err(e) => {
                                source = Some(e);
                            },
                        }
                    )*
                    match source {
                        Some(s) => Err(
                            parkour::Error::unexpected_value(v, Self::possible_values(context))
                                .with_source(s),
                        ),
                        None => Err(parkour::Error::unexpected_value(v, Self::possible_values(context))),
                    }
                }
            }
        }
    };

    let possible_values = quote! {
        #[allow(unused_mut)]
        fn possible_values(context: &Self::Context) -> Option<parkour::help::PossibleValues> {
            let mut values = vec![
                #(
                    parkour::help::PossibleValues::String(#empty_ident_strs.to_string())
                ),*
            ];
            #(
                if let Some(v) = <#inner_types as parkour::FromInputValue>::possible_values(context) {
                    values.push(v);
                }
            ),*
            Some(parkour::help::PossibleValues::OneOf(values))
        }
    };

    let gen = quote! {
        #[automatically_derived]
        impl parkour::FromInputValue<'static> for #name {
            type Context = ();

            #from_input_value

            #possible_values
        }
    };
    Ok(gen)
}
