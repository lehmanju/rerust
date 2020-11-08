use crate::signal::Signal;
use futures::task::AtomicWaker;
use std;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::RwLock;
use std::task::{Context, Poll};

struct Inner<A>
where
    A: Copy + PartialEq,
{
    waker: AtomicWaker,
    value: RwLock<A>,
}
#[derive(Clone)]
pub struct Var<A>(Arc<Inner<A>>)
where
    A: Copy + PartialEq;

impl<A: Copy + PartialEq> Var<A> {
    pub fn new(value: A) -> Var<A> {
        Var(Arc::new(Inner {
            waker: AtomicWaker::new(),
            value: RwLock::new(value),
        }))
    }

    pub fn set(&self, value: A) {
        if *self.0.value.read().unwrap() != value {
            *self.0.value.write().unwrap() = value;
            self.0.waker.wake();
        }
    }
}

impl<A: Copy + PartialEq> Signal for Var<A> {
    type Item = A;
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.0.waker.register(cx.waker());
        let val = self.0.value.read().unwrap().clone();
        Poll::Ready(Some(val))
    }
}
