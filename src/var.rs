use crate::signal::Signal;
use futures::task::AtomicWaker;
use std;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::RwLock;
use std::task::{Context, Poll};

struct Inner<A>
where
    A: Clone + PartialEq,
{
    waker: AtomicWaker,
    value: RwLock<A>,
    transactions: RwLock<Vec<(u32, A)>>,
}
pub struct Var<A>(Arc<Inner<A>>)
where
    A: Clone + PartialEq;

unsafe impl<A> Send for Var<A> where A: Clone + PartialEq {}
unsafe impl<A> Sync for Var<A> where A: Clone + PartialEq {}

impl<A: Clone + PartialEq> Var<A> {
    pub fn new(value: A) -> Var<A> {
        Var(Arc::new(Inner {
            waker: AtomicWaker::new(),
            value: RwLock::new(value),
            transactions: RwLock::new(Vec::new()),
        }))
    }

    pub fn set(&self, value: A) {
        if *self.0.value.read().unwrap() != value {
            *self.0.value.write().unwrap() = value;
            self.0.waker.wake();
        }
    }
}

impl<A: Clone + PartialEq> Signal for Var<A> {
    type Item = A;
    fn poll(self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Self::Item {
        self.0.waker.register(cx.waker());
        // lookup transaction list for previous evaluation
        let existing = self
            .0
            .transactions
            .read()
            .unwrap()
            .iter()
            .find(|(id, _)| *id == uuid)
            .map(|(_, val)| (*val).clone());
        // use cached value or else current value
        existing.unwrap_or(self.0.value.read().unwrap().clone())
    }
    fn transaction_end(self: Pin<&mut Self>, uuid: u32) {
        // delete transaction with given uuid
        self.0
            .transactions
            .write()
            .unwrap()
            .retain(|(u, _)| *u != uuid);
    }
    fn value(&self) -> Self::Item {
        self.0.value.read().unwrap().clone()
    }
}

impl<A: Clone + PartialEq> PartialEq for Var<A> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<A: Clone + PartialEq> Clone for Var<A> {
    fn clone(&self) -> Self {
        Var(self.0.clone())
    }
}
