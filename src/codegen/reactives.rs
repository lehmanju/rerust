use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::analysis::{
    ChoiceNode, EvtNode, FilterNode, FoldNode, GroupNode, NameNode, ReEdge, VarNode,
};
use crate::analysis::{MapNode, ReNode};
use petgraph::{graph::NodeIndex, visit::Topo, Graph};

use super::Generate;

impl Generate for MapNode<'_> {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        let name = self.ident();
        let args = &self.update_expr.inputs;
        let return_type = &self.update_expr.return_type;
        let body = &self.update_expr.body;

        quote! {
            #[inline]
            fn #name (#args) -> #return_type {
                #body
            }
        }
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let incoming_node = &incoming[0];
        let name = self.ident();
        let func_name = self.ident();
        (
            quote! {
                let mut #name = state.#func_name;

                    if change.#incoming_node {
                    let val = #incoming_node.unwrap();
                    let result = Self::#func_name(val);
                    if state.#func_name.is_none() || result != state.#func_name.unwrap() {
                        change.#func_name = true;
                        #name = Some(result);
                    }}
            },
            quote! {
                #name,
            },
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = self.ty;
        (
            quote! {
                #ident: Option<#ty>,
            },
            quote! {
                #ident: None,
            },
        )
    }

    fn ident(&self) -> Ident {
        format_ident!("map_{}", self.id)
    }
}

impl Generate for FoldNode<'_> {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        let name = self.ident();
        let args = &self.update_expr.inputs;
        let return_type = &self.update_expr.return_type;
        let body = &self.update_expr.body;
        quote! {
            #[inline]
            fn #name (#args) -> #return_type {
                #body
            }
        }
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let incoming_node = &incoming[0];
        let name = self.ident();
        let func_name = self.ident();
        (
            quote! {
                let mut #name = state.#func_name;
                if change.#incoming_node {
                    let val = #incoming_node.unwrap();
                    let result = Self::#func_name(state.#func_name.clone().unwrap(), val);
                    if result != state.#func_name.unwrap() {
                        change.#func_name = true;
                        #name = Some(result);
                    }
                }
            },
            quote! {
                #func_name,
            },
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = self.ty;
        let expr = self.initial;
        (
            quote! {
                #ident: Option<#ty>,
            },
            quote! {
                #ident: Some(#expr),
            },
        )
    }

    fn ident(&self) -> Ident {
        format_ident!("fold_{}", self.id)
    }
}

impl Generate for GroupNode {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        TokenStream::new()
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let vec = incoming.clone();
        let (tks_ifa, tks_ifb, tks_ret) = gen_group_update(vec);
        let name = self.ident();
        (
            quote! {
                let mut #name = state.#name;
                if #tks_ifb {
                    if #tks_ifa {
                        change.#name = true;
                    }
                    #name = Some((#tks_ret));
                }
            },
            quote! {
                #name,
            },
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = &self.ty;
        (
            quote! {
                #ident: Option<#ty>,
            },
            quote! {
                #ident: None,
            },
        )
    }

    fn ident(&self) -> Ident {
        format_ident!("group_{}", self.id)
    }
}

fn gen_group_update(mut ident: Vec<Ident>) -> (TokenStream, TokenStream, TokenStream) {
    if ident.len() == 1 {
        let elem = &ident[0];
        (
            quote! {
                change.#elem
            },
            quote! {
                !#elem.is_none()
            },
            quote! {
                #elem.unwrap()
            },
        )
    } else {
        let a = ident.pop().expect("Non empty list");
        let (if_clausea, if_clauseb, ret) = gen_group_update(ident);
        (
            quote! {
                change.#a || #if_clausea
            },
            quote! {
                !#a.is_none() && #if_clauseb
            },
            quote! {#a.unwrap(), #ret},
        )
    }
}

impl Generate for FilterNode<'_> {
    fn gen_function(&self, _incoming: &Vec<Ident>) -> TokenStream {
        let func_name = self.ident();
        let expr_inputs = &self.filter_expr.inputs;
        let expr_body = &self.filter_expr.body;
        quote! {
            #[inline]
            fn #func_name(#expr_inputs) -> bool {
                #expr_body
            }
        }
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let name = self.ident();
        let incoming_node = &incoming[0];
        (
            quote! {
                let mut #name = state.#name;
                if change.#incoming_node {
                    let val = #incoming_node.unwrap();
                    if Self::#name(&val) {
                        change.#name = true;
                        #name = Some(val);
                    }
                }
            },
            quote! {
                #name,
            },
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = &self.ty;
        (
            quote! {
                #ident: Option<#ty>,
            },
            quote! {
                #ident: None,
            },
        )
    }

    fn ident(&self) -> Ident {
        format_ident!("filter_{}", self.id)
    }
}

impl Generate for ChoiceNode {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        TokenStream::new()
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let name = self.ident();
        let (a, b) = (&incoming[0], &incoming[1]);
        (
            quote! {
                let mut #name = state.#name;
                if change.#a {
                    #name = #a;
                } else if change.#b {
                    #name = #b;
                }
            },
            quote! {
                #name,
            },
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = &self.ty;
        (
            quote! {
                #ident: Option<#ty>,
            },
            quote! {
                #ident: None,
            },
        )
    }

    fn ident(&self) -> Ident {
        format_ident!("choice_{}", self.id)
    }
}
