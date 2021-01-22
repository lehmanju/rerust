use std::{cell::RefCell, rc::Rc, sync::mpsc::Sender};
mod generated {
    use rerust::rerust_gen;
    use std::rc::Weak;
    use std::cell::RefCell;
    use std::sync::mpsc::*;
    use std::mem;
    pub struct State {
        var_6: Option<usize>,
        var_5: Option<Vec<String>>,
        evt_1: Option<String>,
        var_0: Option<String>,
        group_2: Option<(String, String)>,
        map_3: Option<String>,
        fold_4: Option<Vec<String>>,
        group_7: Option<(Vec<String>, Vec<String>)>,
        map_8: Option<Vec<Vec<String>>>,
        group_9: Option<(Vec<Vec<String>>, usize)>,
        map_10: Option<Vec<String>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for State {
        #[inline]
        fn clone(&self) -> State {
            match *self {
                State {
                    var_6: ref __self_0_0,
                    var_5: ref __self_0_1,
                    evt_1: ref __self_0_2,
                    var_0: ref __self_0_3,
                    group_2: ref __self_0_4,
                    map_3: ref __self_0_5,
                    fold_4: ref __self_0_6,
                    group_7: ref __self_0_7,
                    map_8: ref __self_0_8,
                    group_9: ref __self_0_9,
                    map_10: ref __self_0_10,
                } => State {
                    var_6: ::core::clone::Clone::clone(&(*__self_0_0)),
                    var_5: ::core::clone::Clone::clone(&(*__self_0_1)),
                    evt_1: ::core::clone::Clone::clone(&(*__self_0_2)),
                    var_0: ::core::clone::Clone::clone(&(*__self_0_3)),
                    group_2: ::core::clone::Clone::clone(&(*__self_0_4)),
                    map_3: ::core::clone::Clone::clone(&(*__self_0_5)),
                    fold_4: ::core::clone::Clone::clone(&(*__self_0_6)),
                    group_7: ::core::clone::Clone::clone(&(*__self_0_7)),
                    map_8: ::core::clone::Clone::clone(&(*__self_0_8)),
                    group_9: ::core::clone::Clone::clone(&(*__self_0_9)),
                    map_10: ::core::clone::Clone::clone(&(*__self_0_10)),
                },
            }
        }
    }
    pub struct Change {
        var_6: bool,
        index: bool,
        var_5: bool,
        room2: bool,
        evt_1: bool,
        text: bool,
        var_0: bool,
        name: bool,
        group_2: bool,
        map_3: bool,
        message: bool,
        fold_4: bool,
        room1: bool,
        group_7: bool,
        map_8: bool,
        room_list: bool,
        group_9: bool,
        map_10: bool,
        selected_room: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Change {
        #[inline]
        fn default() -> Change {
            Change {
                var_6: ::core::default::Default::default(),
                index: ::core::default::Default::default(),
                var_5: ::core::default::Default::default(),
                room2: ::core::default::Default::default(),
                evt_1: ::core::default::Default::default(),
                text: ::core::default::Default::default(),
                var_0: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                group_2: ::core::default::Default::default(),
                map_3: ::core::default::Default::default(),
                message: ::core::default::Default::default(),
                fold_4: ::core::default::Default::default(),
                room1: ::core::default::Default::default(),
                group_7: ::core::default::Default::default(),
                map_8: ::core::default::Default::default(),
                room_list: ::core::default::Default::default(),
                group_9: ::core::default::Default::default(),
                map_10: ::core::default::Default::default(),
                selected_room: ::core::default::Default::default(),
            }
        }
    }
    pub struct Observers {
        index: Vec<Weak<RefCell<dyn FnMut(&usize)>>>,
        room2: Vec<Weak<RefCell<dyn FnMut(&Vec<String>)>>>,
        text: Vec<Weak<RefCell<dyn FnMut(&String)>>>,
        name: Vec<Weak<RefCell<dyn FnMut(&String)>>>,
        message: Vec<Weak<RefCell<dyn FnMut(&String)>>>,
        room1: Vec<Weak<RefCell<dyn FnMut(&Vec<String>)>>>,
        room_list: Vec<Weak<RefCell<dyn FnMut(&Vec<Vec<String>>)>>>,
        selected_room: Vec<Weak<RefCell<dyn FnMut(&Vec<String>)>>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Observers {
        #[inline]
        fn default() -> Observers {
            Observers {
                index: ::core::default::Default::default(),
                room2: ::core::default::Default::default(),
                text: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                message: ::core::default::Default::default(),
                room1: ::core::default::Default::default(),
                room_list: ::core::default::Default::default(),
                selected_room: ::core::default::Default::default(),
            }
        }
    }
    pub struct Program {
        state: State,
        observers: Observers,
        receiver: Receiver<Input>,
        sink: Sink,
    }
    pub struct Input {
        var_6: Option<usize>,
        var_5: Option<Vec<String>>,
        evt_1: Option<String>,
        var_0: Option<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Input {
        #[inline]
        fn default() -> Input {
            Input {
                var_6: ::core::default::Default::default(),
                var_5: ::core::default::Default::default(),
                evt_1: ::core::default::Default::default(),
                var_0: ::core::default::Default::default(),
            }
        }
    }
    impl Input {
        pub fn set_index(&mut self, value: usize) {
            self.var_6 = Some(value);
        }
        pub fn set_room2(&mut self, value: Vec<String>) {
            self.var_5 = Some(value);
        }
        pub fn set_text(&mut self, value: String) {
            self.evt_1 = Some(value);
        }
        pub fn set_name(&mut self, value: String) {
            self.var_0 = Some(value);
        }
    }
    struct Phantom {}
    struct Slots {
        index: Option<Cardindex>,
        room2: Option<Cardroom2>,
        text: Option<Cardtext>,
        name: Option<Cardname>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Slots {
        #[inline]
        fn default() -> Slots {
            Slots {
                index: ::core::default::Default::default(),
                room2: ::core::default::Default::default(),
                text: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
            }
        }
    }
    pub struct Sink {
        slots: Slots,
        channel_sender: Sender<Input>,
    }
    impl Clone for Sink {
        fn clone(&self) -> Self {
            Self {
                slots: Slots::default(),
                channel_sender: self.channel_sender.clone(),
            }
        }
    }
    impl Sink {
        pub fn push_index(&mut self, card: Cardindex) {
            self.slots.index = Some(card);
        }
        pub fn pull_index(&mut self) -> Option<Cardindex> {
            self.slots.index.take()
        }
        pub fn send_index(&mut self, value: usize) {
            let mut input = Input::default();
            input.set_index(value);
            self.send(input);
        }
        pub fn push_room2(&mut self, card: Cardroom2) {
            self.slots.room2 = Some(card);
        }
        pub fn pull_room2(&mut self) -> Option<Cardroom2> {
            self.slots.room2.take()
        }
        pub fn send_room2(&mut self, value: Vec<String>) {
            let mut input = Input::default();
            input.set_room2(value);
            self.send(input);
        }
        pub fn push_text(&mut self, card: Cardtext) {
            self.slots.text = Some(card);
        }
        pub fn pull_text(&mut self) -> Option<Cardtext> {
            self.slots.text.take()
        }
        pub fn send_text(&mut self, value: String) {
            let mut input = Input::default();
            input.set_text(value);
            self.send(input);
        }
        pub fn push_name(&mut self, card: Cardname) {
            self.slots.name = Some(card);
        }
        pub fn pull_name(&mut self) -> Option<Cardname> {
            self.slots.name.take()
        }
        pub fn send_name(&mut self, value: String) {
            let mut input = Input::default();
            input.set_name(value);
            self.send(input);
        }
        pub fn take_all(&mut self, other: &mut Self) {
            if other.slots.index.is_some() {
                mem::swap(&mut self.slots.index, &mut other.slots.index);
            }
            if other.slots.room2.is_some() {
                mem::swap(&mut self.slots.room2, &mut other.slots.room2);
            }
            if other.slots.text.is_some() {
                mem::swap(&mut self.slots.text, &mut other.slots.text);
            }
            if other.slots.name.is_some() {
                mem::swap(&mut self.slots.name, &mut other.slots.name);
            }
        }
        pub fn send(&mut self, input: Input) {
            if (false
                || self.slots.index.is_some()
                || input.var_6.is_none()
                || self.slots.room2.is_some()
                || input.var_5.is_none()
                || self.slots.text.is_some()
                || input.evt_1.is_none()
                || self.slots.name.is_some()
                || input.var_0.is_none())
            {
                self.channel_sender.send(input);
            } else {
                {
                    ::std::rt::begin_panic("Slot empty for one input")
                };
            }
        }
        pub fn new(sender: Sender<Input>) -> Self {
            Self {
                slots: Slots::default(),
                channel_sender: sender,
            }
        }
    }
    impl Program {
        fn update(state: &mut State, receiver: &mut Receiver<Input>) -> Change {
            let mut change = Change::default();
            let result = receiver.try_recv();
            if let Ok(inputs) = result {
                if inputs.var_6.is_some() {
                    state.var_6 = inputs.var_6;
                    change.var_6 = true;
                }
                if inputs.var_5.is_some() {
                    state.var_5 = inputs.var_5;
                    change.var_5 = true;
                }
                if inputs.evt_1.is_some() {
                    state.evt_1 = inputs.evt_1;
                    change.evt_1 = true;
                }
                if inputs.var_0.is_some() {
                    state.var_0 = inputs.var_0;
                    change.var_0 = true;
                }
                if !state.evt_1.is_none() && !state.var_0.is_none() {
                    if change.evt_1 || change.var_0 {
                        change.group_2 = true;
                    }
                    state.group_2 =
                        Some((state.evt_1.clone().unwrap(), state.var_0.clone().unwrap()));
                }
                if change.group_2 {
                    let val = state.group_2.clone().unwrap();
                    let result = Self::map_3(val);
                    if state.map_3.is_none() || result != *state.map_3.as_ref().unwrap() {
                        change.map_3 = true;
                        state.map_3 = Some(result);
                    }
                }
                if change.map_3 {
                    let val = state.map_3.clone().unwrap();
                    let result = Self::fold_4(state.fold_4.clone().unwrap(), val);
                    if result != *state.fold_4.as_ref().unwrap() {
                        change.fold_4 = true;
                        state.fold_4 = Some(result);
                    }
                }
                if !state.fold_4.is_none() && !state.var_5.is_none() {
                    if change.fold_4 || change.var_5 {
                        change.group_7 = true;
                    }
                    state.group_7 =
                        Some((state.fold_4.clone().unwrap(), state.var_5.clone().unwrap()));
                }
                if change.group_7 {
                    let val = state.group_7.clone().unwrap();
                    let result = Self::map_8(val);
                    if state.map_8.is_none() || result != *state.map_8.as_ref().unwrap() {
                        change.map_8 = true;
                        state.map_8 = Some(result);
                    }
                }
                if !state.map_8.is_none() && !state.var_6.is_none() {
                    if change.map_8 || change.var_6 {
                        change.group_9 = true;
                    }
                    state.group_9 =
                        Some((state.map_8.clone().unwrap(), state.var_6.clone().unwrap()));
                }
                if change.group_9 {
                    let val = state.group_9.clone().unwrap();
                    let result = Self::map_10(val);
                    if state.map_10.is_none() || result != *state.map_10.as_ref().unwrap() {
                        change.map_10 = true;
                        state.map_10 = Some(result);
                    }
                }
            }
            change
        }
        fn notify(observers: &mut Observers, changes: Change, state: &State) {
            if changes.var_6 {
                observers.index.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        if let Some(val) = &state.var_6 {
                            (&mut *cb.borrow_mut())(val);
                            true
                        } else {
                            {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                    } else {
                        false
                    }
                });
            }
            if changes.var_5 {
                observers.room2.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        if let Some(val) = &state.var_5 {
                            (&mut *cb.borrow_mut())(val);
                            true
                        } else {
                            {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                    } else {
                        false
                    }
                });
            }
            if changes.evt_1 {
                observers.text.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        if let Some(val) = &state.evt_1 {
                            (&mut *cb.borrow_mut())(val);
                            true
                        } else {
                            {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                    } else {
                        false
                    }
                });
            }
            if changes.var_0 {
                observers.name.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        if let Some(val) = &state.var_0 {
                            (&mut *cb.borrow_mut())(val);
                            true
                        } else {
                            {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                    } else {
                        false
                    }
                });
            }
            if changes.map_3 {
                observers.message.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        if let Some(val) = &state.map_3 {
                            (&mut *cb.borrow_mut())(val);
                            true
                        } else {
                            {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                    } else {
                        false
                    }
                });
            }
            if changes.fold_4 {
                observers.room1.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        if let Some(val) = &state.fold_4 {
                            (&mut *cb.borrow_mut())(val);
                            true
                        } else {
                            {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                    } else {
                        false
                    }
                });
            }
            if changes.map_8 {
                observers.room_list.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        if let Some(val) = &state.map_8 {
                            (&mut *cb.borrow_mut())(val);
                            true
                        } else {
                            {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                    } else {
                        false
                    }
                });
            }
            if changes.map_10 {
                observers.selected_room.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        if let Some(val) = &state.map_10 {
                            (&mut *cb.borrow_mut())(val);
                            true
                        } else {
                            {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                    } else {
                        false
                    }
                });
            }
        }
        pub fn run(&mut self) {
            let Program {
                state,
                observers,
                receiver,
                sink,
            } = self;
            let changes = Self::update(state, receiver);
            Self::notify(observers, changes, state);
        }
        pub fn new() -> Self {
            let (send, recv) = channel();
            Self {
                state: State::default(),
                observers: Observers::default(),
                receiver: recv,
                sink: Sink::new(send),
            }
        }
        pub fn sink(&mut self) -> &mut Sink {
            &mut self.sink
        }
        pub fn observe_index(&mut self, observer: Weak<RefCell<dyn FnMut(&usize)>>) {
            self.observers.index.push(observer);
        }
        pub fn observe_room2(&mut self, observer: Weak<RefCell<dyn FnMut(&Vec<String>)>>) {
            self.observers.room2.push(observer);
        }
        pub fn observe_text(&mut self, observer: Weak<RefCell<dyn FnMut(&String)>>) {
            self.observers.text.push(observer);
        }
        pub fn observe_name(&mut self, observer: Weak<RefCell<dyn FnMut(&String)>>) {
            self.observers.name.push(observer);
        }
        #[inline]
        fn map_3((t, n): (String, String)) -> String {
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", ": "],
                    &match (&n, &t) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            }
        }
        pub fn observe_message(&mut self, observer: Weak<RefCell<dyn FnMut(&String)>>) {
            self.observers.message.push(observer);
        }
        #[inline]
        fn fold_4(mut vec: Vec<String>, msg: String) -> Vec<String> {
            vec.push(msg);
            vec
        }
        pub fn observe_room1(&mut self, observer: Weak<RefCell<dyn FnMut(&Vec<String>)>>) {
            self.observers.room1.push(observer);
        }
        #[inline]
        fn map_8((room1, room2): (Vec<String>, Vec<String>)) -> Vec<Vec<String>> {
            <[_]>::into_vec(box [room1, room2])
        }
        pub fn observe_room_list(&mut self, observer: Weak<RefCell<dyn FnMut(&Vec<Vec<String>>)>>) {
            self.observers.room_list.push(observer);
        }
        #[inline]
        fn map_10((room_list, index): (Vec<Vec<String>>, usize)) -> Vec<String> {
            room_list[index].clone()
        }
        pub fn observe_selected_room(&mut self, observer: Weak<RefCell<dyn FnMut(&Vec<String>)>>) {
            self.observers.selected_room.push(observer);
        }
    }
    impl Default for State {
        fn default() -> Self {
            Self {
                var_6: Some(0),
                var_5: Some(<[_]>::into_vec(box [String::from(
                    "Me: a constant message",
                )])),
                evt_1: None,
                var_0: Some(String::new()),
                group_2: None,
                map_3: None,
                fold_4: Some(Vec::new()),
                group_7: None,
                map_8: None,
                group_9: None,
                map_10: None,
            }
        }
    }
}

fn main() {
    
}