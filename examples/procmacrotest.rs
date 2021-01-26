use std::{cell::RefCell, rc::Rc};

mod generated {
    use rerust::rerust_gen;

    rerust_gen! {
        let name = Var::<String>(String::new());
        let text = Evt::<String>();
        let message = (text, name).map(|(t, n) : (String, String)| -> String { format!("{}: {}", n, t) });
        let room1 = message.fold(Vec::new(),|mut vec: Vec<String>, msg: String| -> Vec<String> { vec.push(msg); vec });
        let room2 = Var::<Vec<String>>(vec![String::from("Me: a constant message")]);
        let index = Var::<usize>(0);
        let room_list = (room1, room2).map(|(room1, room2) : (Vec<String>, Vec<String>)| -> Vec<Vec<String>> { vec![room1, room2] });
        let selected_room = (room_list, index).map(|(room_list, index) : (Vec<Vec<String>>, usize)| -> Vec<String> { room_list[index].clone() });
    }
}

fn main() {
    let mut prog = generated::Program::new();
    let mut sink = prog.sink().clone();
    sink.take_all(prog.sink());
    //let text_card = sink.pull_text();

    let observer = Rc::new(RefCell::new(observer_cb)) as Rc<_>;
    prog.observe_selected_room(Rc::downgrade(&observer));

    sink.send_name(format!("Alice"));
    sink.send_text(format!("Hi bob!"));
    sink.send_text(format!("My name is Alice ;)"));
    sink.send_name(format!("Bob"));
    sink.send_text(format!("Hi Alice, nice to meet you!"));
    sink.send_index(1);
    for _ in 0..5 {
        prog.run();
    }
}

fn observer_cb(history: &Vec<String>) {
    println!("history: {:?}", history);
}
