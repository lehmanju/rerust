use futures::Future;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll}, sync::atomic::AtomicBool,
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
pub struct ValueStream<A, B, I> {
    #[pin]
    pub(crate) signal: A,
    pub(crate) old: Option<I>,
    pub(crate) f: B,
}

pub struct GraphLock {
    pub(crate) locked: AtomicBool,
    pub(crate) id: u32
}

static LOCKS: u32 = 0;

pub fn new_lock() -> GraphLock {
    LOCKS = LOCKS + 1;
    GraphLock { locked: AtomicBool::new(false), id: LOCKS}
}