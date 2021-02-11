use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::analysis::{EvtNode, VarNode};

use super::{Generate, InterfaceTokens};

impl Generate for VarNode<'_> {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        let name = self.ident();
        let sender_func = format_ident!("sender_{}", name);
        let ty = self.ty;
        quote! {}
    }

    fn gen_update(&self, _incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let name = self.ident();
        (
            quote! {
                if inputs.#name.is_some() {
                    mem::swap(&mut state.#name, &mut inputs.#name);
                    change.#name = true;
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
        format_ident!("var_{}", self.id)
    }

    fn gen_source(&self, incoming: &Vec<Ident>) -> InterfaceTokens {
        let mut ift = InterfaceTokens::default();
        ift.input_struct_part = self.gen_state().0;
        ift
    }

    fn gen_initial_input(&self) -> TokenStream {
        let ident = self.ident();
        let init = self.initial;
        quote! {
            #ident: Some(#init),
        }
    }
}

impl Generate for EvtNode<'_> {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        let name = self.ident();
        let ty = self.ty;
        let sender_func = format_ident!("sender_{}", name);
        quote! {}
    }

    fn gen_update(&self, _incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let name = self.ident();
        (
            quote! {
                mem::swap(&mut state.#name, &mut inputs.#name);
                change.#name = state.#name.is_some();
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

    fn gen_source(&self, incoming: &Vec<Ident>) -> InterfaceTokens {
        let mut ift = InterfaceTokens::default();
        ift.input_struct_part = self.gen_state().0;
        ift
    }

    fn ident(&self) -> Ident {
        format_ident!("evt_{}", self.id)
    }

    fn gen_initial_input(&self) -> TokenStream {
        let ident = self.ident();
        quote! {
            #ident: None,
        }
    }
}
