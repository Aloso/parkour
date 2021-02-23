use proc_macro2::{Span, TokenStream};
use syn::spanned::Spanned;
use syn::{Attribute, Expr, ExprLit, Lit, Result};

use crate::parse_attrs;

pub enum Attr {
    Parkour(Parkour),
    Arg(Arg),
}

pub enum Parkour {
    Main,
    Default(Option<Box<Expr>>),
    Subcommand(Option<String>),
}

#[derive(PartialEq, Eq)]
pub enum Arg {
    Named { long: Vec<Option<String>>, short: Vec<Option<String>> },
    Positional { name: Option<String> },
}

pub fn parse(attrs: &[Attribute]) -> Result<Vec<(Attr, Span)>> {
    let mut result = Vec::new();

    for a in attrs {
        if let Some(ident) = a.path.get_ident() {
            if *ident == "parkour" {
                parse_parkour_attrs(&a.tokens, &mut result)?;
            } else if *ident == "arg" {
                result.push((Attr::Arg(parse_arg_attrs(&a.tokens)?), ident.span()));
            }
        }
    }
    Ok(result)
}

fn parse_parkour_attrs(tokens: &TokenStream, buf: &mut Vec<(Attr, Span)>) -> Result<()> {
    let values = parse_attrs::parse(tokens)?;

    for (id, v) in values {
        match (id.to_string().as_str(), v) {
            ("main", None) => {
                buf.push((Attr::Parkour(Parkour::Main), id.span()));
            }
            ("subcommand", Some(t)) => {
                let s = parse_string(&t)?;
                buf.push((Attr::Parkour(Parkour::Subcommand(Some(s))), id.span()));
            }
            ("subcommand", None) => {
                buf.push((Attr::Parkour(Parkour::Subcommand(None)), id.span()));
            }
            ("default", Some(t)) => {
                buf.push((Attr::Parkour(Parkour::Default(Some(Box::new(t)))), id.span()));
            }
            ("default", None) => {
                buf.push((Attr::Parkour(Parkour::Default(None)), id.span()));
            }
            (s, _) => bail!(id.span(), "unexpected key {:?}", s),
        }
    }
    Ok(())
}

fn parse_arg_attrs(tokens: &TokenStream) -> Result<Arg> {
    let mut long = Vec::new();
    let mut short = Vec::new();
    let mut positional = None;

    let span = tokens.span();
    let values = parse_attrs::parse(tokens)?;
    for (id, v) in values {
        match (id.to_string().as_str(), v) {
            ("long", None) => {
                long.push(None);
            }
            ("long", Some(t)) => {
                long.push(Some(parse_string(&t)?));
            }
            ("short", None) => {
                short.push(None);
            }
            ("short", Some(t)) => {
                short.push(Some(parse_string(&t)?));
            }
            ("positional", None) => {
                err_on_duplicate(positional.is_some(), id.span())?;
                positional = Some(None);
            }
            ("positional", Some(p)) => {
                err_on_duplicate(positional.is_some(), id.span())?;
                positional = Some(Some(parse_string(&p)?));
            }
            (s, _) => bail!(id.span(), "unexpected key {:?}", s),
        }
    }

    if positional.is_some() && !(long.is_empty() && short.is_empty()) {
        bail!(
            span,
            "`arg(positional)` can't be used together with `arg(long)` or `arg(short)`",
        );
    }
    if let Some(name) = positional {
        Ok(Arg::Positional { name })
    } else {
        Ok(Arg::Named { long, short })
    }
}

fn parse_string(t: &Expr) -> Result<String> {
    match t {
        Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(s.value()),
        _ => bail!(t.span(), "invalid token: expected string literal"),
    }
}

fn err_on_duplicate(b: bool, span: Span) -> Result<()> {
    if b {
        bail!(span, "key exists multiple times");
    } else {
        Ok(())
    }
}
