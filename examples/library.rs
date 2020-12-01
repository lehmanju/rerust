use futures::channel::mpsc;
use futures::executor::ThreadPool;
use futures::{future::ready, StreamExt};
use rerust::signal::{FutureWrapperExt, Signal};
use rerust::{combinators::CombinedMap, fold::FoldSignalExt, var::Var};

fn main() {
    let pool = ThreadPool::new().expect("Failed to build pool");
    let (tx, rx) = mpsc::unbounded::<String>();

    let sending = async move {
        loop {
            tx.unbounded_send(String::from("Bob: Hi Alice!"))
                .expect("Not Sending?");
            tx.unbounded_send("Alice: Hi Bob!".into())
                .expect("not sending 2");
            //println!("Loop running?");
        }
    };

    // Begin FRP
    let name = Var::new(String::from(""));
    let text = rx;
    let message = text.map(|l| name.value() + ": " + &l);
    let room1 = message.fold_signal(Vec::new(), |mut history, message| {
        history.push(message);
        history
    });
    let room2 = Var::new(vec![String::from("Me: a constant message")]);
    let room_list = CombinedMap::new(room1.clone(), room2.clone(), |r1, r2| vec![r1, r2]);
    let index = Var::new(0u32);
    //let selected_room =

    // End FRP

    // Listen on signals
    let routine = room1.on_change(|list| {
        println!("List: {:?}", list);
        ready(())
    });

    pool.spawn_ok(sending);
    futures::executor::block_on(routine);

    // DSL mit eigenen Keywords
}
