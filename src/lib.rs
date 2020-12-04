use compiler::ReBlock;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod compiler {

    use syn::{
        parenthesized,
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        token::{self, Let, Semi},
        Error, Expr, ExprClosure, Ident, Pat, Token,
    };
    use token::Paren;

    #[derive(Debug)]
    pub struct ReBlock {
        pub stmts: Vec<ReLocal>,
    }

    #[derive(Debug)]
    pub struct ReLocal {
        pub let_token: Let,
        pub ident: ReIdent,
        pub init: Option<(syn::token::Eq, ReExpr)>,
        pub semi_token: Semi,
    }

    #[derive(Debug)]
    pub struct ReIdent {
        pub ident: Ident,
    }

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub struct ChoiceExpr {
        pub left_expr: Box<ReExpr>,
        pub oror: Token![||],
        pub right_expr: Box<ReExpr>,
    }

    #[derive(Debug)]
    pub struct GroupExpr {
        pub paren: Paren,
        pub exprs: Punctuated<ReExpr, Token![,]>,
    }

    #[derive(Debug)]
    pub struct VarExpr {
        pub var_token: kw::Var,
        pub brace: Paren,
        pub expr: Expr,
    }

    #[derive(Debug)]
    pub struct EvtExpr {
        pub evt_token: kw::Evt,
        pub brace: Paren,
    }

    #[derive(Debug)]
    pub struct MapExpr {
        pub left_expr: Box<ReExpr>,
        pub map_token: kw::map,
        pub dot_token: Token![.],
        pub paren: Paren,
        pub closure: ExprClosure,
    }

    #[derive(Debug)]
    pub struct FoldExpr {
        pub left_expr: Box<ReExpr>,
        pub fold_token: kw::fold,
        pub dot_token: Token![.],
        pub paren: Paren,
        pub init_expr: Expr,
        pub comma_token: Token![,],
        pub closure: ExprClosure,
    }

    #[derive(Debug)]
    pub struct FilterExpr {
        pub left_expr: Box<ReExpr>,
        pub filter_token: kw::filter,
        pub dot_token: Token![.],
        pub paren: Paren,
        pub closure: ExprClosure,
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
        ReExprStruct := 'Var' '(' RUST_EXPR ')' | 'Evt' '(' ')'
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
            let var: kw::Var = input.parse()?;
            let content;
            let paren = parenthesized!(content in input);
            let rust_expr: Expr = content.parse()?;
            Ok(Self {
                var_token: var,
                brace: paren,
                expr: rust_expr,
            })
        }
    }

    impl Parse for EvtExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let evt: kw::Evt = input.parse()?;
            let content;
            let paren = parenthesized!(content in input);
            if !content.is_empty() {
                return Err(Error::new(content.span(), "unexpected expression"));
            }
            Ok(Self {
                evt_token: evt,
                brace: paren,
            })
        }
    }

    impl Parse for GroupExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            let paren = parenthesized!(content in input);
            let punctuated = content.call(Punctuated::parse_separated_nonempty)?;
            Ok(GroupExpr {
                paren,
                exprs: punctuated,
            })
        }
    }

    fn parse_method(input: ParseStream) -> syn::Result<ReExpr> {
        let mut expr = parse_primary(input)?;
        let mut content;
        while input.peek(Token![.]) {
            let dot: Token![.] = input.parse()?;
            if input.peek(kw::map) {
                let map_token: kw::map = input.parse()?;
                let paren = parenthesized!(content in input);
                let closure = content.call(parse_closure)?;
                expr = ReExpr::Map(MapExpr {
                    left_expr: Box::new(expr),
                    dot_token: dot,
                    map_token,
                    paren,
                    closure,
                })
            } else if input.peek(kw::filter) {
                let filter_token: kw::filter = input.parse()?;
                let paren = parenthesized!(content in input);
                let closure = content.call(parse_closure)?;
                expr = ReExpr::Filter(FilterExpr {
                    left_expr: Box::new(expr),
                    dot_token: dot,
                    filter_token,
                    paren,
                    closure,
                })
            } else if input.peek(kw::fold) {
                let fold_token: kw::fold = input.parse()?;
                let paren = parenthesized!(content in input);
                let init: Expr = content.parse()?;
                let comma = content.parse()?;
                let closure = content.call(parse_closure)?;
                expr = ReExpr::Fold(FoldExpr {
                    left_expr: Box::new(expr),
                    dot_token: dot,
                    fold_token,
                    paren,
                    init_expr: init,
                    closure,
                    comma_token: comma,
                })
            }
        }
        Ok(expr)
    }

    fn parse_primary(input: ParseStream) -> syn::Result<ReExpr> {
        if input.peek(kw::Var) {
            Ok(ReExpr::Var(input.parse()?))
        } else if input.peek(kw::Evt) {
            Ok(ReExpr::Evt(input.parse()?))
        } else if input.peek(token::Paren) {
            Ok(ReExpr::Group(input.parse()?))
        } else {
            Ok(ReExpr::Ident(input.parse()?))
        }
    }

    fn parse_closure(input: ParseStream) -> syn::Result<ExprClosure> {
        let closure: ExprClosure = input.parse()?;
        closure.inputs.iter().try_for_each(|arg| match *arg {
            Pat::Type(_) => Ok(()),
            _ => {
                return Err(Error::new(
                    input.span(),
                    "unexpected pattern in closure argument",
                ));
            }
        })?;
        Ok(closure)
    }
}

#[proc_macro]
pub fn rerust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ReBlock);
    println!("AST: {:?}", input);
    TokenStream::new()
}
