use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{self, Let, Semi},
    Error, Expr, Ident, Pat, PatType, Token, Type, Block,
};
use token::{Comma, Paren, RArrow};

#[derive(Debug)]
pub struct ReBlock {
    pub stmts: Vec<ReLocal>,
}

#[derive(Debug)]
pub struct ReLocal {
    pub let_token: Let,
    pub ident: ReIdent,
    pub eq_token: Token![=],
    pub init: ReExpr,
    pub semi_token: Semi,
}

#[derive(Debug, PartialEq)]
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
    pub colon2_token: Token![::],
    pub lt_token: Token![<],
    pub ty: Type,
    pub gt_token: Token![>],
    pub brace: Paren,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct EvtExpr {
    pub evt_token: kw::Evt,
    pub colon2_token: Token![::],
    pub lt_token: Token![<],
    pub ty: Type,
    pub gt_token: Token![>],
    pub brace: Paren,
}

#[derive(Debug)]
pub struct MapExpr {
    pub left_expr: Box<ReExpr>,
    pub map_token: kw::map,
    pub dot_token: Token![.],
    pub paren: Paren,
    pub closure: ReClosure,
}

#[derive(Debug)]
pub struct FoldExpr {
    pub left_expr: Box<ReExpr>,
    pub fold_token: kw::fold,
    pub dot_token: Token![.],
    pub paren: Paren,
    pub init_expr: Expr,
    pub comma_token: Token![,],
    pub closure: ReClosure,
}

#[derive(Debug)]
pub struct FilterExpr {
    pub left_expr: Box<ReExpr>,
    pub filter_token: kw::filter,
    pub dot_token: Token![.],
    pub paren: Paren,
    pub closure: ReClosure,
}

#[derive(Debug)]
pub struct ReClosure {
    pub or1_token: Token![|],
    pub inputs: Punctuated<Pat, Comma>,
    pub or2_token: Token![|],
    pub output_arrow: RArrow,
    pub return_type: Type,
    pub body: Box<Block>,
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
            eq_token: input.parse()?,
            init: input.parse()?,
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
        let colon2_token = input.parse()?;
        let lt_token = input.parse()?;
        let ty = input.parse()?;
        let gt_token = input.parse()?;
        let content;
        let paren = parenthesized!(content in input);
        let rust_expr: Expr = content.parse()?;
        Ok(Self {
            var_token: var,
            colon2_token,
            lt_token,
            ty,
            gt_token,
            brace: paren,
            expr: rust_expr,
        })
    }
}

impl Parse for EvtExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let evt: kw::Evt = input.parse()?;
        let colon2_token = input.parse()?;
        let lt_token = input.parse()?;
        let ty = input.parse()?;
        let gt_token = input.parse()?;
        let content;
        let paren = parenthesized!(content in input);
        if !content.is_empty() {
            return Err(Error::new(paren.span, "unexpected expression"));
        }
        Ok(Self {
            evt_token: evt,
            colon2_token,
            lt_token,
            ty,
            gt_token,
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

impl Parse for ReClosure {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let or1_token = input.parse()?;
        let mut inputs = Punctuated::new();
        loop {
            if input.peek(Token![|]) {
                break;
            }
            let pat: Pat = input.parse()?;
            let value: Pat = if input.peek(Token![:]) {
                Pat::Type(PatType {
                    attrs: Vec::new(),
                    pat: Box::new(pat),
                    colon_token: input.parse()?,
                    ty: input.parse()?,
                })
            } else {
                pat
            };
            inputs.push(value);
            if input.peek(Token![|]) {
                break;
            }
            let punct: Token![,] = input.parse()?;
            inputs.push_punct(punct);
        }
        Ok(ReClosure {
            or1_token,
            inputs,
            or2_token: input.parse()?,
            output_arrow: input.parse()?,
            return_type: input.parse()?,
            body: Box::new(input.parse()?),
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
            let closure = content.parse()?;
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
            let closure = content.parse()?;
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
            let closure = content.parse()?;
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
