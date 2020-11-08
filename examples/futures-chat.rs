use futures::{executor, future, pin_mut, stream, stream::Peekable, FutureExt, StreamExt};
use futures_signals::signal::{Mutable, SignalExt};
use std::pin::Pin;

fn main() {
    let name = Mutable::new(String::from(""));
    let event_stream = stream::iter(vec![String::from("Alice: hi"), String::from("Bob: hi!")]);
    //let message = event_stream.map(move |l| name_signal + ": " + l);
}
