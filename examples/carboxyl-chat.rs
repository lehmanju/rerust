use carboxyl::{Signal, Sink, Stream};

fn main() {
    let sink: Sink<String> = Sink::new();
    let name = Signal::new(String::from(""));
    let text = sink.stream();
    // what does move do here? how does carboxyl do async?
    // carboxyl only allows move, otherwise impossible lifetime issues
    // all objects are send/sync/clone
    let message = text.map(move |l| name.sample() + ": " + &l);
    let room1 = message.fold(Vec::new(), |mut history, message| {
        history.push(message);
        history
    });
    let room2 = Signal::new(vec![String::from("Me: a constant message")]);
    let room_list = Signal::new(vec![room1, room2]);
    let index = Signal::new(0);
    let index_c = index.clone();
    let selected_room = room_list.map(move |room_list| {
        let idx = index.sample();
        room_list[idx].clone()
    });

    sink.send(String::from("Bob: Hi Bob!"));
}
