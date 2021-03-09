use proc_macro2::Ident;
use quote::format_ident;
use quote::quote;

use crate::analysis::{EvtNode, NodeData, ReNode, VarNode};

use super::{change_prefix, Generate, InterfaceTokens, temp_prefix};

impl Generate for VarNode<'_> {
    fn generate_interface(&self, _: &Vec<&ReNode>) -> InterfaceTokens {
        let name = self.ident();
        let ty = self.ty();
        let initial_state = self.initial;
        let mut ift = InterfaceTokens::default();
        let change_name = change_prefix(&name);
		let temp_name = temp_prefix(&name);

        ift.update_part = quote! {
            if let Some(val) = inputs.#name {
                state.#name.change = val != state.#name.value;
                state.#name.value = val;
            }
            let #name = &state.#name.value;
            let #change_name = state.#name.change;
        };

        ift.state_struct = quote! {
            #name: Variable<#ty>,
        };

        ift.input_struct_part = quote! {
            #name: Option<#ty>,
        };

        ift.initialize = quote! {
            let #temp_name = Variable { value: #initial_state, change: true };
			let #name = &#temp_name.value;
        };

		ift.initialize_struct = quote! {
			#name: #temp_name,
		};
		
        let init = self.initial;
        ift.initial_input = quote! {
            #name: Some(#init),
        };
        ift
    }

    fn ident(&self) -> Ident {
        format_ident!("var_{}", self.id())
    }
}

impl Generate for EvtNode {
    fn generate_interface(&self, _: &Vec<&ReNode>) -> InterfaceTokens {
        let name = self.ident();
        let ty = self.ty();

        let mut ift = InterfaceTokens::default();

        ift.update_part = quote! {
            if let Some(val) = inputs.#name {
                state.#name = Event::Some(val);
            }
            let #name = &state.#name;
        };

        ift.state_struct = quote! {
            #name: Event<#ty>,
        };

        ift.input_struct_part = quote! {
            #name: Option<#ty>,
        };

        ift.initialize_struct = quote! {
            #name: Event::None,
        };

        ift
    }

    fn ident(&self) -> Ident {
        format_ident!("evt_{}", self.id())
    }
}
