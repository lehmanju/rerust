use futures::Future;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use uuid::Uuid;

/// Signal trait modeling a changing value over time
///
/// This can be seen as a Stream of values that never ends.
#[must_use = "Signals do nothing unless polled"]
pub trait Signal: Clone + PartialEq {
    type Item;

    /// Poll the signal
    ///
    /// # Return value:
    ///
    /// - `Poll::Pending` indicates that the Signal value is not yet ready
    ///
    /// - `Poll::Ready(val)` is returend if the value is ready
    ///
    /// `uuid` corresponds to the current transaction running. If a Signal is evaluated more than once it will encounter the same `uuid`. This indicates that it is evaluated in the same transaction meaning it should return the same value.
    fn poll(self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Self::Item;

    /// Indicate that a transaction has ended.
    ///
    /// Release all objects that are no longer needed.
    fn transaction_end(self: Pin<&mut Self>, uuid: u32);

    fn value(&self) -> Self::Item;
}

/// Wraps a Signal in a Future.
///
/// This is essentially the standard way of interacting with Signal values. The provided closure will be called on each Signal change. Reevaluation will only occur then.
#[pin_project]
pub struct FutureWrapper<A, B>
where
    A: Signal,
{
    #[pin]
    pub(crate) signal: A,
    pub(crate) old: Option<A::Item>,
    pub(crate) f: B,
}

impl<A, B> Future for FutureWrapper<A, B>
where
    A: Signal,
    B: FnMut(A::Item),
    A::Item: Clone + PartialEq,
{
    type Output = A::Item;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        let uuid = Uuid::new_v4().as_u128() as u32;
        let v = this.signal.as_mut().poll(cx, uuid);

        match this.old.as_mut() {
            Some(o) => {
                if *o != v {
                    (this.f)(v.clone());
                    *o = v;
                }
            }
            None => {
                (this.f)(v.clone());
                this.old.replace(v);
            }
        }
        this.signal.as_mut().transaction_end(uuid);
        Poll::Pending
    }
}
