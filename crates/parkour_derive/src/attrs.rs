use proc_macro2::{Span, TokenTree};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Error, LitStr, Result};

#[derive(PartialEq, Eq)]
pub enum Attr {
    Parkour(Parkour),
    Arg(Arg),
}

#[derive(PartialEq, Eq)]
pub enum Parkour {
    Main,
    Subcommand(Option<String>),
}

#[derive(PartialEq, Eq)]
pub struct Arg {
    pub long: Option<Option<String>>,
    pub short: Option<Option<String>>,
}

pub fn parse(attrs: Vec<Attribute>) -> Result<Vec<Attr>> {
    let mut result = Vec::new();

    for a in attrs {
        if let Some(ident) = a.path.get_ident() {
            if *ident == "parkour" {
                parse_parkour_attrs(a.path.span(), a.tokens, &mut result)?;
            } else if *ident == "arg" {
                result.push(Attr::Arg(parse_arg_attrs(a.path.span(), a.tokens)?));
            }
        }
    }
    Ok(result)
}

fn parse_parkour_attrs(
    path_span: Span,
    tokens: proc_macro2::TokenStream,
    buf: &mut Vec<Attr>,
) -> Result<()> {
    let span = tokens.span();
    match tokens.into_iter().next() {
        Some(TokenTree::Group(g)) => {
            let mut values = Vec::new();
            parse_attr_tokens(g.stream(), &mut values)?;
            for (k, sp, v) in values {
                match (k.as_str(), v) {
                    ("main", None) => {
                        buf.push(Attr::Parkour(Parkour::Main));
                    }
                    ("subcommand", Some(t)) => {
                        let s = parse_string(&t)?;
                        buf.push(Attr::Parkour(Parkour::Subcommand(Some(s))));
                    }
                    ("subcommand", None) => {
                        buf.push(Attr::Parkour(Parkour::Subcommand(None)));
                    }
                    (s, _) => {
                        return Err(Error::new(sp, format!("unexpected key {:?}", s)));
                    }
                }
            }
            Ok(())
        }
        Some(_) => Err(Error::new(span, "expected parentheses")),
        _ => Err(Error::new(path_span, "expected parentheses")),
    }
}

fn parse_arg_attrs(path_span: Span, tokens: proc_macro2::TokenStream) -> Result<Arg> {
    let span = tokens.span();
    match tokens.into_iter().next() {
        Some(TokenTree::Group(g)) => {
            let mut long = None;
            let mut short = None;

            let mut values = Vec::new();
            parse_attr_tokens(g.stream(), &mut values)?;
            for (k, _, v) in values {
                match (k.as_str(), v) {
                    ("long", None) => {
                        long = Some(None);
                    }
                    ("long", Some(t)) => {
                        long = Some(Some(parse_string(&t)?));
                    }
                    ("short", None) => {
                        short = Some(None);
                    }
                    ("short", Some(t)) => {
                        short = Some(Some(parse_string(&t)?));
                    }
                    _ => {
                        return Err(Error::new(span, "unexpected token"));
                    }
                }
            }
            Ok(Arg { long, short })
        }
        Some(_) => Err(Error::new(span, "expected parentheses")),
        _ => Err(Error::new(path_span, "expected parentheses")),
    }
}

enum State {
    Initial,
    Ident(Span, String),
    Eq(Span, String),
    Value(Span, String, TokenTree),
}

fn parse_attr_tokens(
    stream: proc_macro2::TokenStream,
    values: &mut Vec<(String, Span, Option<TokenTree>)>,
) -> Result<()> {
    let mut state = State::Initial;

    for s in stream {
        match state {
            State::Initial => match s {
                TokenTree::Ident(i) => {
                    let new_id = i.to_string();
                    state = State::Ident(i.span(), new_id);
                }
                _ => return Err(Error::new(s.span(), "expected ident")),
            },
            State::Ident(sp, k) => {
                let span = s.span();
                match s {
                    TokenTree::Punct(p) if p.as_char() == '=' => {
                        state = State::Eq(sp, k);
                    }
                    TokenTree::Punct(p) if p.as_char() == ',' => {
                        values.push((k, sp, None));
                        state = State::Initial;
                    }
                    _ => return Err(Error::new(span, "unexpected token")),
                }
            }
            State::Eq(sp, k) => {
                state = match s {
                    TokenTree::Group(_) | TokenTree::Ident(_) | TokenTree::Literal(_) => {
                        State::Value(sp, k, s)
                    }
                    TokenTree::Punct(p) => {
                        return Err(Error::new(p.span(), "unexpected punctuation"));
                    }
                };
            }
            State::Value(sp, k, v) => {
                let span = s.span();
                match s {
                    TokenTree::Punct(p) if p.as_char() == ',' => {
                        values.push((k, sp.join(span).unwrap(), Some(v)));
                        state = State::Initial;
                    }
                    _ => return Err(Error::new(span, "unexpected token")),
                }
            }
        }
    }
    match state {
        State::Initial => {}
        State::Ident(sp, id) => {
            values.push((id, sp, None));
        }
        State::Value(sp, k, v) => {
            values.push((k, sp.join(v.span()).unwrap(), Some(v)));
        }
        State::Eq(sp, _) => {
            return Err(Error::new(sp, "unexpected trailing punctuation"));
        }
    }
    Ok(())
}

fn parse_string(t: &TokenTree) -> Result<String> {
    let tokens = t.to_token_stream().into();
    let lit = syn::parse::<LitStr>(tokens)?;
    Ok(lit.value())
}
