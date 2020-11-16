pub mod var;

mod signal {
    use futures::{Future, Stream};
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
    pub trait Signal {
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
        fn poll(self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Poll<Self::Item>;

        /// Indicate that a transaction has ended.
        ///
        /// Release all objects that are no longer needed.
        fn transaction_end(self: Pin<&mut Self>, uuid: u32);
    }

    // Copied from Future in the Rust stdlib
    impl<'a, A> Signal for &'a mut A
    where
        A: ?Sized + Signal + Unpin,
    {
        type Item = A::Item;

        #[inline]
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Poll<Self::Item> {
            A::poll(Pin::new(&mut **self), cx, uuid)
        }
        fn transaction_end(mut self: Pin<&mut Self>, uuid: u32) {
            A::transaction_end(Pin::new(&mut **self), uuid);
        }
    }

    // Copied from Future in the Rust stdlib
    impl<A> Signal for Box<A>
    where
        A: ?Sized + Signal + Unpin,
    {
        type Item = A::Item;

        #[inline]
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Poll<Self::Item> {
            A::poll(Pin::new(&mut *self), cx, uuid)
        }
        fn transaction_end(mut self: Pin<&mut Self>, uuid: u32) {
            A::transaction_end(Pin::new(&mut *self), uuid);
        }
    }

    // Copied from Future in the Rust stdlib
    impl<A> Signal for Pin<A>
    where
        A: Unpin + ::std::ops::DerefMut,
        A::Target: Signal,
    {
        type Item = <<A as ::std::ops::Deref>::Target as Signal>::Item;

        #[inline]
        fn poll(self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Poll<Self::Item> {
            Pin::get_mut(self).as_mut().poll(cx, uuid)
        }
        fn transaction_end(self: Pin<&mut Self>, uuid: u32) {
            Pin::get_mut(self).as_mut().transaction_end(uuid);
        }
    }

    /// A combination of two Signals.
    ///
    /// It will poll both Signals and generate a new Signal by applying the internal callback.
    #[pin_project]
    pub struct CombinedMap<A, B, C> {
        #[pin]
        pub(crate) signal_a: A,
        #[pin]
        pub(crate) signal_b: B,
        pub(crate) callback: C,
    }

    impl<A, B, C, I> Signal for CombinedMap<A, B, C>
    where
        A: Signal,
        B: Signal,
        C: Fn(A::Item, B::Item) -> I + 'static,
    {
        type Item = I;
        fn poll(self: Pin<&mut Self>, cx: &mut Context, uuid: u32) -> Poll<Self::Item> {
            let mut this = self.project();

            let a = this.signal_a.as_mut().poll(cx, uuid);
            let b = this.signal_b.as_mut().poll(cx, uuid);

            match a {
                Poll::Ready(val_a) => match b {
                    Poll::Ready(val_b) => Poll::Ready((this.callback)(val_a, val_b)),
                    Poll::Pending => Poll::Pending,
                },
                Poll::Pending => Poll::Pending,
            }
        }
        fn transaction_end(self: Pin<&mut Self>, uuid: u32) {
            let mut this = self.project();

            this.signal_a.as_mut().transaction_end(uuid);
            this.signal_b.as_mut().transaction_end(uuid);
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
            let mut result = Poll::Pending;

            if let Poll::Ready(v) = this.signal.as_mut().poll(cx, uuid) {
                if let Some(o) = this.old.as_mut() {
                    if *o != v {
                        result = Poll::Ready(v.clone());
                        *o = v;
                    }
                } else {
                    result = Poll::Ready(v.clone());
                    this.old.replace(v);
                }
            }
            this.signal.as_mut().transaction_end(uuid);
            return result;
        }
    }

    #[pin_project]
    pub struct Constant<A>
    where
        A: Future,
    {
        #[pin]
        pub(crate) future: A,
        pub(crate) value: Option<A::Output>,
    }

    impl<A: Future> Signal for Constant<A>
    where
        A::Output: Clone + PartialEq,
    {
        type Item = A::Output;
        fn poll(self: Pin<&mut Self>, cx: &mut Context, _: u32) -> Poll<Self::Item> {
            let mut this = self.project();
            match this.value {
                Some(v) => Poll::Ready(v.clone()),
                None => {
                    let fut_poll = this.future.as_mut().poll(cx);
                    match fut_poll {
                        Poll::Ready(value) => {
                            this.value.as_mut().replace(&mut value.clone());
                            Poll::Ready(value)
                        }
                        Poll::Pending => Poll::Pending,
                    }
                }
            }
        }
        fn transaction_end(self: Pin<&mut Self>, _: u32) {
            // nothing to do
        }
    }

    pub trait Convert {}

    impl<T, A> Convert for T where T: Future<Output = A> {}
}
