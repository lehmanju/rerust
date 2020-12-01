use crate::signal::Signal;
use futures::task::AtomicWaker;
use std;
use std::pin::Pin;
use std::{cell::RefCell, rc::Rc, task::Context};

struct Inner<A>
where
    A: Clone + PartialEq,
{
    waker: AtomicWaker,
    value: A,
    transactions: Vec<(u32, A)>,
}
pub struct Var<A>(Rc<RefCell<Inner<A>>>)
where
    A: Clone + PartialEq;

impl<A: Clone + PartialEq> Var<A> {
    pub fn new(value: A) -> Var<A> {
        Var(Rc::new(RefCell::new(Inner {
            waker: AtomicWaker::new(),
            value,
            transactions: Vec::new(),
        })))
    }

    pub fn set(&self, value: A) {
        let inner = &mut *self.0.borrow_mut();
        if inner.value != value {
            inner.value = value;
            inner.waker.wake();
        }
    }
}

impl<A: Clone + PartialEq> Signal for Var<A> {
    type Item = A;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Self::Item {
        let inner = &mut *self.0.borrow_mut();
        inner.waker.register(cx.waker());
        // lookup transaction list for previous evaluation
        let existing = inner
            .transactions
            .iter() 
            .find(|(id, _)| *id == uuid)
            .map(|(_, val)| (*val).clone());
        // use cached value or else current value
        existing.unwrap_or(inner.value.clone())
    }
    fn transaction_end(mut self: Pin<&mut Self>, uuid: u32) {
        // delete transaction with given uuid
        let inner = &mut *self.0.borrow_mut();
        inner.transactions.retain(|(u, _)| *u != uuid);
    }
    fn value(&self) -> Self::Item {
        let inner = &*self.0.borrow();
        inner.value.clone()
    }
}

impl<A: Clone + PartialEq> PartialEq for Var<A> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<A: Clone + PartialEq> Clone for Var<A> {
    fn clone(&self) -> Self {
        Var(self.0.clone())
    }
}
