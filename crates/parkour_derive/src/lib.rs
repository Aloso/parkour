extern crate proc_macro;

use proc_macro::TokenStream;

use syn::spanned::Spanned;
use syn::{Data, DeriveInput};

#[macro_use]
mod utils;
mod attrs;
mod parse_attrs;

mod from_input;
mod from_input_value;

#[proc_macro_derive(FromInputValue)]
pub fn from_input_value_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;

    if generics.type_params().next().is_some() {
        bail_main!(
            generics.span(),
            "The FromInputValue derive macro currently doesn't support generics",
        );
    }

    match ast.data {
        Data::Enum(e) => match from_input_value::enums(name, e) {
            Ok(stream) => stream.into(),
            Err(err) => err.into_compile_error().into(),
        },
        Data::Struct(s) => bail_main!(
            s.struct_token.span(),
            "The FromInputValue derive macro only supports enums, not structs",
        ),
        Data::Union(u) => bail_main!(
            u.union_token.span(),
            "The FromInputValue derive macro only supports enums, not unions",
        ),
    }
}

#[proc_macro_derive(FromInput, attributes(parkour, arg))]
pub fn from_input_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;

    if generics.type_params().next().is_some() {
        bail_main!(
            generics.span(),
            "The FromInput derive macro currently doesn't support generics",
        );
    }

    let result = match ast.data {
        Data::Enum(e) => from_input::enums(name, e, ast.attrs),
        Data::Struct(s) => from_input::structs(name, s, ast.attrs),
        Data::Union(u) => bail_main!(
            u.union_token.span(),
            "The FromInput derive macro only supports enums, not unions",
        ),
    };
    match result {
        Ok(stream) => stream.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
