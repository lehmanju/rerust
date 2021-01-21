use futures::{executor, future, pin_mut, stream, stream::Peekable, FutureExt, StreamExt};
use futures_signals::{map_ref, signal::{Mutable, SignalExt}};
use std::pin::Pin;
use future::ready;
use executor::ThreadPool;

fn main() {
    let x_mut = Mutable::new(1);
    let x = x_mut.signal();
    let x_clone = x_mut.signal();
    let y = x.map(|x| x * 2);
    let z = x_clone.map(|x| x * 3);
    let t = map_ref!(y,z => *y + *z);
    let future = t.for_each(|value| {
        // This code is run for the current value of my_state, and also every time my_state changes
        println!("{}", value);
        ready(())
    });
    executor::block_on(future);
    *x_mut.lock_mut() = 1;
    *x_mut.lock_mut() = 2;
}
