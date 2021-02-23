use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, DataEnum, Ident, Result, Variant};

use crate::attrs::{Attr, Parkour};
use crate::{attrs, utils};

pub fn enums(name: &Ident, e: DataEnum, attrs: Vec<Attribute>) -> Result<TokenStream> {
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

    let attrs = attrs::parse(&attrs)?;
    let is_main = attrs.iter().any(|(a, _)| matches!(a, Attr::Parkour(Parkour::Main)));

    let start_bump = if is_main {
        quote! { input.bump_argument().unwrap(); }
    } else {
        quote! {}
    };

    let gen = quote! {
        #[automatically_derived]
        impl parkour::FromInput for #name {
            type Context = ();

            fn from_input<P: parkour::Parse>(input: &mut P, _: &Self::Context)
                    -> parkour::Result<Self> {
                #start_bump
                #(
                    if input.parse_command(#empty_ident_strs) {
                        // TODO: Parse -h, --help and -- by default
                        input.expect_empty()?;
                        return Ok(#name::#empty_idents {});
                    }
                )*

                #(
                    match <#inner_types as parkour::FromInput>::from_input(input, &Default::default()) {
                        Ok(__v) => return Ok( #name::#inner_type_ctors ),
                        Err(e) if e.is_no_value() => {},
                        Err(e) => {
                            return Err(e);
                        },
                    }
                )*
                Err(parkour::Error::no_value())
            }
        }
    };
    Ok(gen)
}
