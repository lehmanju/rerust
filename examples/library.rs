use futures::channel::mpsc;
use futures::executor::ThreadPool;
use futures::StreamExt;
use rerust::signal::{FutureWrapperExt, Signal};
use rerust::{combinators::CombinedMap, fold::FoldSignalExt, var::Var};

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
        let index = Var::new(0u32);

        room1.on_change(|list| println!("List: {:?}", list)).await;
        //let selected_room =
    };

    futures::executor::block_on(routine);

    // DSL mit eigenen Keywords
}
