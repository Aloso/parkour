use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Expr, ExprPath, Ident, Result};

pub fn parse(tokens: TokenStream) -> Result<Vec<(Ident, Option<Expr>)>> {
    let AttrMap(values) = syn::parse2(tokens)?;
    Ok(values)
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
                    bail!(p.span(), "Illegal attribute");
                }
                let mut punct = Punctuated::new();
                punct.push_value(*p.expr);
                punct
            }
            Expr::Tuple(t) => {
                if !t.attrs.is_empty() {
                    bail!(t.span(), "Illegal attribute");
                }
                t.elems
            }
            expr => bail!(expr.span(), "expected parentheses"),
        };

        for expr in exprs {
            match expr {
                Expr::Assign(a) => {
                    if !a.attrs.is_empty() {
                        bail!(a.span(), "Illegal attribute");
                    }
                    if let Expr::Path(left) = *a.left {
                        res.push((parse_ident(left)?, Some(*a.right)));
                    } else {
                        bail!(a.span(), "invalid token: expected identifier");
                    }
                }
                Expr::Path(p) => {
                    res.push((parse_ident(p)?, None));
                }
                _ => bail!(expr.span(), "unsupported expression"),
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
    bail!(p.span(), "invalid token: expected identifier")
}
