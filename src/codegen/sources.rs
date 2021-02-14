use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::analysis::{EvtNode, Family, VarNode, ReNode};

use super::{Generate, InterfaceTokens};

impl Generate for  VarNode<'_> {
	fn family(&self) -> Family {
		self.family
	}

	fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens {
		let mut ift = InterfaceTokens::default();
		let name = self.ident();
		let ty = self.ty;
		let initial_state = self.initial;
		ift.update_part = quote! {
            if inputs.#name.is_some() {
                mem::swap(&mut state.#name, &mut inputs.#name);
                change.#name = true;
            }
        };
		ift.state_struct = quote! {
            #name: #ty,
        }      	;
		ift.state_default = quote! {
			#name: #initial_state,
		};
		ift.input_struct_part = quote! {
			#name: Option<#ty>,
		};
		let init = self.initial;
        ift.initial_input = quote! {
            #name: Some(#init),
        };
		ift.change_struct = quote! {
			#name: bool,
		};
		ift
	}
	
    fn ident(&self) -> &Ident {
        &format_ident!("var_{}", self.id)
    }
}

impl Generate for EvtNode<'_> {

	fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens {
		let mut ift = InterfaceTokens::default();
        let name = self.ident();
        let ty = &self.ty;
		ift.update_part = quote! {
            mem::swap(&mut state.#name, &mut inputs.#name);
            change.#name = state.#name.is_some();
        };
        ift.events_struct = quote! {
            #name: Option<#ty>,
        };
		ift.input_struct_part = ift.events_struct;
		ift
	}
	
    fn ident(&self) -> &Ident {
        &format_ident!("evt_{}", self.id)
    }

    fn gen_initial_input(&self) -> TokenStream {
        let ident = self.ident();
        quote! {
            #ident: None,
        }
    }

    fn family(&self) -> Family {
        self.family
    }
}
