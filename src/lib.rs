pub mod var;

mod signal {
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };

    /// Signal trait modeling a changing value over time
    ///
    /// This can be seen as a Stream of values that never ends.
    #[must_use = "Signals do nothing unless polled"]
    pub trait Signal {
        type Item;

        /// Poll the underlying future
        ///
        /// # Return value:
        ///
        /// - `Poll::Pending` indicates that the Signal has not changed
        ///
        /// - `Poll::Ready(Some(val))` is returend if there is a change detected and the Signal has to be reevaluated.
        ///
        /// Unlike Stream a Signal nevert terminates, so there is **no** `Poll::Ready(None)`.
        fn poll_change(
            self: Pin<&mut Self>,
            cx: &mut Context,
            uuid: u32,
        ) -> Poll<Option<Self::Item>>;

        fn transaction_end(self: Pin<&mut Self>, uuid: u32);
    }

    // Copied from Future in the Rust stdlib
    impl<'a, A> Signal for &'a mut A
    where
        A: ?Sized + Signal + Unpin,
    {
        type Item = A::Item;

        #[inline]
        fn poll_change(
            mut self: Pin<&mut Self>,
            cx: &mut Context,
            uuid: u32,
        ) -> Poll<Option<Self::Item>> {
            A::poll_change(Pin::new(&mut **self), cx, uuid)
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
        fn poll_change(
            mut self: Pin<&mut Self>,
            cx: &mut Context,
            uuid: u32,
        ) -> Poll<Option<Self::Item>> {
            A::poll_change(Pin::new(&mut *self), cx, uuid)
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
        fn poll_change(
            self: Pin<&mut Self>,
            cx: &mut Context,
            uuid: u32,
        ) -> Poll<Option<Self::Item>> {
            Pin::get_mut(self).as_mut().poll_change(cx, uuid)
        }
        fn transaction_end(self: Pin<&mut Self>, uuid: u32) {
            Pin::get_mut(self).as_mut().transaction_end(uuid);
        }
    }

    // copy SignalExt

    pub struct CombinedMap {}
}
