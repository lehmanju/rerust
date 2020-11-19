use futures::executor::ThreadPool;
use futures::StreamExt;
use futures::{channel::mpsc, Stream};
use rerust::signal::{CombinedMap, FoldSignal, FoldSignalExt, Signal};
use rerust::var::Var;
use std::sync::Arc;

fn main() {
    let pool = ThreadPool::new().expect("Failed to build pool");
    let (tx, rx) = mpsc::unbounded::<String>();

    let routine = async {
        let sending = async move {
            tx.unbounded_send(String::from("Bob: Hi Alice!"))
                .expect("Not Sending?");
        };

        pool.spawn_ok(sending);

        let name = Var::new(String::from(""));
        let text = rx;
        let message = text.map(|l| name.value() + ": " + &l);
        let room1 = message.fold_signal(Vec::new(), |mut history, message| {
            history.push(message);
            history
        });
        let room2 = Var::new(vec![String::from("Me: a constant message")]);
        let room_list = CombinedMap::new(room1.clone(), room2.clone(), |r1, r2| vec![r1, r2]);
        let index = Var::new(0);
        //let selected_room =
    };
}
