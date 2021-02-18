use enum_dispatch::enum_dispatch;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::quote;

use crate::analysis::{
    ChoiceNode, EvtNode, Family, FilterNode, FoldNode, MapNode, NameNode, ReEdge, ReNode, VarNode,
};
use petgraph::{graph::NodeIndex, visit::Topo, Graph};

mod reactives;
mod sources;

pub fn generate(graph: &Graph<ReNode, ReEdge>) -> TokenStream {
    let mut topo_visitor = Topo::new(graph);
    let mut tks_change = TokenStream::new();
    let mut tks_state = TokenStream::new();
    let mut tks_function = TokenStream::new();
    let mut tks_update = TokenStream::new();
    let mut tks_observers = TokenStream::new();
    let mut tks_input_struct = TokenStream::new();
    let mut tks_notify = TokenStream::new();
    let mut tks_default_state = TokenStream::new();
    let mut tks_card_structs = TokenStream::new();
    let mut tks_slots = TokenStream::new();
    let mut tks_sink_fn = TokenStream::new();
    let mut tks_input_fn = TokenStream::new();
    let mut tks_slot_check = quote! {true};
    let mut tks_take_all = TokenStream::new();
    let mut tks_slot_init = TokenStream::new();
    let mut tks_initial_input = TokenStream::new();
    while let Some(nodeidx) = topo_visitor.next(graph) {
        let incoming = &get_incoming_weights(graph, nodeidx);
        let weight = graph.node_weight(nodeidx).expect("expect valid node index");
        let tokens = weight.generate_interface(incoming);
        tks_change.extend(tokens.change_struct);
        tks_card_structs.extend(tokens.card_struct);
        tks_slots.extend(tokens.slot_part);
        tks_sink_fn.extend(tokens.sink_fn);
        tks_input_fn.extend(tokens.input_fn);
        tks_take_all.extend(tokens.take_all);
        tks_slot_check.extend(tokens.check_input);
        tks_slot_init.extend(tokens.slot_init);
        tks_state.extend(tokens.state_struct);
        tks_input_struct.extend(tokens.input_struct_part);
        tks_default_state.extend(tokens.state_default);
        tks_function.extend(tokens.functions);
        tks_update.extend(tokens.update_part);
        tks_notify.extend(tokens.notify_part);
        tks_observers.extend(tokens.observer_struct);
        tks_initial_input.extend(tokens.initial_input);
    }
    quote! {
        use std::rc::Rc;
        use std::rc::Weak;
        use std::cell::RefCell;
        use std::sync::mpsc::*;
        use std::mem;

        struct Group<T> {
            value: T,
            change: bool,
        }

        #[derive(Clone)]
        pub struct State {
            #tks_state
        }
        #[derive(Default)]
        pub struct Change {
            #tks_change
        }
        #[derive(Default)]
        struct Observers {
            #tks_observers
        }
        pub struct Program {
            state: State,
            observers: Observers,
            receiver: Receiver<Input>,
            sink: Sink,
        }

        #[derive(Default, Clone)]
        pub struct Input {
            #tks_input_struct
        }

        impl Input {
            pub fn initial() -> Self {
                Self {
                    #tks_initial_input
                }
            }
            #tks_input_fn
        }

        struct Phantom {}

        #tks_card_structs

        #[derive(Default)]
        struct Slots {
            #tks_slots
        }
        impl Slots {
            fn new(data: Weak<Phantom>) -> Self {
                Self {
                #tks_slot_init
                }
            }
        }

        pub struct Sink {
            slots: Slots,
            channel_sender: Sender<Input>,
            id: Rc<Phantom>,
        }

        impl Clone for Sink {
            fn clone(&self) -> Self {
                Self { slots: Slots::default(), channel_sender: self.channel_sender.clone(), id: self.id.clone()}
            }
        }

        impl Sink {
            #tks_sink_fn

            pub fn take_all(&mut self, other: &mut Self) {
                #tks_take_all
            }

            pub fn send(&mut self, input: Input) {
                if #tks_slot_check
                {
                    self.channel_sender.send(input).unwrap();
                } else
                {
                    panic!("Slot empty or from another program instance");
                }
            }
            fn new(sender: Sender<Input>) -> Self {
                let id = Rc::new(Phantom {});
                Self {
                    slots: Slots::new(Rc::downgrade(&id)),
                    channel_sender: sender,
                    id,
                }
            }
        }

        impl Program {
            pub fn update(state: &mut State, mut inputs: Input, change: &mut Change) {
                #tks_update
            }

            fn notify(observers: &mut Observers, changes: Change, state: &State) {
                #tks_notify
            }

            pub fn run(&mut self) {
                let Program {state, observers, receiver, sink} = self;
                let result = receiver.try_recv();
                match result {
                    Ok(inputs) => {
                        let mut changes = Change::default();
                        Self::update(state, inputs, &mut changes);
                        Self::notify(observers, changes, state);
                    }
                    Err(recv_error) => {
                        println!("Queue error: {:?}", recv_error);
                    }
                }
            }

            pub fn new() -> Self {
                let (send,recv) = channel();
                let input = Input::initial();
                send.send(input).unwrap();
                Self {
                    state: State::default(),
                    observers: Observers::default(),
                    receiver: recv,
                    sink: Sink::new(send),
                }
            }

            pub fn sink(&mut self) -> Sink {
                let mut new_sink = self.sink.clone();
                new_sink.take_all(&mut self.sink);
                new_sink
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
    }
}
fn get_incoming_weights<'ast>(
    graph: &'ast Graph<ReNode<'ast>, ReEdge>,
    idx: NodeIndex,
) -> Vec<&'ast ReNode> {
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
    pub initial_input: TokenStream,
    pub functions: TokenStream,
    pub update_part: TokenStream,
    pub state_struct: TokenStream,
    pub change_struct: TokenStream,
    pub observer_struct: TokenStream,
    pub notify_part: TokenStream,
    pub state_default: TokenStream,
}

#[enum_dispatch]
pub trait Generate {
    fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens;
    fn ident(&self) -> Ident;
    fn family(&self) -> Family;
}

impl Generate for NameNode<'_> {
    fn family(&self) -> Family {
        self.family
    }

    fn ident(&self) -> Ident {
        self.id.ident.clone()
    }

    fn generate_interface(&self, incoming: &Vec<&ReNode>) -> InterfaceTokens {
        let name = self.ident();
        let ty = &self.ty;
        let ident = self.ident();
        let income = incoming[0].ident();
        let parent_node = incoming[0];
        let mut ift = InterfaceTokens::default();
        ift.observer_struct = quote! {
            #name: Vec<Weak<RefCell<dyn FnMut(&#ty)>>>,
        };
        let observer_ident = format_ident!("observer_{}", ident);
        ift.functions = quote! {
            pub fn #observer_ident(&mut self, observer: Weak<RefCell<dyn FnMut(&#ty)>>) {
                self.observers.#name.push(observer);
            }
        };
        ift.notify_part = quote! {
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
        };
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
