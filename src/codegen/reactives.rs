use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;
use syn::{Type, TypeReference};

use crate::analysis::{ChoiceNode, Family, FilterNode, FoldNode, MapNode, ReNode};

use super::{Generate, InterfaceTokens};

impl Generate for MapNode<'_> {
    fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens {
        let mut ift = InterfaceTokens::default();
        let name = self.ident();
        let args = &self.update_expr.inputs;
        let return_type = &self.update_expr.return_type;
        let body = &self.update_expr.body;
        ift.functions = quote! {
            #[inline]
            fn #name (#args) -> #return_type
                #body
        };

        let family = self.family();
        let (event_condition, var_condition) = generate_condition(incoming.clone(), family);
        let method_args = generate_method_args(incoming.clone());

        if family == Family::Event {
            ift.update_part = quote! {
                if #event_condition {
                    state.#name = Self::#name(#method_args);
                    change.#name = true;
                }
            };
        } else {
            ift.update_part = quote! {
                if #var_condition {
                    let result = Self::#name(#method_args);
                    if result != state.#name {
                        state.#name = result;
                        change.#name = true;
                    }
                }
            }
        }
        let ty = self.ty;
        ift.state_struct = quote! {
            #name: #ty,
        };
        ift.change_struct = quote! {
            #name: bool,
        };
        ift
    }

    fn ident(&self) -> Ident {
        format_ident!("map_{}", self.id)
    }

    fn family(&self) -> Family {
        self.family
    }
}

fn generate_condition(mut incoming: Vec<&ReNode>, family: Family) -> (TokenStream, TokenStream) {
    if incoming.len() == 1 {
        let node = incoming[0];
        let name = node.ident();
        let fam = node.family();
        if family == Family::Event && fam == Family::Variable {
            return (quote! {}, quote! {});
        }
        if fam == Family::Event {
            (quote! { change.#name }, quote! {})
        } else {
            (quote! {}, quote! { change.#name })
        }
    } else {
        let node = incoming.pop().unwrap();
        let fam = node.family();
        let name = node.ident();
        let (rest_events, rest_variables) = generate_condition(incoming, family);
        if family == Family::Event && fam == Family::Variable {
            return (rest_events, rest_variables);
        }
        if fam == Family::Event {
            (quote! { change.#name && #rest_events }, rest_variables)
        } else {
            (rest_events, quote! { change.#name || #rest_variables })
        }
    }
}

fn generate_method_args(mut incoming: Vec<&ReNode>) -> TokenStream {
    if incoming.len() == 1 {
        let node = incoming[0];
        let name = node.ident();

        quote! { &state.#name }
    } else {
        let node = incoming.pop().unwrap();
        let name = node.ident();
        let rest = generate_method_args(incoming);
        quote! { &state.#name, #rest }
    }
}

impl Generate for FoldNode<'_> {
    fn ident(&self) -> Ident {
        format_ident!("fold_{}", self.id)
    }

    fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens {
        let mut ift = InterfaceTokens::default();
        let name = self.ident();
        let args = &self.update_expr.inputs;
        let return_type = &self.update_expr.return_type;
        let body = &self.update_expr.body;
        ift.functions = quote! {
            #[inline(always)]
            fn #name (#args) -> #return_type
                #body
        };

        let family = self.family();
        let (event_condition, var_condition) = generate_condition(incoming.clone(), family);
        let method_args = generate_method_args(incoming.clone());

        if family == Family::Event {
            ift.update_part = quote! {
                if #event_condition {
                    let result = Self::#name(state.#name.clone(), #method_args);
                    if result != state.#name {
                        state.#name = result;
                        change.#name = true;
                    }
                }
            };
        } else {
            ift.update_part = quote! {
                if #var_condition {
                    let result = Self::#name(state.#name.clone(), #method_args);
                    if result != state.#name {
                        state.#name = result;
                        change.#name = true;
                    }
                }
            }
        }
        let ty = self.ty;
        ift.state_struct = quote! {
            #name: #ty,
        };
        ift.change_struct = quote! {
            #name: bool,
        };

        let init_expr = self.initial;
        ift.state_default = quote! {
            #name: #init_expr,
        };
        ift
    }

    fn family(&self) -> Family {
        self.family
    }
}

impl Generate for FilterNode<'_> {
    fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens {
        let mut ift = InterfaceTokens::default();
        let name = self.ident();
        let inputs = &self.filter_expr.inputs;
        let body = &self.filter_expr.body;

        ift.functions = quote! {
            #[inline]
            fn #name (#inputs) -> bool
                #body

        };

        let family = self.family();
        assert!(incoming.len() == 1);
        let node = incoming[0].ident();

        if family == Family::Event {
            ift.update_part = quote! {
                if change.#node {
                    if Self::#name(&#node) {
                        state.#name = #node.clone();
                        change.#name = true;
                    }
                }
            };
        } else {
            ift.update_part = quote! {
                if change.#node {
                    if Self::#name(&#node) && #node != state.#name {
                        state.#name = #node.clone();
                        change.#name = true;
                    }
                }
            }
        }
        let ty = &self.ty;
        ift.state_struct = quote! {
            #name: #ty,
        };
        ift.change_struct = quote! {
            #name: bool,
        };
        ift
    }

    fn family(&self) -> Family {
        self.family
    }

    fn ident(&self) -> Ident {
        format_ident!("filter_{}", self.id)
    }
}

impl Generate for ChoiceNode {
    fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens {
        let mut ift = InterfaceTokens::default();
        let name = self.ident();

        assert!(incoming.len() == 2);
        let node_a = incoming[0].ident();
        let node_b = incoming[1].ident();

        ift.update_part = quote! {
            if change.#node_a {
                state.#name = state.#node_a.clone();
                change.#name = true;
            } else if change.#node_b {
                state.#name = state.#node_b.clone();
                change.#name = true;
            }
        };

        let ty = &self.ty;
        ift.state_struct = quote! {
            #name: #ty,
        };
        ift.change_struct = quote! {
            #name: bool,
        };
        ift
    }
    fn ident(&self) -> Ident {
        format_ident!("choice_{}", self.id)
    }
    fn family(&self) -> Family {
        self.family
    }
}
