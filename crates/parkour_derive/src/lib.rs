extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};

use syn::spanned::Spanned;
use syn::{Attribute, Data, DataEnum, DeriveInput, Error, Fields, Result, Variant};

use attrs::{Arg, Attr, Parkour};

mod attrs;

#[proc_macro_derive(FromInputValue)]
pub fn from_input_value_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = match syn::parse(input) {
        Ok(ast) => ast,
        Err(e) => return e.into_compile_error().into(),
    };
    let name = &ast.ident;
    let generics = &ast.generics;

    if generics.type_params().next().is_some() {
        return Error::new(
            generics.span(),
            "The FromInputValue derive macro currently doesn't support generics",
        )
        .into_compile_error()
        .into();
    }

    let result = match ast.data {
        Data::Enum(e) => enum_from_input_value(name, e),
        Data::Struct(s) => Err(Error::new(
            s.struct_token.span(),
            "The FromInputValue derive macro only supports enums, not structs",
        )),
        Data::Union(u) => Err(Error::new(
            u.union_token.span(),
            "The FromInputValue derive macro only supports enums, not unions",
        )),
    };
    match result {
        Ok(s) => s,
        Err(e) => e.into_compile_error().into(),
    }
}

fn enum_from_input_value(name: &Ident, e: DataEnum) -> Result<TokenStream> {
    let variants: Vec<Variant> = e.variants.into_iter().collect();
    let len = variants.len();

    let mut empty_ident_strs = Vec::new();
    let mut ident_strs_concat = String::new();
    let mut empty_idents = Vec::new();

    let mut inner_types = Vec::new();
    let mut inner_types_string = Vec::new();
    let mut inner_type_ctors = Vec::new();

    for (i, v) in variants.into_iter().enumerate() {
        let field_len = field_len(&v.fields);

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
                return Err(Error::new(
                    field.span(),
                    "The FromInputValue derive macro doesn't support multiple variants \
                     with the same type",
                ));
            }
            inner_types_string.push(field_ty_string);
            inner_types.push(field.ty);

            if let Some(ident) = field.ident {
                inner_type_ctors.push(quote! { #var_name { #ident: __v } });
            } else {
                inner_type_ctors.push(quote! { #var_name(__v) });
            }
        } else {
            return Err(Error::new(
                v.fields.span(),
                "The FromInputValue derive macro doesn't support variants with more \
                 than 1 field",
            ));
        }
    }

    let gen = quote! {
        impl parkour::FromInputValue for #name {
            type Context = ();

            fn from_input_value(value: &str, _: &Self::Context) -> Result<Self, parkour::Error> {
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
    Ok(gen.into())
}

#[proc_macro_derive(FromInput, attributes(parkour, arg))]
pub fn from_input_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = match syn::parse(input) {
        Ok(ast) => ast,
        Err(e) => return e.into_compile_error().into(),
    };
    let name = &ast.ident;
    let generics = &ast.generics;

    if generics.type_params().next().is_some() {
        return Error::new(
            generics.span(),
            "The FromInput derive macro currently doesn't support generics",
        )
        .into_compile_error()
        .into();
    }

    let result = match ast.data {
        Data::Enum(e) => enum_from_input(name, e, ast.attrs),
        Data::Struct(s) => struct_from_input(name, s, ast.attrs),
        Data::Union(u) => Err(Error::new(
            u.union_token.span(),
            "The FromInput derive macro only supports enums, not unions",
        )),
    };
    match result {
        Ok(s) => s,
        Err(e) => e.into_compile_error().into(),
    }
}

fn struct_from_input(
    name: &Ident,
    s: syn::DataStruct,
    attr: Vec<Attribute>,
) -> Result<TokenStream> {
    let attrs = attrs::parse(attr)?;

    let is_main = attrs.iter().any(|(a, _)| matches!(a, Attr::Parkour(Parkour::Main)));
    let mut subcommands: Vec<String> = attrs
        .iter()
        .filter_map(|(a, _)| match a {
            Attr::Parkour(Parkour::Subcommand(s)) => {
                Some(s.clone().unwrap_or_else(|| name.to_string().to_lowercase()))
            }
            _ => None,
        })
        .collect();
    subcommands.sort_unstable();
    if let Some(w) = subcommands.windows(2).find(|pair| pair[0] == pair[1]) {
        return Err(Error::new(
            Span::call_site(),
            format!("subcommand {:?} is specified twice", w[0]),
        ));
    }

    if is_main && !subcommands.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "`parkour(main)` and `parkour(subcommand)` can't be combined",
        ));
    }
    if !is_main && subcommands.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "The FromInput derive macro requires a `parkour(main)` or \
             `parkour(subcommand)` attribute",
        ));
    }

    let main_condition = if is_main {
        quote! { input.bump_argument().is_some() }
    } else {
        quote! { #( input.parse_command(#subcommands) )||* }
    };

    let field_len = field_len(&s.fields);

    if field_len > 0 {
        if let Fields::Unnamed(_) = s.fields {
            return Err(Error::new(
                Span::call_site(),
                "The FromInput derive macro doesn't support tuple structs",
            ));
        }
    }

    let mut field_idents = Vec::new();
    let mut field_strs = Vec::new();
    let mut contexts = Vec::new();

    for field in s.fields {
        let attrs = attrs::parse(field.attrs)?;
        let ident = field.ident.expect("a field has no ident");

        let mut field_str = None;

        let mut args = Vec::new();
        for (attr, span) in attrs {
            if let Attr::Arg(a) = attr {
                args.push(match a {
                    Arg::Named { mut long, mut short } => {
                        if long.is_empty() && short.is_empty() {
                            return Err(Error::new(span, "no flags specified"));
                        }

                        let ident_str = ident_to_flag_string(&ident);

                        if field_str.is_none() {
                            if !long.is_empty() {
                                let long = long[0].as_deref().unwrap_or(&ident_str);
                                field_str = Some(format!("--{}", long));
                            } else {
                                let short = short[0].as_deref().unwrap_or(&ident_str);
                                field_str = Some(format!("-{}", short));
                            }
                        }

                        long.sort_unstable();
                        short.sort_unstable();
                        if let Some(w) = long.windows(2).find(|pair| pair[0] == pair[1]) {
                            return Err(Error::new(
                                span,
                                format!(
                                    "long flag {:?} is specified twice",
                                    w[0].as_deref().unwrap_or(&ident_str)
                                ),
                            ));
                        }
                        if let Some(w) = short.windows(2).find(|pair| pair[0] == pair[1])
                        {
                            return Err(Error::new(
                                span,
                                format!(
                                    "short flag {:?} is specified twice",
                                    w[0].as_deref()
                                        .unwrap_or(first_char(span, &ident_str)?)
                                ),
                            ));
                        }

                        match (long.len(), short.len()) {
                            (1, 1) => {
                                let long = long[0].as_deref().unwrap_or(&ident_str);
                                let short = short[0].as_deref().map_or_else(
                                    || first_char(ident.span(), &long),
                                    Ok,
                                )?;
                                quote! { Flag::LongShort(#long, #short).into() }
                            }
                            (0, 1) => {
                                let short = short[0].as_deref().map_or_else(
                                    || first_char(ident.span(), &ident_str),
                                    Ok,
                                )?;
                                quote! { Flag::Short(#short).into() }
                            }
                            (1, 0) => {
                                let long = long[0].as_deref().unwrap_or(&ident_str);
                                quote! { Flag::Long(#long).into() }
                            }
                            (_, _) => {
                                let long: Vec<&str> = long
                                    .iter()
                                    .map(|l| l.as_deref().unwrap_or(&ident_str))
                                    .collect();
                                let short =
                                    short.iter().map(|l| l.as_deref().unwrap_or(long[0]));

                                quote! {
                                    Flag::Many(vec![
                                        #( Flag::Long(#long), )*
                                        #( Flag::Short(#short), )*
                                    ]).into()
                                }
                            }
                        }
                    }

                    Arg::Positional { name: None } => {
                        if field_str.is_none() {
                            field_str = Some(ident.to_string());
                        }

                        quote! { todo!() }
                    }
                    Arg::Positional { name: Some(_p) } => {
                        if field_str.is_none() {
                            field_str = Some(ident.to_string());
                        }

                        quote! { todo!() }
                    }
                })
            } else if let Attr::Parkour(_) = attr {
                return Err(Error::new(span, "this key is not yet implemented!"));
            }
        }

        if args.is_empty() {
            return Err(Error::new(
                ident.span(),
                "This field is missing a `arg` attribute",
            ));
        }
        contexts.push(args);

        field_strs.push(field_str.expect("a field has no string"));
        field_idents.push(ident);
    }

    let gen = quote! {
        impl parkour::FromInput for #name {
            type Context = ();

            fn from_input<P: parkour::Parse>(input: &mut P, _: &Self::Context)
                -> Result<Self, parkour::Error> {
                if #main_condition {
                    #(
                        let mut #field_idents = None;
                    )*
                    while input.is_not_empty() {
                        #(
                            #(
                                if SetOnce(&mut #field_idents).apply(input, &#contexts)? {
                                    continue;
                                }
                            )*
                        )*

                        input.expect_empty()?;
                    }
                    Ok(#name {
                        #(
                            #field_idents: #field_idents.ok_or_else(|| parkour::Error::missing_argument(#field_strs))?,
                        )*
                    })
                } else {
                    Err(parkour::Error::no_value())
                }
            }
        }
    };
    Ok(gen.into())
}

fn enum_from_input(
    name: &Ident,
    e: DataEnum,
    attrs: Vec<Attribute>,
) -> Result<TokenStream> {
    let variants: Vec<Variant> = e.variants.into_iter().collect();
    let len = variants.len();

    let mut empty_ident_strs = Vec::new();
    let mut ident_strs_concat = String::new();
    let mut empty_idents = Vec::new();

    let mut inner_types = Vec::new();
    let mut inner_types_string = Vec::new();
    let mut inner_type_ctors = Vec::new();

    let attrs = attrs::parse(attrs)?;
    let is_main = attrs.iter().any(|(a, _)| matches!(a, Attr::Parkour(Parkour::Main)));

    for (i, v) in variants.into_iter().enumerate() {
        let field_len = field_len(&v.fields);

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
            let field = v.fields.into_iter().next().expect("a variant has no field");
            let field_ty_string = field.ty.to_token_stream().to_string();
            if inner_types_string.contains(&field_ty_string) {
                return Err(Error::new(
                    field.span(),
                    "The FromInput derive macro doesn't support multiple variants with \
                     the same type",
                ));
            }
            inner_types_string.push(field_ty_string);
            inner_types.push(field.ty);

            if let Some(ident) = field.ident {
                inner_type_ctors.push(quote! { #var_name { #ident: __v } });
            } else {
                inner_type_ctors.push(quote! { #var_name(__v) });
            }
        } else {
            return Err(Error::new(
                v.fields.span(),
                "The FromInput derive macro doesn't support variants with more than 1 \
                 field",
            ));
        }
    }

    let start_bump = if is_main {
        quote! { input.bump_argument().unwrap(); }
    } else {
        quote! {}
    };

    let gen = quote! {
        impl parkour::FromInput for #name {
            type Context = ();

            fn from_input<P: parkour::Parse>(input: &mut P, _: &Self::Context)
                -> Result<Self, parkour::Error> {
                #start_bump
                #(
                    if input.parse_command(#empty_ident_strs) {
                        // TODO: Parse -h, --help and -- by default
                        input.expect_empty()?;
                        return Ok(#name::#empty_idents {});
                    }
                )*

                #(
                    match <#inner_types>::from_input(input, &Default::default()) {
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
    Ok(gen.into())
}

fn field_len(fields: &Fields) -> usize {
    match fields {
        Fields::Named(n) => n.named.len(),
        Fields::Unnamed(n) => n.unnamed.len(),
        Fields::Unit => 0,
    }
}

fn first_char(span: Span, s: &str) -> Result<&str> {
    match s.char_indices().nth(1) {
        Some((i, _)) => Ok(&s[0..i]),
        None if !s.is_empty() => Ok(s),
        None => Err(Error::new(span, "flag is empty")),
    }
}

fn ident_to_flag_string(ident: &Ident) -> String {
    ident.to_string().trim_matches('_').replace('_', "-")
}
