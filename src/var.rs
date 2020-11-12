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
    value: RwLock<Option<A>>,
    transactions: RwLock<Vec<(u32, A)>>,
}
#[derive(Clone)]
pub struct Var<A>(Arc<Inner<A>>)
where
    A: Clone + PartialEq;

unsafe impl<A> Send for Var<A> where A: Clone + PartialEq {}
unsafe impl<A> Sync for Var<A> where A: Clone + PartialEq {}

impl<A: Clone + PartialEq> Var<A> {
    pub fn new(value: A) -> Var<A> {
        Var(Arc::new(Inner {
            waker: AtomicWaker::new(),
            value: RwLock::new(Some(value)),
            transactions: RwLock::new(Vec::new()),
        }))
    }

    pub fn set(&self, value: A) {
        let v = Some(value);
        if *self.0.value.read().unwrap() != v {
            *self.0.value.write().unwrap() = v;
            self.0.waker.wake();
        }
    }
}

impl<A: Clone + PartialEq> Signal for Var<A> {
    type Item = A;
    fn poll(self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Poll<Self::Item> {
        self.0.waker.register(cx.waker());
        let existing = self
            .0
            .transactions
            .read()
            .unwrap()
            .iter()
            .find(|(id, _)| *id == uuid)
            .map(|(_, val)| (*val).clone());
        let val = existing.or(self.0.value.read().unwrap().clone());
        match val {
            Some(v) => Poll::Ready(v),
            None => Poll::Pending,
        }
    }
    fn transaction_end(self: Pin<&mut Self>, uuid: u32) {
        self.0
            .transactions
            .write()
            .unwrap()
            .retain(|(u, _)| *u != uuid);
    }
}
