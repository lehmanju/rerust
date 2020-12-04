use compiler::{ReBlock, ReExpr};
use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Block, Local, Stmt,
};

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
        Var(VarExpr),
        Evt(EvtExpr),
        Ident(ReIdent),
        Group(GroupExpr),
        Fold(FoldExpr),
        Choice(ChoiceExpr),
        Map(MapExpr),
        Filter(FilterExpr),
    }

    pub struct ChoiceExpr {
        left_expr: Box<ReExpr>,
        oror: Token![||],
        right_expr: Box<ReExpr>,
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

    pub struct MapExpr {
        map_token: kw::map,
        paren: Paren,
        closure: ExprClosure,
    }

    pub struct FoldExpr {
        fold_token: kw::fold,
        paren: Paren,
        init_expr: Expr,
        closure: ExprClosure,
    }

    pub struct FilterExpr {
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
            let mut method_call = parse_method(input)?;
            while input.peek(Token![||]) {
                let choice_token: Token![||] = input.parse()?;
                let choice_expr = parse_method(input)?;
                method_call = ReExpr::Choice(ChoiceExpr {
                    left_expr: Box::new(method_call),
                    oror: choice_token,
                    right_expr: Box::new(choice_expr),
                })
            }
            Ok(method_call)
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

    impl Parse for VarExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            todo!()
        }
    }

    impl Parse for EvtExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            todo!()
        }
    }

    fn parse_method(input: ParseStream) -> syn::Result<ReExpr> {
        todo!()
    }

    fn parse_primary(input: ParseStream) -> syn::Result<ReExpr> {
        todo!()
    }

    fn parse_transform(input: ParseStream) -> syn::Result<ReExpr> {
        todo!()
    }
}

#[proc_macro]
pub fn rerust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ReBlock);
    todo!()
}
