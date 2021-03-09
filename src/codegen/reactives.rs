use super::{change_prefix, temp_prefix, val_prefix, Generate, InterfaceTokens};
use crate::analysis::{
    ChangedNode, Family, FilterNode, FoldNode, MapNode, NodeData, ReNode,
};
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

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
        let ty = self.ty();
        let change_name = change_prefix(&name);
        let temp_name = temp_prefix(&name);

        if self.pin() {
            if family == Family::Event {
                ift.state_struct = quote! {
                    #name: Event<#ty>,
                };
				ift.initialize_struct = quote! {
					#name: Event::None,
				};
                ift.update_part = quote! {
                    if_chain! {
                        #event_condition
                        then {
                            state.#name = Event::Some(Self::#name(#method_args));
                        } else {
                            state.#name = Event::None;
                        }
                    }
                    let #name = &state.#name;
                };
            } else {
                ift.state_struct = quote! {
                    #name: Variable<#ty>,
                };
                ift.update_part = quote! {
                    if_chain! {
                        if #var_condition;
                        then {
                            let result = Self::#name(#method_args);
                            if result != state.#name.value {
                                state.#name.value = result;
                                state.#name.change = true;
                            }
                        }
                    }
                    let #name = &state.#name.value;
                    let #change_name = state.#name.change;
                };
				ift.initialize = quote! {
					let #temp_name = Variable { value: Self::#name(#method_args), change: true };
					let #name = &#temp_name.value;
				};
				ift.initialize_struct = quote! {
					#name: #temp_name,
				};
            }
        } else {
            if family == Family::Event {
                ift.update_part = quote! {
                    let #temp_name = if_chain! {
                        if #event_condition;
                        then {
                            Event::Some(Self::#name(#method_args))
                        } else {
                            Event::None
                        }
                    };
                    let #name = &#temp_name;
                };
            } else {
				ift.initialize = quote! {
					let #temp_name = Self::#name(#method_args);
					let #name = &#temp_name;
				};
                ift.update_part = quote! {
                    let #temp_name = Self::#name(#method_args);
                    let #name = &#temp_name;
					let #change_name = true;
                };
            }
        }
        ift
    }

    fn ident(&self) -> Ident {
        format_ident!("map_{}", self.id())
    }
}

fn generate_condition(mut incoming: Vec<&ReNode>, family: Family) -> (TokenStream, TokenStream) {
    if incoming.len() == 1 {
        let node = incoming[0];
        let name = node.ident();
		let change_name = change_prefix(&name);
        let fam = node.family();
        let local_name = val_prefix(&name);
        if family == Family::Event && fam == Family::Variable {
            return (quote! {}, quote! {});
        }
        if fam == Family::Event {
            (
                quote! { if let Event::Some(#local_name) = #name; },
                quote! {},
            )
        } else {
            (quote! {}, quote! { #change_name })
        }
    } else {
        let node = incoming.pop().unwrap();
        let fam = node.family();
        let name = node.ident();
		let change_name = change_prefix(&name);
        let local_name = val_prefix(&name);
        let (rest_events, rest_variables) = generate_condition(incoming, family);
        if family == Family::Event && fam == Family::Variable {
            return (rest_events, rest_variables);
        }
        if fam == Family::Event {
            (
                quote! { if let Event::Some(#local_name) = #name; #rest_events },
                rest_variables,
            )
        } else {
            (rest_events, quote! { #change_name || #rest_variables })
        }
    }
}

fn generate_method_args(mut incoming: Vec<&ReNode>) -> TokenStream {
    if incoming.len() == 1 {
        let node = incoming[0];
        let name = node.ident();
        let fam = node.family();
        if fam == Family::Event {
            let local_name = val_prefix(&name);
            quote! { #local_name }
        } else {
            quote! { #name }
        }
    } else {
        let node = incoming.pop().unwrap();
        let name = node.ident();
        let rest = generate_method_args(incoming);
        let fam = node.family();
        if fam == Family::Event {
            let local_name = val_prefix(&name);
            quote! { #local_name, #rest }
        } else {
            quote! { #name, #rest }
        }
    }
}

impl Generate for FoldNode<'_> {
    fn ident(&self) -> Ident {
        format_ident!("fold_{}", self.id())
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
        let (event_condition, _var_condition) = generate_condition(incoming.clone(), family);
        let method_args = generate_method_args(incoming.clone());
        let change_name = change_prefix(&name);
		let temp_name = temp_prefix(&name);
		
        assert!(family == Family::Event);
        ift.update_part = quote! {
            if_chain! {
                #event_condition
                then {
                    let result = Self::#name(state.#name.value.clone(), #method_args);
                    if result != state.#name.value {
                        state.#name.value = result;
                        state.#name.change = true;
                    }
                }
            }
            let #name = &state.#name.value;
            let #change_name = state.#name.change;
        };
        let ty = self.ty();
        ift.state_struct = quote! {
            #name: Variable<#ty>,
        };

        let init_expr = self.initial;
        ift.initialize = quote! {
            let #temp_name = Variable { value: #init_expr, change: true };
			let #name = &#temp_name.value;
        };
		ift.initialize_struct = quote! {
			#name: #temp_name,
		};
		
        ift
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
        let (event_condition, _) = generate_condition(incoming.clone(), family);
        let method_args = generate_method_args(incoming.clone());
		
        assert!(family == Family::Event);
        if self.pin() {
			let ty = &self.ty();
			ift.state_struct = quote! {
				#name: Event<#ty>,
			};
			ift.initialize_struct = quote! {
				#name: Event::None,
			};
			ift.update_part = quote! {
				if_chain! {
					#event_condition
					Self::#name(#method_args);
					then {
						state.#name = Event::Some(#method_args.clone());
					} else {
						state.#name = Event::None;
					}
				}
				let #name = &state.#name;
			};
        } else {
            ift.update_part = quote! {
				let #name = if_chain! {
					#event_condition
					Self::#name(#method_args);
					then {
						Event::Some(#method_args)
					} else {
						Event::None
					}
				};
			};
        }
        ift
    }

    fn ident(&self) -> Ident {
        format_ident!("filter_{}", self.id())
    }
}

impl Generate for ChangedNode {
    fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens {
        let mut ift = InterfaceTokens::default();
		let name = self.ident();
		let family = self.family();
		let ty = self.ty();
		assert!(incoming.len() == 1);
		assert!(family == Family::Variable);
		let incoming_name = incoming[0].ident();
		if self.pin() {
			ift.update_part = quote! {
				state.#name = if change_name {
					Event::Some(#incoming_name.clone())
				} else {
					Event::None
				};
				let #name = &state.#name;
			};
			ift.state_struct = quote! {
				#name: Event<#ty>,
			};
			ift.initialize_struct = quote! {
				#name: Event::None,
			};
		} else {
			ift.update_part = quote! {
				let #name = if change_name {
					Event::Some(#incoming_name)
				} else {
					Event::None
				};
			};
		}
		ift
    }

    fn ident(&self) -> Ident {
		format_ident!("changed_{}", self.id())
    }
}
