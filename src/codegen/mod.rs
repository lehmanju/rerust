use enum_dispatch::enum_dispatch;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::analysis::{
    ChoiceNode, EvtNode, FilterNode, FoldNode, GroupNode, NameNode, ReEdge, VarNode,
};
use crate::analysis::{MapNode, ReNode};
use petgraph::{graph::NodeIndex, visit::Topo, Graph};

mod reactives;
mod sources;

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
        use std::rc::Weak;
        use std::cell::RefCell;
        use std::sync::mpsc::*;

        #[derive(Clone)]
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
            fn update(state: &mut State, sources: &mut Sources) -> Change {
                let mut change = Change::default();
                #tks_update
                change
            }

            fn notify(observers: &mut Observers, changes: Change, state: &State) {
                #tks_notify
            }

            pub fn run(&mut self) {
                let Program {state, observers, sources} = self;
                let changes = Self::update(state, sources);
                Self::notify(observers, changes, state);
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
    fn gen_update_state(&self) -> TokenStream {
        let ident = self.ident();
        let unwrap = self.gen_unwrap();
        quote! {
            #ident: #unwrap,
        }
    }
    fn gen_unwrap(&self) -> TokenStream {
        let name = &self.ident();
        quote! {
            #name
        }
    }
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

impl Generate for NameNode<'_> {
    fn gen_function(&self, incoming: &Vec<Ident>) -> TokenStream {
        let ident = format_ident!("observe_{}", self.ident());
        let sink = format_ident!("get_sink_{}", self.ident());
        let name = self.ident();
        let income = format_ident!("sender_{}", &incoming[0]);
        let ty = &self.ty;
        let common = quote! {
            pub fn #ident(&mut self, observer: Weak<RefCell<dyn FnMut(&#ty)>>) {
                self.observers.#name.push(observer);
            }
        };
        if incoming[0].to_string().starts_with("var") || incoming[0].to_string().starts_with("evt")
        {
            quote! {
                pub fn #sink(&self) -> Sender<#ty> {
                    Self::#income(&self.sources)
                }

                #common
            }
        } else {
            common
        }
    }

    fn gen_notify(&self, incoming: &Vec<Ident>) -> TokenStream {
        let income = &incoming[0];
        let ident = self.ident();
        quote! {
            if changes.#income {
                observers.#ident.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        if let Some(val) = &state.#income {
                            (&mut *cb.borrow_mut())(val);
                            true
                        } else
                        {
                            unreachable!()
                        }
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
            #name: Vec<Weak<RefCell<dyn FnMut(&#ty)>>>,
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
