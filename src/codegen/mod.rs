use enum_dispatch::enum_dispatch;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::analysis::{Family, NameNode, NodeData, ReEdge, ReNode};
use petgraph::{graph::NodeIndex, visit::Topo, Graph};

mod reactives;
mod sources;

pub fn generate(graph: &Graph<ReNode, ReEdge>) -> TokenStream {
    let mut topo_visitor = Topo::new(graph);
    let mut tks_state = TokenStream::new();
    let mut tks_function = TokenStream::new();
    let mut tks_update = TokenStream::new();
    let mut tks_observers = TokenStream::new();
    let mut tks_input_struct = TokenStream::new();
    let mut tks_notify = TokenStream::new();
    let mut tks_card_structs = TokenStream::new();
    let mut tks_slots = TokenStream::new();
    let mut tks_sink_fn = TokenStream::new();
    let mut tks_input_fn = TokenStream::new();
    let mut tks_slot_check = quote! {true};
    let mut tks_take_all = TokenStream::new();
    let mut tks_slot_init = TokenStream::new();
    let mut tks_initialize = TokenStream::new();
    let mut tks_initialize_struct = TokenStream::new();
    let mut tks_observer_init = TokenStream::new();
    while let Some(nodeidx) = topo_visitor.next(graph) {
        let incoming = &get_incoming_weights(graph, nodeidx);
        let weight = graph.node_weight(nodeidx).expect("expect valid node index");
        let tokens = weight.generate_interface(incoming);
        tks_card_structs.extend(tokens.card_struct);
        tks_slots.extend(tokens.slot_part);
        tks_sink_fn.extend(tokens.sink_fn);
        tks_input_fn.extend(tokens.input_fn);
        tks_take_all.extend(tokens.take_all);
        tks_slot_check.extend(tokens.check_input);
        tks_slot_init.extend(tokens.slot_init);
        tks_state.extend(tokens.state_struct);
        tks_input_struct.extend(tokens.input_struct_part);
        tks_function.extend(tokens.functions);
        tks_update.extend(tokens.update_part);
        tks_notify.extend(tokens.notify_part);
        tks_observers.extend(tokens.observer_struct);
        tks_initialize.extend(tokens.initialize);
        tks_initialize_struct.extend(tokens.initialize_struct);
        tks_observer_init.extend(tokens.initialize_observers);
    }
    quote! {
        extern crate alloc;

        use alloc::rc::Rc;
        use alloc::rc::Weak;
        use core::cell::RefCell;
        use if_chain::if_chain;
        use alloc::collections::vec_deque::VecDeque;
        use alloc::vec::Vec;

        #[derive(Clone)]
        struct Variable<T> {
            value: T,
            change: bool,
        }

        #[derive(Clone)]
        enum Event<T> {
            Some(T),
            None,
        }

        #[derive(Clone)]
        pub struct State {
            #tks_state
        }

        #[derive(Default)]
        struct Observers {
            #tks_observers
        }

        pub struct Program {
            state: State,
            observers: Observers,
            buffer: Rc<RefCell<VecDeque<Input>>>,
        }

        #[derive(Default, Clone)]
        pub struct Input {
            #tks_input_struct
        }

        impl Input {
            #tks_input_fn
        }

        impl Default for State {
            fn default() -> Self {
                Program::default_state()
            }
        }

        impl Program {
            pub fn update(state: &mut State, mut inputs: Input) {
                #tks_update
            }

            fn notify(observers: &mut Observers, state: &mut State) {
                #tks_notify
            }

            pub fn default_state() -> State {
                #tks_initialize
                State { #tks_initialize_struct }
            }

            pub fn run(&mut self) {
                let Program {state, observers, buffer} = self;
                let result = buffer.borrow_mut().pop_front();
                match result {
                    Some(inputs) => {
                        Self::update(state, inputs);
                        Self::notify(observers, state);
                    }
                    None => {}
                }
            }

            pub fn init(&mut self) {
                let Program { state, observers, buffer } = self;
                #tks_observer_init
                Self::notify(observers, state);
            }

            pub fn new() -> Self {
                Self {
                    state: State::default(),
                    observers: Observers::default(),
                    buffer: Rc::new(RefCell::new(VecDeque::new())),
                }
            }

            pub fn sink(&self) -> Rc<RefCell<VecDeque<Input>>> {
                self.buffer.clone()
            }

            #tks_function
        }
    }
}
fn get_incoming_weights<'ast>(
    graph: &'ast Graph<ReNode<'ast>, ReEdge>,
    idx: NodeIndex,
) -> Vec<&'ast ReNode<'ast>> {
    let incoming_nodes = graph.neighbors_directed(idx, petgraph::Incoming);
    let mut weights = Vec::new();
    for node_idx in incoming_nodes {
        let node = graph.node_weight(node_idx).expect("invalid node index");
        if let ReNode::Name(_) = node {
            weights.extend(get_incoming_weights(graph, node_idx));
        } else {
            weights.push(node);
        }
    }
    weights
}

#[derive(Default)]
pub struct InterfaceTokens {
    pub input_struct_part: TokenStream,
    pub card_struct: TokenStream,
    pub slot_part: TokenStream,
    pub sink_fn: TokenStream,
    pub input_fn: TokenStream,
    pub check_input: TokenStream,
    pub take_all: TokenStream,
    pub slot_init: TokenStream,
    pub functions: TokenStream,
    pub update_part: TokenStream,
    pub state_struct: TokenStream,
    pub observer_struct: TokenStream,
    pub notify_part: TokenStream,
    pub initialize: TokenStream,
    pub initialize_struct: TokenStream,
    pub initialize_observers: TokenStream,
}

pub fn change_prefix(ident: &Ident) -> Ident {
    format_ident!("change_{}", ident)
}

pub fn temp_prefix(ident: &Ident) -> Ident {
    format_ident!("temp_{}", ident)
}

pub fn val_prefix(ident: &Ident) -> Ident {
    format_ident!("val_{}", ident)
}

#[enum_dispatch]
pub trait Generate {
    fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens;
    fn ident(&self) -> Ident;
}

impl Generate for NameNode<'_> {
    fn ident(&self) -> Ident {
        self.id.ident.clone()
    }

    fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens {
        let name = self.ident();
        let ty = self.ty();
        let ident = self.ident();
        let income = incoming[0].ident();
        let parent_node = incoming[0];
        let pin = self.pin();
        let family = self.family();
        let mut ift = InterfaceTokens::default();

        // save state and generate observers
        if pin {
            ift.observer_struct = quote! {
                #name: Vec<Weak<RefCell<dyn FnMut(&#ty)>>>,
            };
            let observer_ident = format_ident!("observe_{}", ident);
            ift.functions = quote! {
                pub fn #observer_ident(&mut self, observer: Weak<RefCell<dyn FnMut(&#ty)>>) {
                    self.observers.#name.push(observer);
                }
            };

            match family {
                Family::Event => {
                    ift.notify_part = quote! {
                        if let Event::Some(value) = &state.#income {
                            observers.#ident.retain(|lst| {
                                if let Some(cb) = Weak::upgrade(lst) {
                                    (&mut *cb.borrow_mut())(value);
                                    true
                                } else {
                                    false
                                }
                            });
                        }
                        state.#income = Event::None;
                    };
                }
                Family::Variable => {
                    ift.notify_part = quote! {
                        if state.#income.change {
                            observers.#ident.retain(|lst| {
                                if let Some(cb) = Weak::upgrade(lst) {
                                    (&mut *cb.borrow_mut())(&state.#income.value);
                                    true
                                } else {
                                    false
                                }
                            });
                        }
                        state.#income.change = false;
                    };
                }
            }
        }

        // setters for sources
        match parent_node {
            ReNode::Var(_) | ReNode::Evt(_) => {
                let input_fn_name = format_ident!("set_{}", name);
                ift.input_fn = quote! {
                    pub fn #input_fn_name(&mut self, value: #ty) {
                        self.#income = Some(value);
                    }
                };
                let card_name = format_ident!("Card{}", name);
                ift.card_struct = quote! {
                    pub struct #card_name {
                        data: Weak<Phantom>,

                    }
                };
                ift.slot_part = quote! {
                    #name: Option<#card_name>,
                };
                ift.slot_init = quote! {
                    #name: Some(#card_name {data: data.clone()}),
                };
                let push_card = format_ident!("push_{}", name);
                let pull_card = format_ident!("pull_{}", name);
                let send_single = format_ident!("send_{}", name);
                ift.sink_fn = quote! {
                    pub fn #push_card(&mut self, card: #card_name) {
                        let id_self = Rc::downgrade(&self.id);
                        if id_self.ptr_eq(&card.data) {
                            self.slots.#name = Some(card);
                        } else {
                            panic!("Card has incorrect id");
                        }
                    }
                    pub fn #pull_card(&mut self) -> Option<#card_name> {
                        self.slots.#name.take()
                    }
                    pub fn #send_single(&mut self, value: #ty) {
                        let mut input = Input::default();
                        input.#input_fn_name(value);
                        self.send(input);
                    }
                };
                ift.take_all = quote! {
                    if other.slots.#name.is_some() {
                        mem::swap(&mut self.slots.#name, &mut other.slots.#name);
                    }
                };
                ift.check_input = quote! {
                    && (self.slots.#name.is_some() || input.#income.is_none())
                };
            }
            _ => {}
        };
        ift
    }
}
