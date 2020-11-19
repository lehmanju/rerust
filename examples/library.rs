use futures::executor::ThreadPool;
use futures::StreamExt;
use futures::{channel::mpsc, Stream};
use rerust::signal::Signal;
use rerust::var::Var;

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
    };
}
