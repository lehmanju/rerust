use crate::signal::{Signal, Transaction};
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
    fn poll(self: Pin<&mut Self>, cx: &mut Context, uuid: Transaction) -> Self::Item {
        self.0.waker.register(cx.waker());
        match uuid {
            Transaction::None => self.0.value.read().unwrap().clone(),
            Transaction::Id(uid) => {
                // lookup transaction list for previous evaluation
                let existing = self
                    .0
                    .transactions
                    .read()
                    .unwrap()
                    .iter()
                    .find(|(id, _)| *id == uid)
                    .map(|(_, val)| (*val).clone());
                // use cached value or else current value
                existing.unwrap_or(self.0.value.read().unwrap().clone())
            }
        }
    }
    fn transaction_end(self: Pin<&mut Self>, uuid: Transaction) {
        if let Transaction::Id(id) = uuid {
            // delete transaction with given uuid
            self.0
                .transactions
                .write()
                .unwrap()
                .retain(|(u, _)| *u != id);
        }
    }
}
