use crate::signal::Signal;
use pin_project::pin_project;
use std::{pin::Pin, sync::Arc, task::Context};

/// A combination of two Signals.
///
/// It will poll both Signals and generate a new Signal by applying the internal callback.
#[pin_project]
//#[derive(Clone)]
pub struct CombinedMap<A, B, C> {
    #[pin]
    pub(crate) signal_a: A,
    #[pin]
    pub(crate) signal_b: B,
    pub(crate) callback: Arc<C>,
}

impl<A, B, C, I> CombinedMap<A, B, C>
where
    A: Signal,
    B: Signal,
    C: Fn(A::Item, B::Item) -> I + 'static,
    I: Clone + PartialEq,
{
    pub fn new(signal_a: A, signal_b: B, f: C) -> Self {
        Self {
            signal_a,
            signal_b,
            callback: Arc::new(f),
        }
    }
}

impl<A, B, C, I> Signal for CombinedMap<A, B, C>
where
    A: Signal,
    B: Signal,
    C: Fn(A::Item, B::Item) -> I + 'static,
{
    type Item = I;
    fn poll(self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Self::Item {
        let mut this = self.project();

        let a = this.signal_a.as_mut().poll(cx, uuid);
        let b = this.signal_b.as_mut().poll(cx, uuid);

        (this.callback)(a, b)
    }
    fn transaction_end(self: Pin<&mut Self>, uuid: u32) {
        let mut this = self.project();

        this.signal_a.as_mut().transaction_end(uuid);
        this.signal_b.as_mut().transaction_end(uuid);
    }
    fn value(&self) -> Self::Item {
        let a = self.signal_a.value();
        let b = self.signal_b.value();
        (self.callback)(a, b)
    }
}

impl<A, B, C> PartialEq for CombinedMap<A, B, C>
where
    A: Signal,
    B: Signal,
{
    fn eq(&self, other: &Self) -> bool {
        self.signal_a == other.signal_a
            && self.signal_b == other.signal_b
            && Arc::ptr_eq(&self.callback, &other.callback)
    }
}

impl<A, B, C> Clone for CombinedMap<A, B, C>
where
    A: Signal,
    B: Signal,
{
    fn clone(&self) -> Self {
        CombinedMap {
            signal_a: self.signal_a.clone(),
            signal_b: self.signal_b.clone(),
            callback: self.callback.clone(),
        }
    }
}
