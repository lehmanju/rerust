use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::analysis::{ChoiceNode, FilterNode, FoldNode, GroupNode, MapNode, ReNode, Family};

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
		let incoming_node = incoming[0]; //only one possible incoming node
		let incoming_family = incoming_node.family();
		let incoming_name = incoming_node.ident();
		let ty = self.ty;
		if incoming_family == Family::Event {
			match incoming_node {
				ReNode::Group(group) => {
					ift.update_part = quote! {
						let #name = #incoming_name.map(Self::#name);
					};				
				},
				_ => {
					ift.update_part = quote! {
						let #name = #incoming_name.as_ref().map(Self::#name);
					};
				}
			}
		} else { //if Family::Variable
			match incoming_node {
				ReNode::Group(_) => {
					ift.update_part = quote! {
						if #incoming_name.change {
							let result = Self::#name(#incoming_name.value);
							if result != state.#name {
								change.#name = true;
								state.#name = result;
							}
						}
					};
				}
				_ => {
					ift.update_part = quote! {
						if change.#incoming_name {
							let result = Self::#name(&state.#incoming_name);
							if result != state.#name {
								change.#name = true;
								state.#name = result;
							}
						}
					};
				}
			}
			ift.state_struct = quote! {
                #name: #ty,
			};
			ift.change_struct = quote! {
				#name: bool,
			};
		}
		ift
	}

    fn ident(&self) -> &Ident {
        &format_ident!("map_{}", self.id)
    }

	fn family(&self) -> Family {
		self.family
	}
}

impl Generate for FoldNode<'_> {
    fn ident(&self) -> &Ident {
        &format_ident!("fold_{}", self.id)
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
		let incoming_node = incoming[0]; //only one possible incoming node
		let incoming_family = incoming_node.family();
		let incoming_name = incoming_node.ident();
		let ty = self.ty;
		if incoming_family == Family::Event {
			match incoming_node {
				ReNode::Group(group) => { //use local variable and copy it
					ift.update_part = quote! {
						if let Some(val) = #incoming_name {
							let result = Self::#name(state.#name.clone(), val);
							if result != state.#name {
								change.#name = true;
								state.#name = result;
							}
						}
					};				
				},
				_ => { //use local variable and reference it
					ift.update_part = quote! {
						if let Some(val) = &#incoming_name {
							let result = Self::name(state.#name.clone(), val);
							if result != state.#name {
								change.#name = true;
								state.#name = result;
							}
						}
					};
				}
			}
		} else { //if Family::Variable
			match incoming_node {
				ReNode::Group(_) => { //use global state and copy it
					ift.update_part = quote! {
						if #incoming_name.change {
							let result = Self::#name(state.#name.clone(), #incoming_name.value);
							if result != state.#name {
								change.#name = true;
								state.#name = result;
							}
						}
					};
				}
				_ => { //use global state and reference it
					ift.update_part = quote! {
						if change.#incoming_name {
							let result = Self::#name(state.#name.clone(), &state.#incoming_name);
							if result != state.#name {
								change.#name = true;
								state.#name = result;
							}
						}
					};
				}
			}
			
		}
		let init_expr = self.initial;
		ift.state_struct = quote! {
            #name: #ty,
		};
		ift.change_struct = quote! {
			#name: bool,
		};
		ift.state_default = quote! {
			#name: #init_expr,
		};
		ift
    }

    fn family(&self) -> Family {
        self.family
    }
}

impl Generate for GroupNode {
	fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens {
		let mut ift = InterfaceTokens::default();
		let family = self.family();
		let name = self.ident();
		let (compose, condition) = gen_group_update(incoming.clone(), family);
		let ty = self.ty;
		if family == Family::Event {
			ift.update_part = quote! {
				let #name = if #condition {
					Some((#compose))
				} else {
					None
				};
			}
		} else { //if Family::Variable
			ift.update_part = quote! {
				let #name = Group { value: (#condition), change: #compose };
			}
		}
		ift
	}

    fn ident(&self) -> &Ident {
        &format_ident!("group_{}", self.id)
    }

	fn family(&self) -> Family {
		self.family
	}
}

fn gen_group_update(mut incoming: Vec<&ReNode>, self_family: Family) -> (TokenStream, TokenStream) {
    if incoming.len() == 1 {
        let elem = &incoming[0];
		let ident = elem.ident();
		let family = elem.family();
		if family == Family::Event {
			match elem {
				ReNode::Group(_) => {
					(quote! { #ident.unwrap() }, quote! { #ident.is_some() })
				},
				_ => {
					(quote! { #ident.as_ref().unwrap() }, quote! { #ident.is_some() })
				},
			}
		} else {
			match elem {
				ReNode::Group(_) => {
					if self_family == Family::Variable {
						(quote! { #ident.value }, quote! { #ident.change })
					} else {
						(quote! { #ident.value }, TokenStream::new())
					}
				},
				_ => {
					if self_family == Family::Variable {
						(quote! { &state.#ident }, quote! { change.#ident })
					} else {
						(quote! { &state.#ident }, TokenStream::new())
					}
				}
			}
		}
    } else {
        let elem = incoming.pop().expect("Non empty list");
		let ident = elem.ident();
		let family = elem.family();
        let (compose, condition) = gen_group_update(incoming, self_family);
		if family == Family::Event {
			match elem {
				ReNode::Group(_) => {
					(quote! { #ident.unwrap(), #compose }, quote! { #ident.is_some() && #condition })
				},
				_ => {
					(quote! { #ident.as_ref().unwrap(), #compose }, quote! { #ident.is_some() && #condition })
				},
			}
		} else {
			match elem {
				ReNode::Group(_) => {
					if self_family == Family::Variable {
						(quote! { #ident.value, #compose }, quote! { #ident.change || #condition })
					} else {
						(quote! { #ident.value, #compose }, condition)
					}					
				},
				_ => {
					if self_family == Family::Variable {
						(quote! { &state.#ident, #compose }, quote! { change.#ident || #condition })
					} else {
						(quote! { &state.#ident, #compose }, condition)
					}
				}
			}
		}
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
		
		let incoming_node = incoming[0]; //only one possible incoming node
		let incoming_family = incoming_node.family();
		let incoming_name = incoming_node.ident();
		let ty = self.ty;
		if incoming_family == Family::Event {
			match incoming_node {
				ReNode::Group(group) => {
					ift.update_part = quote! {
						let #name = #incoming_name.and_then(|val| {
							let choice = Self::#name(val);
							if choice {
								Some(val)
							} else {
								None
							}
						});
					};				
				},
				_ => {
					ift.update_part = quote! {
						let #name = #incoming_name.filter(Self::#name);
					};
				}
			}
		} else { //if Family::Variable
			match incoming_node {
				ReNode::Group(_) => {
					ift.update_part = quote! {
						if #incoming_name.change {
							let choice = Self::#name(#incoming_name.value);
							if choice {
								change.#name = true;
								state.#name = #incoming_name
							let result = Self::#name(#incoming_name.value);
							if result != state.#name {
								change.#name = true;
								state.#name = result;
							}
						}
					};
				}
				_ => {
					ift.update_part = quote! {
						if change.#incoming_name {
							let result = Self::#name(&state.#incoming_name);
							if result != state.#name {
								change.#name = true;
								state.#name = result;
							}
						}
					};
				}
			}
			ift.state_struct = quote! {
                #name: #ty,
			};
			ift.change_struct = quote! {
				#name: bool,
			};
		}
		ift
	}


	fn gen_function(&self, _incoming: &Vec<Ident>) -> TokenStream {
        let func_name = self.ident();
        let expr_inputs = &self.filter_expr.inputs;
        let expr_body = &self.filter_expr.body;
        quote! {
            #[inline]
            fn #func_name(#expr_inputs) -> bool
                #expr_body

        }
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let name = self.ident();
        let incoming_node = &incoming[0];
        (
            quote! {
                if change.#incoming_node {
                    let val = #incoming_node.as_ref().unwrap();
                    if Self::#name(val) {
                        change.#name = true;
                        state.#name = Some(val.clone());
                    }
                } else if state.#incoming_node.is_none() {
                    state.#name = None;
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
                if change.#a {
                    state.#name = state.#a.clone();
                    change.#name = true;
                } else if change.#b {
                    state.#name = state.#b.clone();
                    change.#name = true;
                } else if state.#a.is_none() || state.#b.is_none() {
                    state.#name = None;
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
