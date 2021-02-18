use proc_macro2::Ident;
use quote::format_ident;
use quote::quote;
use syn::Type;

use crate::analysis::{EvtNode, Family, ReNode, VarNode};

use super::{Generate, InterfaceTokens};

fn generate_common_source(name: &Ident, fam: Family, ty: &Type) -> InterfaceTokens {
    let mut ift = InterfaceTokens::default();
    ift.update_part = quote! {
        if inputs.#name.is_some() {
            state.#name = inputs.#name.unwrap();
            change.#name = true;
        }
    };
    ift.state_struct = quote! {
        #name: #ty,
    };
    ift.input_struct_part = quote! {
        #name: Option<#ty>,
    };
    ift.change_struct = quote! {
        #name: bool,
    };
    ift
}

impl Generate for VarNode<'_> {
    fn family(&self) -> Family {
        self.family
    }

    fn generate_interface(&self, _: &Vec<&ReNode>) -> InterfaceTokens {
        let name = self.ident();
        let ty = self.ty;
        let initial_state = self.initial;
        let mut ift = generate_common_source(&name, Family::Variable, ty);

        ift.state_default = quote! {
            #name: #initial_state,
        };

        let init = self.initial;
        ift.initial_input = quote! {
            #name: Some(#init),
        };
        ift
    }

    fn ident(&self) -> Ident {
        format_ident!("var_{}", self.id)
    }
}

impl Generate for EvtNode<'_> {
    fn generate_interface(&self, _: &Vec<&ReNode>) -> InterfaceTokens {
        let name = self.ident();
        let ty = self.ty;

        let mut ift = generate_common_source(&name, Family::Event, ty);

        ift.state_default = quote! {
            #name: #ty::default(),
        };

        ift
    }

    fn ident(&self) -> Ident {
        format_ident!("evt_{}", self.id)
    }

    fn family(&self) -> Family {
        self.family
    }
}
