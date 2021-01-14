mod generated {
    use futures::stream::FusedStream;
    use futures::stream::Stream;
    use futures::task::Poll;
    use pin_utils::pin_mut;
    use rerust::rerust_gen;
    use std::cell::RefCell;
    use std::rc::Weak;
    use std::sync::mpsc::*;
    pub struct State {
        evt_1: Option<String>,
        var_0: Option<String>,
        group_2: Option<(String, String)>,
        map_3: Option<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for State {
        #[inline]
        fn clone(&self) -> State {
            match *self {
                State {
                    evt_1: ref __self_0_0,
                    var_0: ref __self_0_1,
                    group_2: ref __self_0_2,
                    map_3: ref __self_0_3,
                } => State {
                    evt_1: ::core::clone::Clone::clone(&(*__self_0_0)),
                    var_0: ::core::clone::Clone::clone(&(*__self_0_1)),
                    group_2: ::core::clone::Clone::clone(&(*__self_0_2)),
                    map_3: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    pub struct Change {
        evt_1: bool,
        text: bool,
        var_0: bool,
        name: bool,
        group_2: bool,
        map_3: bool,
        message: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Change {
        #[inline]
        fn default() -> Change {
            Change {
                evt_1: ::core::default::Default::default(),
                text: ::core::default::Default::default(),
                var_0: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                group_2: ::core::default::Default::default(),
                map_3: ::core::default::Default::default(),
                message: ::core::default::Default::default(),
            }
        }
    }
    pub struct Observers {
        text: Vec<Weak<RefCell<dyn FnMut(&String)>>>,
        name: Vec<Weak<RefCell<dyn FnMut(&String)>>>,
        message: Vec<Weak<RefCell<dyn FnMut(&String)>>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Observers {
        #[inline]
        fn default() -> Observers {
            Observers {
                text: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                message: ::core::default::Default::default(),
            }
        }
    }
    pub struct Sources {
        evt_1: (Sender<String>, Receiver<String>),
        var_0: (Sender<String>, Receiver<String>),
    }
    pub struct Program {
        state: State,
        observers: Observers,
        sources: Sources,
    }
    impl Program {
        fn update(state: &mut State, sources: &mut Sources) -> Change {
            let mut change = Change::default();
            let val = Self::evt_1(&mut sources.evt_1.1);
            state.evt_1 = match val {
                Some(v) => {
                    change.evt_1 = true;
                    Some(v)
                }
                _ => None,
            };
            let val = Self::var_0(&mut sources.var_0.1, None);
            state.var_0 = match val {
                Some(v) => {
                    change.var_0 = true;
                    Some(v)
                }
                None => state.var_0,
            };
            if !state.evt_1.is_none() && !state.var_0.is_none() {
                if change.evt_1 || change.var_0 {
                    change.group_2 = true;
                }
                state.group_2 = Some((state.evt_1.clone().unwrap(), state.var_0.clone().unwrap()));
            }
            if change.group_2 {
                let val = state.group_2.clone().unwrap();
                let result = Self::map_3(val);
                if state.map_3.is_none() || result != *state.map_3.as_ref().unwrap() {
                    change.map_3 = true;
                    state.map_3 = Some(result);
                }
            }
            change
        }
        fn notify(observers: &mut Observers, changes: Change, state: &State) {
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
        }
        pub fn run(&mut self) {
            let Program {
                state,
                observers,
                sources,
            } = self;
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
        #[inline]
        fn evt_1(stream: &Receiver<String>) -> Option<String> {
            let result = stream.try_recv();
            match result {
                Ok(val) => Some(val),
                _ => None,
            }
        }
        #[inline]
        fn sender_evt_1(sources: &Sources) -> Sender<String> {
            sources.evt_1.0.clone()
        }
        pub fn get_sink_text(&self) -> Sender<String> {
            Self::sender_evt_1(&self.sources)
        }
        pub fn observe_text(&mut self, observer: Weak<RefCell<dyn FnMut(&String)>>) {
            self.observers.text.push(observer);
        }
        #[inline]
        fn var_0(stream: &Receiver<String>, old_val: Option<String>) -> Option<String> {
            let result = stream.try_recv();
            match result {
                Ok(val) => Self::var_0(stream, Some(val)),
                _ => old_val,
            }
        }
        #[inline]
        fn sender_var_0(sources: &Sources) -> Sender<String> {
            sources.var_0.0.clone()
        }
        pub fn get_sink_name(&self) -> Sender<String> {
            Self::sender_var_0(&self.sources)
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
    }
    impl Default for State {
        fn default() -> Self {
            Self {
                evt_1: None,
                var_0: Some(String::new()),
                group_2: None,
                map_3: None,
            }
        }
    }
    impl Default for Sources {
        fn default() -> Self {
            Self {
                evt_1: channel(),
                var_0: channel(),
            }
        }
    }
}
/// struct ReProgram
///
/// impl ReTrait for ReProgram
fn main() {}
