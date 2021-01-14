use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::analysis::{EvtNode, VarNode};

use super::Generate;

impl Generate for VarNode<'_> {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        let name = self.ident();
        let sender_func = format_ident!("sender_{}", name);
        let ty = self.ty;
        quote! {
            #[inline]
            fn #name(stream: &Receiver<#ty>, old_val: Option<#ty>) -> Option<#ty> {
                let result = stream.try_recv();
                match result {
                    Ok(val) => Self::#name(stream, Some(val)),
                    _ => old_val,
                }
            }

            #[inline]
            fn #sender_func(sources: &Sources) -> Sender<#ty> {
                sources.#name.0.clone()
            }
        }
    }

    fn gen_update(&self, _incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let name = self.ident();
        (
            quote! {

                    let val = Self::#name(&mut sources.#name.1, None);
                    if let Some(v) = val {
                        state.#name = Some(v);
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
        let init = self.initial;
        (
            quote! {
                #ident: Option<#ty>,
            },
            quote! {
                #ident: Some(#init),
            },
        )
    }

    fn ident(&self) -> Ident {
        format_ident!("var_{}", self.id)
    }

    fn gen_source(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = self.ty;
        (
            quote! {
                #ident: (Sender<#ty>, Receiver<#ty>),
            },
            quote! {
                #ident: channel(),
            },
        )
    }
}

impl Generate for EvtNode<'_> {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        let name = self.ident();
        let ty = self.ty;
        let sender_func = format_ident!("sender_{}", name);
        quote! {
            #[inline]
            fn #name(stream: &Receiver<#ty>) -> Option<#ty> {
                let result = stream.try_recv();
                match result {
                    Ok(val) => Some(val),
                    _ => None,
                }
            }

            #[inline]
            fn #sender_func(sources: &Sources) -> Sender<#ty> {
                sources.#name.0.clone()
            }
        }
    }

    fn gen_update(&self, _incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let name = self.ident();
        (
            quote! {

                    let val = Self::#name(&mut sources.#name.1);
                    state.#name =
                    match val {
                        Some(v) => {
                            change.#name = true;
                            Some(v)
                        }
                        _ => None
                    };

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

    fn gen_source(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = self.ty;
        (
            quote! {
                #ident: (Sender<#ty>, Receiver<#ty>),
            },
            quote! {
                #ident: channel(),
            },
        )
    }

    fn ident(&self) -> Ident {
        format_ident!("evt_{}", self.id)
    }
}
