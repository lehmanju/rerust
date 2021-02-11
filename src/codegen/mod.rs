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
        let incoming = &get_incoming_idents(graph, nodeidx);
        let weight = graph.node_weight(nodeidx).expect("expect valid node index");
        tks_change.extend(weight.gen_change());
        let tks_if = weight.gen_source(&incoming);
        tks_card_structs.extend(tks_if.card_struct);
        tks_slots.extend(tks_if.slot_part);
        tks_sink_fn.extend(tks_if.sink_fn);
        tks_input_fn.extend(tks_if.input_fn);
        tks_take_all.extend(tks_if.take_all);
        tks_slot_check.extend(tks_if.check_input);
        tks_slot_init.extend(tks_if.slot_init);
        let (state_members, state_default) = weight.gen_state();
        tks_state.extend(state_members);
        tks_input_struct.extend(tks_if.input_struct_part);
        tks_default_state.extend(state_default);
        tks_function.extend(weight.gen_function(incoming));
        let (update, update_return) = weight.gen_update(incoming);
        tks_update.extend(update);
        tks_update_return.extend(update_return);
        tks_notify.extend(weight.gen_notify(incoming));
        tks_observers.extend(weight.gen_observer());
        tks_initial_input.extend(weight.gen_initial_input());
    }
    quote! {
        use std::rc::Rc;
        use std::rc::Weak;
        use std::cell::RefCell;
        use std::sync::mpsc::*;
        use std::mem;

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

#[derive(Default)]
pub struct InterfaceTokens {
    input_struct_part: TokenStream,
    card_struct: TokenStream,
    slot_part: TokenStream,
    sink_fn: TokenStream,
    input_fn: TokenStream,
    check_input: TokenStream,
    take_all: TokenStream,
    slot_init: TokenStream,
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
    fn gen_source(&self, incoming: &Vec<Ident>) -> InterfaceTokens {
        InterfaceTokens::default()
    }
    fn gen_observer(&self) -> TokenStream {
        TokenStream::new()
    }
    fn gen_notify(&self, _incoming: &Vec<Ident>) -> TokenStream {
        TokenStream::new()
    }
    fn ident(&self) -> Ident;
    fn gen_initial_input(&self) -> TokenStream {
        TokenStream::new()
    }
}

impl Generate for NameNode<'_> {
    fn gen_function(&self, incoming: &Vec<Ident>) -> TokenStream {
        let ident = format_ident!("observe_{}", self.ident());
        let name = self.ident();
        let ty = &self.ty;
        quote! {
            pub fn #ident(&mut self, observer: Weak<RefCell<dyn FnMut(&#ty)>>) {
                self.observers.#name.push(observer);
            }
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

    fn gen_change(&self) -> TokenStream {
        TokenStream::new()
    }

    fn gen_source(&self, incoming: &Vec<Ident>) -> InterfaceTokens {
        let name = self.ident();
        let ty = &self.ty;
        let incoming_node = &incoming[0];
        let mut ift = InterfaceTokens::default();
        if incoming_node.to_string().starts_with("var")
            || incoming_node.to_string().starts_with("evt")
        {
            let input_fn_name = format_ident!("set_{}", name);
            ift.input_fn = quote! {
                pub fn #input_fn_name(&mut self, value: #ty) {
                    self.#incoming_node = Some(value);
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
                && (self.slots.#name.is_some() || input.#incoming_node.is_none())
            };
        }
        ift
    }
}
