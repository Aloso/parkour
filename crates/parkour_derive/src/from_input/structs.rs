use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Attribute, Fields, GenericArgument, Ident, PathArguments, Result, Type, TypePath,
};

use crate::attrs::{Arg, Attr, Parkour};
use crate::{attrs, utils};

pub fn structs(
    name: &Ident,
    s: syn::DataStruct,
    attr: Vec<Attribute>,
) -> Result<TokenStream> {
    let attrs = attrs::parse(&attr)?;

    let subcommands = get_subcommand_names(&attrs, name)?;

    let is_main = attrs.iter().any(|(a, _)| matches!(a, Attr::Parkour(Parkour::Main)));
    if is_main && !subcommands.is_empty() {
        bail!(
            Span::call_site(),
            "`parkour(main)` and `parkour(subcommand)` can't be combined",
        );
    } else if !is_main && subcommands.is_empty() {
        bail!(
            Span::call_site(),
            "The FromInput derive macro requires a `parkour(main)` or \
             `parkour(subcommand)` attribute",
        );
    }

    let main_condition = if is_main {
        quote! { input.bump_argument().is_some() }
    } else {
        quote! { #( input.parse_command(#subcommands) )||* }
    };

    let field_len = utils::field_len(&s.fields);

    if field_len > 0 {
        if let Fields::Unnamed(_) = s.fields {
            bail!(
                Span::call_site(),
                "The FromInput derive macro doesn't support tuple structs",
            );
        }
    }

    let mut field_idents = Vec::new();
    let mut field_initials = Vec::new();
    let mut field_getters = Vec::new();
    let mut contexts = Vec::new();

    for field in &s.fields {
        let attrs = attrs::parse(&field.attrs)?;
        let ident = field.ident.as_ref().expect("a field has no ident");

        let ty = parse_my_type(&field.ty);

        let mut field_str = None;

        let mut args = Vec::new();
        for (attr, span) in attrs {
            if let Attr::Arg(a) = attr {
                args.push(match a {
                    Arg::Named { long, short } => {
                        if long.is_empty() && short.is_empty() {
                            bail!(span, "no flags specified");
                        }

                        let main_flag = long
                            .iter()
                            .find_map(|f| f.as_deref().map(ToString::to_string))
                            .unwrap_or_else(|| utils::ident_to_flag_string(ident));

                        if field_str.is_none() {
                            field_str = Some(format!("--{}", &main_flag));
                        }

                        let (long, short) =
                            flatten_flags(span, &main_flag, &long, &short)?;
                        generate_flag_context(&long, &short)
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
                bail!(span, "this key is not yet implemented!");
            }
        }

        if args.is_empty() {
            bail!(ident.span(), "This field is missing a `arg` attribute");
        }
        contexts.push(args);

        field_idents.push(ident);

        field_initials.push(match ty {
            MyType::Bool => quote! { false },
            _ => quote! { None },
        });

        let field_str = field_str.expect("a field has no string");
        field_getters.push(match ty {
            MyType::Bool | MyType::Option(_) => quote! {},
            MyType::Other(_) => quote! {
                .ok_or_else(|| {
                    parkour::Error::missing_argument(#field_str)
                })?
            },
        });
    }

    let gen = quote! {
        #[automatically_derived]
        impl parkour::FromInput<'static> for #name {
            type Context = ();

            fn from_input<P: parkour::Parse>(input: &mut P, _: &Self::Context)
                    -> parkour::Result<Self> {
                if #main_condition {
                    #(
                        let mut #field_idents = #field_initials;
                    )*
                    while input.is_not_empty() {
                        if input.parse_long_flag("") {
                            input.set_ignore_dashes(true);
                        }

                        #(
                            #(
                                if parkour::actions::SetOnce(&mut #field_idents)
                                    .apply(input, &#contexts)?
                                {
                                    input.expect_end_of_argument()?;
                                    continue;
                                }
                            )*
                        )*

                        input.expect_empty()?;
                    }
                    Ok(#name {
                        #(
                            #field_idents: #field_idents #field_getters,
                        )*
                    })
                } else {
                    Err(parkour::Error::no_value())
                }
            }
        }
    };
    Ok(gen)
}

enum MyType<'a> {
    Bool,
    Option(&'a Type),
    Other(&'a Type),
}

fn is_bool(path: &TypePath) -> bool {
    if path.qself.is_none() {
        if let Some(ident) = path.path.get_ident() {
            return ident == "bool";
        }
    }
    false
}

fn parse_my_type(ty: &Type) -> MyType<'_> {
    if let Type::Path(path) = ty {
        if is_bool(&path) {
            return MyType::Bool;
        } else if path.qself.is_none() {
            let segments = path.path.segments.iter().collect::<Vec<_>>();

            let is_option = (segments.len() == 1 && segments[0].ident == "Option")
                || (segments.len() == 3
                    && (segments[0].ident == "std" || segments[0].ident == "core")
                    && segments[0].arguments.is_empty()
                    && segments[1].ident == "option"
                    && segments[1].arguments.is_empty()
                    && segments[2].ident == "Option");

            if is_option {
                if let PathArguments::AngleBracketed(a) =
                    &segments[segments.len() - 1].arguments
                {
                    if let Some(GenericArgument::Type(t)) = a.args.iter().next() {
                        return MyType::Option(t);
                    }
                }
            }
        }
    }
    MyType::Other(ty)
}

fn generate_flag_context(long: &[&str], short: &[&str]) -> TokenStream {
    match (long.len(), short.len()) {
        (1, 1) => {
            let long = long[0];
            let short = short[0];
            quote! { parkour::util::Flag::LongShort(#long, #short).into() }
        }
        (0, 1) => {
            let short = short[0];
            quote! { parkour::util::Flag::Short(#short).into() }
        }
        (1, 0) => {
            let long = long[0];
            quote! { parkour::util::Flag::Long(#long).into() }
        }
        (_, _) => quote! {
            parkour::util::Flag::Many(vec![
                #( parkour::util::Flag::Long(#long), )*
                #( parkour::util::Flag::Short(#short), )*
            ]).into()
        },
    }
}

fn flatten_flags<'a>(
    span: Span,
    main_flag: &'a str,
    long: &'a [Option<String>],
    short: &'a [Option<String>],
) -> Result<(Vec<&'a str>, Vec<&'a str>)> {
    let main_short = utils::first_char(span, main_flag)?;

    let mut long: Vec<&str> =
        long.iter().map(|o| o.as_deref().unwrap_or(main_flag)).collect();
    let mut short: Vec<&str> =
        short.iter().map(|o| o.as_deref().unwrap_or(main_short)).collect();

    long.sort_unstable();
    short.sort_unstable();

    if let Some(w) = long.windows(2).find(|pair| pair[0] == pair[1]) {
        bail!(span, "long flag {:?} is specified twice", w[0]);
    }
    if let Some(w) = short.windows(2).find(|pair| pair[0] == pair[1]) {
        bail!(span, "short flag {:?} is specified twice", w[0]);
    }

    Ok((long, short))
}

fn get_subcommand_names(attrs: &[(Attr, Span)], name: &Ident) -> Result<Vec<String>> {
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

    if let Some(w) = subcommands.windows(2).find(|&pair| pair[0] == pair[1]) {
        bail!(Span::call_site(), "subcommand {:?} is specified twice", w[0]);
    }
    Ok(subcommands)
}
