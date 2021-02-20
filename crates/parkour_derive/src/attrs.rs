use proc_macro2::Span;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Expr, ExprLit, ExprPath, Ident, Lit, Result};

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

pub fn parse(attrs: Vec<Attribute>) -> Result<Vec<(Attr, Span)>> {
    let mut result = Vec::new();

    for a in attrs {
        if let Some(ident) = a.path.get_ident() {
            if *ident == "parkour" {
                parse_parkour_attrs(a.tokens, &mut result)?;
            } else if *ident == "arg" {
                result.push((Attr::Arg(parse_arg_attrs(a.tokens)?), ident.span()));
            }
        }
    }
    Ok(result)
}

fn parse_parkour_attrs(
    tokens: proc_macro2::TokenStream,
    buf: &mut Vec<(Attr, Span)>,
) -> Result<()> {
    let AttrMap(values) = syn::parse2(tokens)?;

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
            (s, _) => {
                return Err(Error::new(id.span(), format!("unexpected key {:?}", s)));
            }
        }
    }
    Ok(())
}

fn parse_arg_attrs(tokens: proc_macro2::TokenStream) -> Result<Arg> {
    let mut long = Vec::new();
    let mut short = Vec::new();
    let mut positional = None;

    let span = tokens.span();
    let AttrMap(values) = syn::parse2(tokens)?;
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
            (s, _) => {
                return Err(Error::new(id.span(), format!("unexpected key {:?}", s)));
            }
        }
    }

    if positional.is_some() && !(long.is_empty() && short.is_empty()) {
        return Err(Error::new(
            span,
            "`arg(positional)` can't be used together with `arg(long)` or `arg(short)`",
        ));
    }
    if let Some(name) = positional {
        Ok(Arg::Positional { name })
    } else {
        Ok(Arg::Named { long, short })
    }
}

struct AttrMap(Vec<(Ident, Option<Expr>)>);

impl Parse for AttrMap {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut res = Vec::new();

        if input.is_empty() {
            return Ok(AttrMap(res));
        }

        let exprs = match input.parse::<syn::Expr>()? {
            Expr::Paren(p) => {
                if !p.attrs.is_empty() {
                    return Err(Error::new(p.span(), "Illegal attribute"));
                }
                let mut punct = Punctuated::new();
                punct.push_value(*p.expr);
                punct
            }
            Expr::Tuple(t) => {
                if !t.attrs.is_empty() {
                    return Err(Error::new(t.span(), "Illegal attribute"));
                }
                t.elems
            }
            expr => return Err(Error::new(expr.span(), "expected parentheses")),
        };

        for expr in exprs {
            match expr {
                Expr::Assign(a) => {
                    if !a.attrs.is_empty() {
                        return Err(Error::new(a.span(), "Illegal attribute"));
                    }
                    if let Expr::Path(left) = *a.left {
                        res.push((parse_ident(left)?, Some(*a.right)));
                    } else {
                        return Err(Error::new(
                            a.span(),
                            "invalid token: expected identifier",
                        ));
                    }
                }
                Expr::Path(p) => {
                    res.push((parse_ident(p)?, None));
                }
                _ => return Err(Error::new(expr.span(), "unsupported expression")),
            }
        }

        Ok(AttrMap(res))
    }
}

fn parse_ident(p: ExprPath) -> Result<Ident> {
    if p.qself.is_none() && p.attrs.is_empty() {
        if let Some(id) = p.path.get_ident() {
            return Ok(id.clone());
        }
    }
    Err(Error::new(p.span(), "invalid token: expected identifier"))
}

fn parse_string(t: &Expr) -> Result<String> {
    match t {
        Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(s.value()),
        _ => Err(Error::new(t.span(), "invalid token: expected string literal")),
    }
}

fn err_on_duplicate(b: bool, span: Span) -> Result<()> {
    if b {
        Err(Error::new(span, "key exists multiple times"))
    } else {
        Ok(())
    }
}
