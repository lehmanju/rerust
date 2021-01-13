use ::rerust::rerust_gen;
use futures::stream::FusedStream;
use futures::stream::Stream;
use futures::task::Poll;
use pin_utils::pin_mut;
use std::cell::RefCell;
use std::rc::Weak;
use std::sync::mpsc::*;
pub struct State {
    evt_2: Option<i32>,
    fold_8: Option<String>,
    var_1: Option<u32>,
    var_0: Option<i32>,
    group_3: Option<(i32, u32, i32)>,
    map_4: Option<u32>,
    group_5: Option<(i32, u32)>,
    map_6: Option<u32>,
    choice_7: Option<u32>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for State {
    #[inline]
    fn clone(&self) -> State {
        match *self {
            State {
                evt_2: ref __self_0_0,
                fold_8: ref __self_0_1,
                var_1: ref __self_0_2,
                var_0: ref __self_0_3,
                group_3: ref __self_0_4,
                map_4: ref __self_0_5,
                group_5: ref __self_0_6,
                map_6: ref __self_0_7,
                choice_7: ref __self_0_8,
            } => State {
                evt_2: ::core::clone::Clone::clone(&(*__self_0_0)),
                fold_8: ::core::clone::Clone::clone(&(*__self_0_1)),
                var_1: ::core::clone::Clone::clone(&(*__self_0_2)),
                var_0: ::core::clone::Clone::clone(&(*__self_0_3)),
                group_3: ::core::clone::Clone::clone(&(*__self_0_4)),
                map_4: ::core::clone::Clone::clone(&(*__self_0_5)),
                group_5: ::core::clone::Clone::clone(&(*__self_0_6)),
                map_6: ::core::clone::Clone::clone(&(*__self_0_7)),
                choice_7: ::core::clone::Clone::clone(&(*__self_0_8)),
            },
        }
    }
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
    group_3: bool,
    map_4: bool,
    group_5: bool,
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
            group_3: ::core::default::Default::default(),
            map_4: ::core::default::Default::default(),
            group_5: ::core::default::Default::default(),
            map_6: ::core::default::Default::default(),
            choice_7: ::core::default::Default::default(),
            c: ::core::default::Default::default(),
        }
    }
}
pub struct Observers {
    evt: Vec<Weak<RefCell<dyn FnMut(&i32)>>>,
    evt_fold: Vec<Weak<RefCell<dyn FnMut(&String)>>>,
    b: Vec<Weak<RefCell<dyn FnMut(&u32)>>>,
    a: Vec<Weak<RefCell<dyn FnMut(&i32)>>>,
    c: Vec<Weak<RefCell<dyn FnMut(&u32)>>>,
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
    evt_2: (Sender<i32>, Receiver<i32>),
    var_1: (Sender<u32>, Receiver<u32>),
    var_0: (Sender<i32>, Receiver<i32>),
}
pub struct Program {
    state: State,
    observers: Observers,
    sources: Sources,
}
impl Program {
    fn update(state: &mut State, sources: &mut Sources) -> Change {
        let mut change = Change::default();
        let val = Self::evt_2(&mut sources.evt_2.1);
        state.evt_2 = match val {
            Some(v) => {
                change.evt_2 = true;
                Some(v)
            }
            _ => None,
        };
        if change.evt_2 {
            let val = state.evt_2.clone().unwrap();
            let result = Self::fold_8(state.fold_8.clone().unwrap(), val);
            if result != *state.fold_8.as_ref().unwrap() {
                change.fold_8 = true;
                state.fold_8 = Some(result);
            }
        }
        let val = Self::var_1(&mut sources.var_1.1, None);
        state.var_1 = match val {
            Some(v) => {
                change.var_1 = true;
                Some(v)
            }
            None => state.var_1,
        };
        let val = Self::var_0(&mut sources.var_0.1, None);
        state.var_0 = match val {
            Some(v) => {
                change.var_0 = true;
                Some(v)
            }
            None => state.var_0,
        };
        if !state.var_0.is_none() && !state.var_1.is_none() && !state.evt_2.is_none() {
            if change.var_0 || change.var_1 || change.evt_2 {
                change.group_3 = true;
            }
            state.group_3 = Some((
                state.var_0.clone().unwrap(),
                state.var_1.clone().unwrap(),
                state.evt_2.clone().unwrap(),
            ));
        }
        if change.group_3 {
            let val = state.group_3.clone().unwrap();
            let result = Self::map_4(val);
            if state.map_4.is_none() || result != *state.map_4.as_ref().unwrap() {
                change.map_4 = true;
                state.map_4 = Some(result);
            }
        }
        if !state.var_0.is_none() && !state.var_1.is_none() {
            if change.var_0 || change.var_1 {
                change.group_5 = true;
            }
            state.group_5 = Some((state.var_0.clone().unwrap(), state.var_1.clone().unwrap()));
        }
        if change.group_5 {
            let val = state.group_5.clone().unwrap();
            let result = Self::map_6(val);
            if state.map_6.is_none() || result != *state.map_6.as_ref().unwrap() {
                change.map_6 = true;
                state.map_6 = Some(result);
            }
        }
        if change.map_6 {
            state.choice_7 = state.map_6.clone();
            change.choice_7 = true;
        } else if change.map_4 {
            state.choice_7 = state.map_4.clone();
            change.choice_7 = true;
        }
        change
    }
    fn notify(observers: &mut Observers, changes: Change, state: &State) {
        if changes.evt_2 {
            observers.evt.retain(|lst| {
                if let Some(cb) = Weak::upgrade(lst) {
                    if let Some(val) = &state.evt_2 {
                        (&mut *cb.borrow_mut())(val);
                        true
                    } else {
                        {
                            unreachable!()
                        }
                    }
                } else {
                    false
                }
            });
        }
        if changes.fold_8 {
            observers.evt_fold.retain(|lst| {
                if let Some(cb) = Weak::upgrade(lst) {
                    if let Some(val) = &state.fold_8 {
                        (&mut *cb.borrow_mut())(val);
                        true
                    } else {
                        {
                            unreachable!()
                        }
                    }
                } else {
                    false
                }
            });
        }
        if changes.var_1 {
            observers.b.retain(|lst| {
                if let Some(cb) = Weak::upgrade(lst) {
                    if let Some(val) = &state.var_1 {
                        (&mut *cb.borrow_mut())(val);
                        true
                    } else {
                        {
                            unreachable!()
                        }
                    }
                } else {
                    false
                }
            });
        }
        if changes.var_0 {
            observers.a.retain(|lst| {
                if let Some(cb) = Weak::upgrade(lst) {
                    if let Some(val) = &state.var_0 {
                        (&mut *cb.borrow_mut())(val);
                        true
                    } else {
                        {
                            unreachable!()
                        }
                    }
                } else {
                    false
                }
            });
        }
        if changes.choice_7 {
            observers.c.retain(|lst| {
                if let Some(cb) = Weak::upgrade(lst) {
                    if let Some(val) = &state.choice_7 {
                        (&mut *cb.borrow_mut())(val);
                        true
                    } else {
                        {
                            unreachable!()
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
    fn evt_2(stream: &Receiver<i32>) -> Option<i32> {
        let result = stream.try_recv();
        match result {
            Ok(val) => Some(val),
            _ => None,
        }
    }
    #[inline]
    fn sender_evt_2(sources: &Sources) -> Sender<i32> {
        sources.evt_2.0.clone()
    }
    pub fn get_sink_evt(&self) -> Sender<i32> {
        Self::sender_evt_2(&self.sources)
    }
    pub fn observe_evt(&mut self, observer: Weak<RefCell<dyn FnMut(&i32)>>) {
        self.observers.evt.push(observer);
    }
    #[inline]
    fn fold_8(mut string: String, evt: i32) -> String {
        {
            string
        }
    }
    pub fn observe_evt_fold(&mut self, observer: Weak<RefCell<dyn FnMut(&String)>>) {
        self.observers.evt_fold.push(observer);
    }
    #[inline]
    fn var_1(stream: &Receiver<u32>, old_val: Option<u32>) -> Option<u32> {
        let result = stream.try_recv();
        match result {
            Ok(val) => Self::var_1(stream, Some(val)),
            _ => old_val,
        }
    }
    #[inline]
    fn sender_var_1(sources: &Sources) -> Sender<u32> {
        sources.var_1.0.clone()
    }
    pub fn get_sink_b(&self) -> Sender<u32> {
        Self::sender_var_1(&self.sources)
    }
    pub fn observe_b(&mut self, observer: Weak<RefCell<dyn FnMut(&u32)>>) {
        self.observers.b.push(observer);
    }
    #[inline]
    fn var_0(stream: &Receiver<i32>, old_val: Option<i32>) -> Option<i32> {
        let result = stream.try_recv();
        match result {
            Ok(val) => Self::var_0(stream, Some(val)),
            _ => old_val,
        }
    }
    #[inline]
    fn sender_var_0(sources: &Sources) -> Sender<i32> {
        sources.var_0.0.clone()
    }
    pub fn get_sink_a(&self) -> Sender<i32> {
        Self::sender_var_0(&self.sources)
    }
    pub fn observe_a(&mut self, observer: Weak<RefCell<dyn FnMut(&i32)>>) {
        self.observers.a.push(observer);
    }
    #[inline]
    fn map_4((a, b, evt): (i32, u32, i32)) -> u32 {
        
            (a + b as i32 + evt) as u32
        
    }
    #[inline]
    fn map_6((a, b): (i32, u32)) -> u32 {
        {
            a as u32 - b
        }
    }
    pub fn observe_c(&mut self, observer: Weak<RefCell<dyn FnMut(&u32)>>) {
        self.observers.c.push(observer);
    }
}
impl Default for State {
    fn default() -> Self {
        Self {
            evt_2: None,
            fold_8: Some(String::new()),
            var_1: Some(0u32),
            var_0: Some(0i32),
            group_3: None,
            map_4: None,
            group_5: None,
            map_6: None,
            choice_7: None,
        }
    }
}
impl Default for Sources {
    fn default() -> Self {
        Self {
            evt_2: channel(),
            var_1: channel(),
            var_0: channel(),
        }
    }
}
/// struct ReProgram
///
/// impl ReTrait for ReProgram
fn main() {}
