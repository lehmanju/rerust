use crate::signal::Signal;
use futures::{task::AtomicWaker, Stream};
use pin_project::pin_project;
use std::{
    pin::Pin,
    sync::{Arc, RwLock},
    task::{Context, Poll},
};

pub trait FoldSignalExt {
    type Item;
    fn fold_signal<T, F>(self, initial: T, f: F) -> FoldSignal<Self, T, F>
    where
        T: Clone + PartialEq,
        F: Fn(T, Self::Item) -> T,
        Self: Sized;
}

impl<T> FoldSignalExt for T
where
    T: Stream,
{
    type Item = T::Item;
    fn fold_signal<A, F>(self, initial: A, f: F) -> FoldSignal<Self, A, F>
    where
        A: Clone + PartialEq,
        F: Fn(A, Self::Item) -> A,
        Self: Sized,
    {
        FoldSignal(Arc::new(RwLock::new(Box::pin(FoldSignalInner {
            stream: self,
            acc: initial,
            transactions: Vec::new(),
            f,
            waker: AtomicWaker::new(),
        }))))
    }
}

#[pin_project]
struct FoldSignalInner<S, A: Clone + PartialEq, F> {
    #[pin]
    pub(crate) stream: S,
    pub(crate) acc: A,
    pub(crate) transactions: Vec<(u32, A)>,
    pub(crate) f: F,
    pub(crate) waker: AtomicWaker,
}

impl<S: Stream, A: Clone + PartialEq, F: Fn(A, S::Item) -> A> FoldSignalInner<S, A, F> {
    fn poll(self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> A {
        self.waker.register(cx.waker());
        let existing = self
            .transactions
            .iter()
            .find(|(id, _)| *id == uuid)
            .map(|(_, val)| (*val).clone());

        let mut this = self.project();
        existing.unwrap_or(match this.stream.as_mut().poll_next(cx) {
            Poll::Ready(element) => match element {
                Some(v) => {
                    *this.acc = (this.f)(this.acc.clone(), v);
                    this.acc.clone()
                }
                None => this.acc.clone(),
            },
            Poll::Pending => this.acc.clone(),
        })
    }

    fn transaction_end(self: Pin<&mut Self>, uuid: u32) {
        let this = self.project();
        this.transactions.retain(|(u, _)| *u != uuid);
    }

    fn value(&self) -> A {
        self.acc.clone()
    }
}

pub struct FoldSignal<S, A: Clone + PartialEq, F>(Arc<RwLock<Pin<Box<FoldSignalInner<S, A, F>>>>>);

impl<S, A, F> Signal for FoldSignal<S, A, F>
where
    S: Stream,
    F: Fn(A, S::Item) -> A,
    A: PartialEq + Clone,
{
    type Item = A;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Self::Item {
        let inner = &mut *self.0.write().unwrap();
        inner.as_mut().poll(cx, uuid)
    }
    fn transaction_end(mut self: Pin<&mut Self>, uuid: u32) {
        let inner = &mut *self.0.write().unwrap();
        inner.as_mut().transaction_end(uuid);
    }
    fn value(&self) -> Self::Item {
        let inner = &*self.0.read().unwrap();
        inner.value()
    }
}

impl<S, A: Clone + PartialEq, F> Clone for FoldSignal<S, A, F> {
    fn clone(&self) -> Self {
        FoldSignal(self.0.clone())
    }
}

impl<S, A: Clone + PartialEq, F> PartialEq for FoldSignal<S, A, F> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
