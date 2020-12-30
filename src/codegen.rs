use enum_dispatch::enum_dispatch;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::analysis::{
    ChoiceNode, EvtNode, FilterNode, FoldNode, GroupNode, NameNode, ReEdge, VarNode,
};
use crate::analysis::{MapNode, ReNode};
use petgraph::{graph::NodeIndex, visit::Topo, Graph};

pub struct Codegen<'gen> {
    graph: &'gen Graph<ReNode<'gen>, ReEdge>,
}

impl<'gen> Codegen<'gen> {
    pub fn new() -> Self {
        todo!()
    }
    pub fn generate(&self) -> TokenStream {
        let mut topo_visitor = Topo::new(self.graph);
        let mut tks_change = TokenStream::new();
        let mut tks_state = TokenStream::new();
        let mut tks_function = TokenStream::new();
        let mut tks_update = TokenStream::new();
        let mut tks_update_return = TokenStream::new();
        let mut tks_observers = TokenStream::new();
        let mut tks_sources = TokenStream::new();
        let mut tks_notify = TokenStream::new();
        let mut tks_default_state = TokenStream::new();
        while let Some(nodeidx) = topo_visitor.next(self.graph) {
            let incoming = &self.get_incoming_idents(nodeidx);
            let weight = self
                .graph
                .node_weight(nodeidx)
                .expect("expect valid node index");
            tks_change.extend(weight.gen_change());
            tks_sources.extend(weight.gen_source());
            let (state_members, state_default) = weight.gen_state();
            tks_state.extend(state_members);
            tks_default_state.extend(state_default);
            tks_function.extend(weight.gen_function(incoming));
            let (update, update_return) = weight.gen_update(incoming);
            tks_update.extend(update);
            tks_update_return.extend(update_return);
            tks_notify.extend(weight.gen_notify());
            tks_observers.extend(weight.gen_observer(incoming));
        }
        quote! {
            pub mod rerust {
                pub struct State {
                    #tks_state
                }
                #[derive(Default)]
                pub struct Change {
                    #tks_change
                }
                #[derive(Default)]
                pub struct Observers {
                    #tks_observers
                }
                #[derive(Default)]
                pub struct Sources {
                    #tks_sources
                }
                pub struct Program {
                    state: State,
                    observers: Observers,
                    sources: Sources,
                }

                pub impl Program {
                    #tks_function

                    fn update(state: State) -> (State, Change) {
                        let mut change = Change::default();
                        #tks_update
                        (State { #tks_update_return }, change)
                    }

                    fn notify(&self, changes: Change, state: &State) {
                        #tks_notify
                    }

                    pub fn run(&mut self) {
                        let (new_state, changes) = update(self.state);
                        self.state = new_state;
                        self.notify(changes, &self.state);
                    }

                    pub fn new() -> Self {
                        Self {
                            state: State::default(),
                            observers: Observers::default(),
                            sources: Sources::default(),
                        }
                    }
                }

                impl Default for State {
                    fn default() -> Self {
                        Self {
                            #tks_default_state
                        }
                    }
                }
            }
        }
    }
    fn get_incoming_idents(&self, idx: NodeIndex) -> Vec<Ident> {
        let incoming_nodes = self.graph.neighbors_directed(idx, petgraph::Incoming);
        let mut idents = Vec::new();
        for node_idx in incoming_nodes {
            let node = self
                .graph
                .node_weight(node_idx)
                .expect("invalid node index");
            idents.push(node.ident());
        }
        idents
    }
}

#[enum_dispatch]
pub trait Generate {
    fn gen_function(&self, incoming: &Vec<Ident>) -> TokenStream;
    fn gen_update(&self, incoming: &Vec<Ident>) -> (TokenStream, TokenStream);
    fn gen_state(&self) -> (TokenStream, TokenStream);
    fn gen_change(&self) -> TokenStream {
        let ident = self.ident();
        quote! {
            #ident: bool,
        }
    }
    fn gen_source(&self) -> TokenStream {
        TokenStream::new()
    }
    fn gen_observer(&self, incoming: &Vec<Ident>) -> TokenStream {
        TokenStream::new()
    }
    fn gen_notify(&self) -> TokenStream {
        TokenStream::new()
    }
    fn ident(&self) -> Ident;
    fn ident_val(&self) -> Ident;
}

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

    fn gen_update(&self, incoming: &Vec<Ident>) -> TokenStream {
        let incoming_node = &incoming[0];
        let name = self.ident_val();
        let func_name = self.ident();
        quote! {
            let #name = if #incoming_node.is_some() {
                let val = #incoming_node.unwrap();
                let result = #func_name (val);
                if result != state.#func_name {
                    change.#func_name = true;
                    Some(result)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    fn gen_state(&self) -> TokenStream {
        let ident = self.ident();
        let ty = self.ty;
        quote! {
            #ident: #ty,
        }
    }

    fn ident_val(&self) -> Ident {
        format_ident!("node_map_{}", self.id)
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

    fn gen_update(&self, incoming: &Vec<Ident>) -> TokenStream {
        let incoming_node = &incoming[0];
        let name = self.ident_val();
        let func_name = self.ident();
        quote! {
            let #name = if #incoming_node.is_some() {
                let val = #incoming_node.unwrap();
                let result = #func_name (state.#func_name.clone(), val);
                if result != state.#func_name {
                    change.#func_name = true;
                    Some(result)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    fn gen_state(&self) -> TokenStream {
        let ident = self.ident();
        let ty = self.ty;
        quote! {
            #ident: #ty,
        }
    }

    fn ident_val(&self) -> Ident {
        format_ident!("node_fold_{}", self.id)
    }

    fn ident(&self) -> Ident {
        format_ident!("fold_{}", self.id)
    }
}

impl Generate for GroupNode {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        TokenStream::new()
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> TokenStream {
        let name = self.ident_val();
        let (a, b) = (&incoming[0], &incoming[1]);
        quote! {
            let #name = if let Some(val) = #a {
                Some(val)
            } else if let Some(val) = #b {
                Some(b)
            } else {
                None
            }
        }
    }

    fn gen_state(&self) -> TokenStream {
        TokenStream::new()
    }

    fn ident(&self) -> Ident {
        format_ident!("node_group_{}", self.id)
    }

    fn ident_val(&self) -> Ident {
        format_ident!("group_{}", self.id)
    }

    fn gen_change(&self) -> TokenStream {
        TokenStream::new()
    }
}

impl Generate for FilterNode<'_> {
    fn gen_function(&self, incoming: &Vec<Ident>) -> TokenStream {
        let name = self.ident_val();
        let incoming_node = &incoming[0];
        let func_name = self.ident();
        quote! {
            let #name = if #incoming_node.is_some() {
                let val = #incoming_node.unwrap();
                if #func_name(&val) {
                    change.#func_name = true;
                    Some(val)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> TokenStream {
        let name = self.ident_val();
        let incoming_node = &incoming[0];
        let func_name = self.ident();
        quote! {
            let #name = if #incoming_node.is_some() {
                let val = #incoming_node.unwrap();
                if #func_name(&val) {
                    change.#func_name = true;
                    Some(val)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    fn gen_state(&self) -> TokenStream {
        let ident = self.ident();
        let ty = &self.ty;
        quote! {
            #ident: #ty,
        }
    }

    fn ident_val(&self) -> Ident {
        format_ident!("node_filter_{}", self.id)
    }

    fn ident(&self) -> Ident {
        format_ident!("filter_{}", self.id)
    }
}

impl Generate for ChoiceNode {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        TokenStream::new()
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> TokenStream {
        let name = self.ident_val();
        let node = self.ident();
        let (a, b) = (&incoming[0], &incoming[1]);
        quote! {
            let #name = if let Some(val) = #a {
                Some(val)
            } else if let Some(val) = #b {
                Some(b)
            } else {
                None
            }
        }
    }

    fn gen_state(&self) -> TokenStream {
        let ident = self.ident();
        let ty = &self.ty;
        quote! {
            #ident: #ty,
        }
    }

    fn ident_val(&self) -> Ident {
        format_ident!("node_choice_{}", self.id)
    }

    fn ident(&self) -> Ident {
        format_ident!("choice_{}", self.id)
    }
}

impl Generate for VarNode<'_> {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        let name = self.ident();
        let ty = self.ty;
        quote! {
            #[inline]
            fn #name(stream: &mut FusedStream<#ty>) -> Option<#ty> {
                if !stream.is_terminated() {
                    if let Poll::Ready(val) = stream.poll_next() {
                        Some(val)
                    }
                }
                None
            }
        }
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> TokenStream {
        let name = self.ident_val();
        let node = self.ident();
        quote! {
            let #name = #name(sources.#node);
        }
    }

    fn gen_state(&self) -> TokenStream {
        let ident = self.ident();
        let ty = &self.ty;
        quote! {
            #ident: #ty,
        }
    }

    fn ident(&self) -> Ident {
        format_ident!("var_{}", self.id)
    }

    fn ident_val(&self) -> Ident {
        format_ident!("node_var_{}", self.id)
    }

    fn gen_source(&self) -> TokenStream {
        let ident = self.ident();
        let ty = &self.ty;
        quote! {
            #ident: FusedStream<#ty>,
        }
    }
}

impl Generate for EvtNode<'_> {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        let name = self.ident();
        let ty = self.ty;
        quote! {
            #[inline]
            fn #name(stream: &mut FusedStream<#ty>) -> Option<#ty> {
                if !stream.is_terminated() {
                    if let Poll::Ready(val) = stream.poll_next() {
                        Some(val)
                    }
                }
                None
            }
        }
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> TokenStream {
        let name = self.ident_val();
        let node = self.ident();
        quote! {
            let #name = #name(sources.#node);
        }
    }

    fn gen_state(&self) -> TokenStream {
        let ident = self.ident();
        let ty = &self.ty;
        quote! {
            #ident: #ty,
        }
    }

    fn ident(&self) -> Ident {
        format_ident!("evt_{}", self.id)
    }

    fn ident_val(&self) -> Ident {
        format_ident!("node_evt_{}", self.id)
    }
}

impl Generate for NameNode<'_> {
    fn gen_function(&self, incoming: &Vec<Ident>) -> TokenStream {
        todo!()
    }

    fn gen_update(&self, incoming: &Vec<Ident>) -> TokenStream {
        todo!()
    }

    fn gen_state(&self) -> TokenStream {
        todo!()
    }

    fn ident(&self) -> Ident {
        todo!()
    }

    fn ident_val(&self) -> Ident {
        todo!()
    }
}
