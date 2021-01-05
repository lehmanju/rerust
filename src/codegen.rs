use enum_dispatch::enum_dispatch;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::analysis::{
    ChoiceNode, EvtNode, FilterNode, FoldNode, GroupNode, NameNode, ReEdge, VarNode,
};
use crate::analysis::{MapNode, ReNode};
use petgraph::{graph::NodeIndex, visit::Topo, Graph};

pub fn generate(graph: &Graph<ReNode, ReEdge>) -> TokenStream {
    let mut topo_visitor = Topo::new(graph);
    let mut tks_change = TokenStream::new();
    let mut tks_state = TokenStream::new();
    let mut tks_function = TokenStream::new();
    let mut tks_update = TokenStream::new();
    let mut tks_update_return = TokenStream::new();
    let mut tks_observers = TokenStream::new();
    let mut tks_sources = TokenStream::new();
    let mut tks_notify = TokenStream::new();
    let mut tks_default_state = TokenStream::new();
    let mut tks_sources_default = TokenStream::new();
    while let Some(nodeidx) = topo_visitor.next(graph) {
        let incoming = &get_incoming_idents(graph, nodeidx);
        let weight = graph.node_weight(nodeidx).expect("expect valid node index");
        tks_change.extend(weight.gen_change());
        let (sources, sources_default) = weight.gen_source();
        tks_sources.extend(sources);
        tks_sources_default.extend(sources_default);
        let (state_members, state_default) = weight.gen_state();
        tks_state.extend(state_members);
        tks_default_state.extend(state_default);
        tks_function.extend(weight.gen_function(incoming));
        let (update, update_return) = weight.gen_update(incoming);
        tks_update.extend(update);
        tks_update_return.extend(update_return);
        tks_notify.extend(weight.gen_notify(incoming));
        tks_observers.extend(weight.gen_observer());
    }
    quote! {
            pub mod rerust {
                use std::rc::Weak;
                use futures::task::Poll;
                use futures::stream::FusedStream;

                #[derive(Clone)]
                enum ReactiveState<T> {
                    Old(T),
                    New(T),
                }

                impl<T> ReactiveState<T> {
                    fn is_none(&self) -> bool {
                        match self {
                            None => true,
                            _ => false,
                        }
                    }
                    fn is_new(&self) -> bool {
                        match self {
                            New(_) => true,
                            _ => false,
                        }
                    }
                    fn get_value(self) -> Option<T> {
                        match self {
                            None => None,
                            Old(val) => Some(val),
                            New(val) => Some(val),
                        }
                    }
                }

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
                pub struct Sources {
                    #tks_sources
                }
                pub struct Program {
                    state: State,
                    observers: Observers,
                    sources: Sources,
                }

                impl Program {
                    fn update(state: State, sources: &mut Sources) -> (State, Change) {
                        let mut change = Change::default();
                        #tks_update
                        (State { #tks_update_return }, change)
                    }

                    fn notify(&mut self, changes: Change, state: &State) {
                        let mut observers = &self.observers;
                        #tks_notify
                    }

                    pub fn run(&mut self) {
                        let (new_state, changes) = Self::update(self.state, &mut self.sources);
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

                    #tks_function
                }
    /*
                impl Default for State {
                    fn default() -> Self {
                        Self {
                            #tks_default_state
                        }
                    }
                }

                impl Default for Sources {
                    fn default() -> Self {
                        Self {
                            #tks_sources_default
                        }
                    }
                }*/
            }
        }
}
fn get_incoming_idents(graph: &Graph<ReNode, ReEdge>, idx: NodeIndex) -> Vec<Ident> {
    let incoming_nodes = graph.neighbors_directed(idx, petgraph::Incoming);
    let mut idents = Vec::new();
    for node_idx in incoming_nodes {
        let node = graph.node_weight(node_idx).expect("invalid node index");
        if let ReNode::Name(_) = node {
            idents.extend(get_incoming_idents(graph, node_idx));
        } else {
            idents.push(node.ident());
        }
    }
    idents
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
    fn gen_source(&self) -> (TokenStream, TokenStream) {
        (TokenStream::new(), TokenStream::new())
    }
    fn gen_observer(&self) -> TokenStream {
        TokenStream::new()
    }
    fn gen_notify(&self, _incoming: &Vec<Ident>) -> TokenStream {
        TokenStream::new()
    }
    fn ident(&self) -> Ident;
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

    fn gen_update(&self, incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let incoming_node = &incoming[0];
        let name = self.ident();
        let func_name = self.ident();
        (
            quote! {
                let #name = if #incoming_node.is_new() {
                    let val = #incoming_node.get_value().unwrap();
                    let result = Self::#func_name(val);
                    if state.#func_name.is_none() || result != state.#func_name.unwrap() {
                        change.#func_name = true;
                        ReactiveState::New(result)
                    } else {
                        ReactiveState::Old(result)
                    }
                } else {
                    match state.#func_name {
                        Some(v) => ReactiveState::Old(v),
                        None => ReactiveState::None,
                    }
                };
            },
            quote! {
                #func_name: #name.get_value(),
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
                let #name = if #incoming_node.is_new() {
                    let val = #incoming_node.get_value().unwrap();
                    let result = Self::#func_name(state.#func_name.clone(), val);
                    if result != state.#func_name {
                        change.#func_name = true;
                        ReactiveState::New(result)
                    } else {
                        ReactiveState::Old(result)
                    }
                } else {
                    ReactiveState::Old(state.#func_name)
                };
            },
            quote! {
                #func_name: #name.get_value().unwrap(),
            },
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = self.ty;
        let expr = self.initial;
        (
            quote! {
                #ident: #ty,
            },
            quote! {
                #ident: #expr,
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
                let #name = if #tks_ifb {
                    if #tks_ifa {
                        ReactiveState::New((#tks_ret))
                    } else
                    {
                        ReactiveState::Old((#tks_ret))
                    }
                } else {
                    None
                };
            },
            TokenStream::new(),
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        (TokenStream::new(), TokenStream::new())
    }

    fn ident(&self) -> Ident {
        format_ident!("node_group_{}", self.id)
    }

    fn gen_change(&self) -> TokenStream {
        TokenStream::new()
    }
}

fn gen_group_update(mut ident: Vec<Ident>) -> (TokenStream, TokenStream, TokenStream) {
    if ident.len() == 1 {
        let elem = &ident[0];
        (
            quote! {
                #elem.is_new()
            },
            quote! {
                !#elem.is_none()
            },
            quote! {
                #elem.get_value().unwrap()
            },
        )
    } else {
        let a = ident.pop().expect("Non empty list");
        let (if_clausea, if_clauseb, ret) = gen_group_update(ident);
        (
            quote! {
                #a.is_new() || #if_clausea
            },
            quote! {
                !#a.is_none() && #if_clauseb
            },
            quote! {#a.get_value().unwrap(), #ret},
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
                let #name = if #incoming_node.is_new() {
                    let val = #incoming_node.unwrap();
                    if Self::#name(&val) {
                        change.#name = true;
                        ReactiveState::New(val)
                    } else {
                        ReactiveState::None
                    }
                } else {
                    ReactiveState::None
                };
            },
            quote! {},
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        (TokenStream::new(), TokenStream::new())
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
                let #name = if let ReactiveState::New(val) = #a {
                    ReactiveState::New(val)
                } else if let ReactiveState::New(val) = #b {
                    ReactiveState::New(val)
                } else {
                    ReactiveState::None
                };
            },
            quote! {},
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        (quote! {}, quote! {})
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
            fn #name(stream: &mut impl FusedStream<Item=#ty>) -> Option<#ty> {
                if !stream.is_terminated() {
                    if let Poll::Ready(val) = stream.poll_next() {
                        Some(val)
                    }
                }
                None
            }
        }
    }

    fn gen_update(&self, _incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let name = self.ident();
        let node = self.ident();
        (
            quote! {
                let #name = Self::#node(&mut sources.#node);
            },
            quote! {
                #node: #name.unwrap_or(state.#name),
            },
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = &self.ty;
        let init = self.initial;
        (
            quote! {
                #ident: #ty,
            },
            quote! {
                #ident: #init,
            },
        )
    }

    fn ident(&self) -> Ident {
        format_ident!("var_{}", self.id)
    }

    fn gen_source(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = &self.ty;
        (
            quote! {
                #ident: Box<dyn FusedStream<Item=#ty>>,
            },
            quote! {
                #ident: futures::stream::empty()
            },
        )
    }
}

impl Generate for EvtNode<'_> {
    fn gen_function(&self, _: &Vec<Ident>) -> TokenStream {
        let name = self.ident();
        let ty = self.ty;
        quote! {
            #[inline]
            fn #name(stream: &mut impl FusedStream<Item=#ty>) -> Option<#ty> {
                if !stream.is_terminated() {
                    if let Poll::Ready(val) = stream.poll_next() {
                        Some(val)
                    }
                }
                None
            }
        }
    }

    fn gen_update(&self, _incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        let name = self.ident();
        let node = self.ident();
        (
            quote! {
                let #name = Self::#node(&mut sources.#node);
            },
            quote! {},
        )
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        (TokenStream::new(), TokenStream::new())
    }

    fn gen_source(&self) -> (TokenStream, TokenStream) {
        let ident = self.ident();
        let ty = self.ty;
        (
            quote! {
                #ident: Box<dyn FusedStream<Item=#ty>>,
            },
            quote! {
                #ident: futures::stream::empty()
            },
        )
    }

    fn ident(&self) -> Ident {
        format_ident!("evt_{}", self.id)
    }
}

impl Generate for NameNode<'_> {
    fn gen_function(&self, incoming: &Vec<Ident>) -> TokenStream {
        quote! {}
    }

    fn gen_notify(&self, incoming: &Vec<Ident>) -> TokenStream {
        let income = &incoming[0];
        let ident = self.ident();
        quote! {
            if changes.#income {
                observers.#ident.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        (&mut *cb.borrow_mut())(&state.#income);
                        true
                    } else {
                        false
                    }
                });
            }
        }
    }

    fn gen_observer(&self) -> TokenStream {
        let name = self.ident();
        let ty = &self.ty;
        quote! {
            #name: Vec<Weak<FnMut(&#ty)>>,
        }
    }

    fn gen_update(&self, _incoming: &Vec<Ident>) -> (TokenStream, TokenStream) {
        (TokenStream::new(), TokenStream::new())
    }

    fn gen_state(&self) -> (TokenStream, TokenStream) {
        (TokenStream::new(), TokenStream::new())
    }

    fn ident(&self) -> Ident {
        self.id.ident.clone()
    }
}
