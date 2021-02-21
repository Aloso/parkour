use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{DataEnum, Ident, Result, Variant};

use crate::utils;

pub fn enums(name: &Ident, e: DataEnum) -> Result<TokenStream> {
    let variants: Vec<Variant> = e.variants.into_iter().collect();
    let len = variants.len();

    let mut empty_ident_strs = Vec::new();
    let mut ident_strs_concat = String::new();
    let mut empty_idents = Vec::new();

    let mut inner_types = Vec::new();
    let mut inner_types_string = Vec::new();
    let mut inner_type_ctors = Vec::new();

    for (i, v) in variants.into_iter().enumerate() {
        let field_len = utils::field_len(&v.fields);

        if field_len == 0 {
            let mut s = format!("{}", v.ident);
            s.make_ascii_lowercase();

            if i != 0 {
                if i < len - 1 {
                    ident_strs_concat.push_str(", ");
                } else {
                    ident_strs_concat.push_str(" or ");
                }
            }
            ident_strs_concat.push_str(&s);

            empty_ident_strs.push(s);
            empty_idents.push(v.ident);
        } else if field_len == 1 {
            let var_name = v.ident;
            let field = v.fields.into_iter().next().expect("an enum has no field");
            let field_ty_string = field.ty.to_token_stream().to_string();
            if inner_types_string.contains(&field_ty_string) {
                bail!(
                    field.span(),
                    "The FromInputValue derive macro doesn't support multiple variants \
                     with the same type",
                );
            }
            inner_types_string.push(field_ty_string);
            inner_types.push(field.ty);

            if let Some(ident) = field.ident {
                inner_type_ctors.push(quote! { #var_name { #ident: __v } });
            } else {
                inner_type_ctors.push(quote! { #var_name(__v) });
            }
        } else {
            bail!(
                v.fields.span(),
                "The FromInputValue derive macro doesn't support variants with more \
                 than 1 field",
            );
        }
    }

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
