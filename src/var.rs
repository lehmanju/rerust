use std;
use std::fmt;
use std::marker::Unpin;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
// TODO use parking_lot ?
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};
// TODO use parking_lot ?
use crate::signal::Signal;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll, Waker};

struct VarState<A> {
    value: RwLock<
}

pub struct Var<A>
where
    A: Copy + PartialEq,
{
    value: A,
    
}

pub struct VarSignal<A>
where A : Copy + PartialEq
{
    receivers: Vec<Weak<VarReceiver>>,
}

pub struct VarReceiver {
    waker: Mutex<Option<Waker>>,
}

impl<A: Copy + PartialEq> Var<A> {
    pub fn new(value: A) -> Var<A> {
        todo!()
    }

    pub fn set(&mut self, value: A) {
        if value != self.value {
            self.value = value;
            self.receivers.retain(|receiver| {
                if let Some(receiver) = receiver.upgrade() {
                    let mut lock = receiver.waker.lock().unwrap();
                    if let Some(waker) = lock.take() {
                        drop(lock);
                        waker.wake();
                    }
                    true
                } else {
                    false
                }
            })
        }
    }
}

impl<A: Copy + PartialEq> Signal for Var<A> {
    type Item = A;
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
