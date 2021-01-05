use ::rerust::rerust_gen;

rerust_gen! {
    let a = Var::<i32>(0i32);
    let b = Var::<u32>(0u32);
    let evt = Evt::<i32>();
    let c = (a,b,evt).map(|(a, b, evt) : (i32, u32, i32)| -> u32 { (a + b as i32 + evt) as u32 }) || (a,b).map(|(a, b): (i32, i32)| -> u32 { (a - b) as u32 });
    let evt_fold = evt.fold(String::new(), |mut string: String, evt: i32| -> String { string });
}

pub mod rerust {
    use std::rc::Weak;
    use futures::task::Poll;
    use futures::stream::FusedStream;
    pub struct State {
        fold_8: String,
        var_1: u32,
        var_0: i32,
        map_4: Option<u32>,
        map_6: Option<u32>,
    }
    pub struct Change {
        evt_2: bool,
        evt: bool,
        fold_8: bool,
        evt_fold: bool,
        var_1: bool,
        b: bool,
        var_0: bool,
        a: bool,
        map_4: bool,
        map_6: bool,
        choice_7: bool,
        c: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Change {
        #[inline]
        fn default() -> Change {
            Change {
                evt_2: ::core::default::Default::default(),
                evt: ::core::default::Default::default(),
                fold_8: ::core::default::Default::default(),
                evt_fold: ::core::default::Default::default(),
                var_1: ::core::default::Default::default(),
                b: ::core::default::Default::default(),
                var_0: ::core::default::Default::default(),
                a: ::core::default::Default::default(),
                map_4: ::core::default::Default::default(),
                map_6: ::core::default::Default::default(),
                choice_7: ::core::default::Default::default(),
                c: ::core::default::Default::default(),
            }
        }
    }
    pub struct Observers {
        evt: Vec<Weak<FnMut(&i32)>>,
        evt_fold: Vec<Weak<FnMut(&String)>>,
        b: Vec<Weak<FnMut(&u32)>>,
        a: Vec<Weak<FnMut(&i32)>>,
        c: Vec<Weak<FnMut(&u32)>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Observers {
        #[inline]
        fn default() -> Observers {
            Observers {
                evt: ::core::default::Default::default(),
                evt_fold: ::core::default::Default::default(),
                b: ::core::default::Default::default(),
                a: ::core::default::Default::default(),
                c: ::core::default::Default::default(),
            }
        }
    }
    pub struct Sources {
        evt_2: Box<dyn FusedStream<Item = i32>>,
        var_1: Box<dyn FusedStream<Item = u32>>,
        var_0: Box<dyn FusedStream<Item = i32>>,
    }
    pub struct Program {
        state: State,
        observers: Observers,
        sources: Sources,
    }
    impl Program {
        fn update(state: State, sources: &mut Sources) -> (State, Change) {
            let mut change = Change::default();
            let evt_2 = Self::evt_2(&mut sources.evt_2);
            let fold_8 = if evt_2.is_some() {
                let val = evt_2.unwrap();
                let result = Self::fold_8(state.fold_8.clone(), val);
                if result != state.fold_8 {
                    change.fold_8 = true;
                    Some(result)
                } else {
                    None
                }
            } else {
                None
            };
            let var_1 = Self::var_1(&mut sources.var_1);
            let var_0 = Self::var_0(&mut sources.var_0);
            let node_group_3 = if var_0.is_some() || var_1.is_some() || evt_2.is_some() {
                Some((var_0.unwrap(), var_1.unwrap(), evt_2.unwrap()))
            } else {
                None
            };
            let map_4 = if node_group_3.is_some() {
                let val = node_group_3.unwrap();
                let result = Self::map_4(val);
                if result != state.map_4 {
                    change.map_4 = true;
                    Some(result)
                } else {
                    None
                }
            } else {
                None
            };
            let node_group_5 = if var_0.is_some() || var_1.is_some() {
                Some((var_0.unwrap(), var_1.unwrap()))
            } else {
                None
            };
            let map_6 = if node_group_5.is_some() {
                let val = node_group_5.unwrap();
                let result = Self::map_6(val);
                if result != state.map_6 {
                    change.map_6 = true;
                    Some(result)
                } else {
                    None
                }
            } else {
                None
            };
            let choice_7 = if let Some(val) = map_6 {
                Some(val)
            } else if let Some(val) = map_4 {
                Some(val)
            } else {
                None
            };
            (
                State {
                    fold_8: fold_8.unwrap_or(state.fold_8),
                    var_1: var_1.unwrap_or(state.var_1),
                    var_0: var_0.unwrap_or(state.var_0),
                    map_4: map_4.or(state.map_4),
                    map_6: map_6.or(state.map_6),
                },
                change,
            )
        }
        fn notify(&mut self, changes: Change, state: &State) {
            let mut observers = &self.observers;
            if changes.evt_2 {
                observers.evt.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        (&mut *cb.borrow_mut())(&state.evt_2);
                        true
                    } else {
                        false
                    }
                });
            }
            if changes.fold_8 {
                observers.evt_fold.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        (&mut *cb.borrow_mut())(&state.fold_8);
                        true
                    } else {
                        false
                    }
                });
            }
            if changes.var_1 {
                observers.b.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        (&mut *cb.borrow_mut())(&state.var_1);
                        true
                    } else {
                        false
                    }
                });
            }
            if changes.var_0 {
                observers.a.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        (&mut *cb.borrow_mut())(&state.var_0);
                        true
                    } else {
                        false
                    }
                });
            }
            if changes.choice_7 {
                observers.c.retain(|lst| {
                    if let Some(cb) = Weak::upgrade(lst) {
                        (&mut *cb.borrow_mut())(&state.choice_7);
                        true
                    } else {
                        false
                    }
                });
            }
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
        #[inline]
        fn evt_2(stream: &mut impl FusedStream<Item = i32>) -> Option<i32> {
            if !stream.is_terminated() {
                if let Poll::Ready(val) = stream.poll_next() {
                    Some(val)
                }
            }
            None
        }
        #[inline]
        fn fold_8(mut string: String, evt: i32) -> String {
            {
                string
            }
        }
        #[inline]
        fn var_1(stream: &mut impl FusedStream<Item = u32>) -> Option<u32> {
            if !stream.is_terminated() {
                if let Poll::Ready(val) = stream.poll_next() {
                    Some(val)
                }
            }
            None
        }
        #[inline]
        fn var_0(stream: &mut impl FusedStream<Item = i32>) -> Option<i32> {
            if !stream.is_terminated() {
                if let Poll::Ready(val) = stream.poll_next() {
                    Some(val)
                }
            }
            None
        }
        #[inline]
        fn map_4((a, b, evt): (i32, u32, i32)) -> u32 {
            {
                (a + b as i32 + evt) as u32
            }
        }
        #[inline]
        fn map_6((a, b): (i32, i32)) -> u32 {
            {
                (a - b) as u32
            }
        }
    }
}

/// struct ReProgram
///
/// impl ReTrait for ReProgram

fn main() {}
