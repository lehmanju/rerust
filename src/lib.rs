pub mod var;

pub mod signal {
    use crate::var::Var;
    use futures::{Future, Stream, pin_mut};
    use pin_project::pin_project;
    use std::{
        pin::Pin,
        sync::{Arc, RwLock},
        task::{Context, Poll},
    };
    use uuid::Uuid;
    use pin_utils::unsafe_pinned;

    /// Signal trait modeling a changing value over time
    ///
    /// This can be seen as a Stream of values that never ends.
    #[must_use = "Signals do nothing unless polled"]
    pub trait Signal : Clone + PartialEq {
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

    impl<A, B, C> PartialEq for CombinedMap<A, B, C> {
        fn eq(&self, other: &Self) -> bool {
        self.signal_a == other.signal_a && self.signal_b == other.signal_b && Arc::ptr_eq(self.callback, other.callback)
    }
    }

    impl<A,B,C> Clone for CombinedMap<A, B, C> where A: Signal, B: Signal{
        fn clone(&self) -> Self {
            CombinedMap {signal_a: self.signal_a.clone(), signal_b: self.signal_b.clone(), callback: self.callback.clone()}
        }
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

    #[pin_project]
    #[derive(Clone, PartialEq)]
    pub struct Constant<A>
    where
        A: Future,
    {
        #[pin]
        pub(crate) future: A, //TODO RwLock + Clone
        pub(crate) value: A::Output,
    }

    impl<A: Future> Signal for Constant<A>
    where
        A::Output: Clone + PartialEq,
    {
        type Item = A::Output;
        fn poll(self: Pin<&mut Self>, cx: &mut Context, _: u32) -> Self::Item {
            let mut this = self.project();

            let fut_poll = this.future.as_mut().poll(cx);
            match fut_poll {
                Poll::Ready(value) => {
                    *this.value = value.clone();
                    value
                }
                Poll::Pending => this.value.clone(),
            }
        }
        fn transaction_end(self: Pin<&mut Self>, _: u32) {
            // nothing to do
        }
        fn value(&self) -> Self::Item {
            self.value.clone()
        }
    }

    pub trait FoldSignalExt {
        type Item;
        fn fold_signal<T, F>(initial: T, f: F)
        where
            F: FnMut(T, Self::Item) -> T,
        {
        }
    }

    impl<T> FoldSignalExt for T
    where
        T: Stream,
    {
        type Item = T::Item;
    }

    #[pin_project]
    struct FoldSignalInner<S, A: Clone + PartialEq, F> {
        #[pin]
        stream: S,
        acc: A,
        transactions: Vec<(u32, A)>,
        f: F,
    }

    impl<S: Stream, A: Clone + PartialEq, F: Fn(A, S::Item) -> A> FoldSignalInner<S, A, F> {

        fn poll(self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> A {
            let existing = self
            .transactions
            .iter()
            .find(|(id, _)| *id == uuid)
            .map(|(_, val)| (*val).clone());

            let mut this = self.project();
            existing.unwrap_or(
            match this.stream.as_mut().poll_next(cx) {
                Poll::Ready(element) => {
                    match element {
                        Some(v) => {
                            *this.acc = (this.f)(this.acc.clone(), v);
                            this.acc.clone()
                        }
                        None => {
                            this.acc.clone()
                        }
                    }                    
                }
                Poll::Pending => {
                    this.acc.clone()
                }
            })
        }

        fn transaction_end(self: Pin<&mut Self>, uuid: u32) {

        }

        fn value(&self) -> A {

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
        fn transaction_end(self: Pin<&mut Self>, uuid: u32) {
            let inner = &mut *self.0.write().unwrap();
            inner.transactions.retain(|(u, _)| *u != uuid);
        }
        fn value(&self) -> Self::Item {
            todo!()
        }
    }

    impl<S, A: Clone + PartialEq, F> Clone for FoldSignal<S, A, F> {
        fn clone(&self) -> Self {
            FoldSignal(self.0.clone())
        }
    }

    impl<S, A: Clone + PartialEq, F> PartialEq for FoldSignal<S, A, F> {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(self.0, other.0)
        }
        
    }

    // from future to signal
    pub trait Convert {}

    impl<T, A> Convert for T where T: Future<Output = A> {}
}
