use proc_macro::TokenStream;
use syn::{parse_macro_input, Block, Local, Stmt};
use compiler::ReBlock;

mod compiler {

    use crate::rerust;
    use proc_macro::TokenStream;
    use std::{cell::RefCell, rc::Rc};
    use syn::{
        braced, parenthesized,
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        token::{self, Let, Semi},
        Attribute, Block, Error, Expr, ExprClosure, Ident, LitStr, Local, Pat, PatType, Token,
        Type,
    };
    use token::{Brace, Comma, Paren};

    pub struct ReBlock {
        stmts: Vec<ReLocal>,
    }

    pub struct ReLocal {
        let_token: Let,
        ident: ReIdent,
        init: Option<(syn::token::Eq, ReExpr)>,
        semi_token: Semi,
    }

    pub struct ReIdent {
        ident: Ident,
    }

    pub enum ReExpr {
        Ident(ReIdent),
        Group(GroupExpr),
        MethodCall(CallExpr),
        Var(VarExpr),
        Evt(EvtExpr),
        Choice(ChoiceExpr),
    }

    pub struct GroupExpr {
        paren_open: Paren,
        exprs: Punctuated<ReExpr, Token![,]>,
    }

    pub struct VarExpr {
        var_token: kw::Var,
        brace: Brace,
        expr: Expr,
    }

    pub struct EvtExpr {
        evt_token: kw::Evt,
        brace: Brace,
    }

    pub struct ChoiceExpr {
        expr_left: Box<ReExpr>,
        or_token: Token![||],
        expr_right: Box<ReExpr>,
    }

    pub struct CallExpr {
        expr_left: Box<ReExpr>,
        dot_token: Token![.],
        transform: ReTransform,
    }

    pub enum ReTransform {
        Map(ReMap),
        Fold(ReFold),
        Filter(ReFilter),
    }

    pub struct ReMap {
        map_token: kw::map,
        paren: Paren,
        closure: ExprClosure,
    }

    pub struct ReFold {
        fold_token: kw::fold,
        paren: Paren,
        init_expr: Expr,
        closure: ExprClosure,
    }

    pub struct ReFilter {
        filter_token: kw::filter,
        paren: Paren,
        closure: ExprClosure,
    }

    pub mod kw {
        syn::custom_keyword!(filter);
        syn::custom_keyword!(map);
        syn::custom_keyword!(fold);
        syn::custom_keyword!(Var);
        syn::custom_keyword!(Evt);
    }

    fn is_keyword(str: String) -> bool {
        match str.to_string().as_str() {
            "Var" | "Evt" | "map" | "fold" | "filter" => true,
            _ => false,
        }
    }

    /*
        ReLet := 'let' <ident> '=' ReExpr ';'
        ReExpr := Literal | Grouping | Binary
        Literal := <ident> | <ReExprStruct>
        Grouping := '(' ReExpr ( ',' ReExpr )* ')'
        Binary := ReExpr '.' ReTransform | ReExpr '||' ReExpr
        ReTransform := 'map' '(' RUST_CLOSURE ')' | 'fold' '(' RUST_EXPR ',' RUST_CLOSURE ')'
        ReExprStruct := 'Var' '{' RUST_EXPR '}' | 'Evt' '{' '}'
    */

    impl Parse for ReBlock {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            //loop over Local statements https://docs.rs/syn/1.0.53/src/syn/stmt.rs.html#110
            let mut let_stmts = Vec::new();
            loop {
                if input.is_empty() {
                    break;
                }
                let stmt: ReLocal = input.parse()?;
                let_stmts.push(stmt);
            }
            Ok(Self { stmts: let_stmts })
        }
    }

    impl Parse for ReLocal {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(ReLocal {
                let_token: input.parse()?,
                ident: input.parse()?,
                init: {
                    if input.peek(Token![=]) {
                        let eq_token: Token![=] = input.parse()?;
                        let init: ReExpr = input.parse()?;
                        Some((eq_token, init))
                    } else {
                        None
                    }
                },
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ReExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            if input.peek(token::Paren) {
                let content;
                let paren_token = parenthesized!(content in input);
                let mut elems = Punctuated::new();
                let first: ReExpr = content.parse()?;
                elems.push(first);
                while !content.is_empty() {
                    let punct = content.parse()?;
                    elems.push(punct);
                    if content.is_empty() {
                        break;
                    }
                    let value = content.parse()?;
                    elems.push(value);
                }
                Ok(ReExpr::Group(GroupExpr {
                    paren_open: paren_token,
                    exprs: elems,
                }))
            } else if input.peek(kw::Evt) {
                let evt_token: kw::Evt = input.parse()?;
                let content;
                let braces = braced!(content in input);
                if !content.is_empty() {
                    return Err(Error::new(content.span(), "expected empty body"));
                }
                Ok(ReExpr::Evt(EvtExpr {
                    evt_token,
                    brace: braces,
                }))
            } else if input.peek(kw::Var) {
                let var_token: kw::Var = input.parse()?;
                let content;
                let braces = braced!(content in input);
                let expr: Expr = content.parse()?;
                Ok(ReExpr::Var(VarExpr {
                    var_token,
                    brace: braces,
                    expr,
                }))
            } else {
                let ident: ReIdent = input.parse()?;
                if input.peek(Token![.]) {
                } else {
                    Ok(ReExpr::Ident(ident))
                }
            }
        }
    }

    impl Parse for ReIdent {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let ident: Ident = input.parse()?;
            if is_keyword(ident.to_string()) {
                return Err(Error::new(ident.span(), "expected identifier"));
            }
            Ok(Self { ident })
        }
    }
}

#[proc_macro]
pub fn rerust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ReBlock);
    todo!()
}
