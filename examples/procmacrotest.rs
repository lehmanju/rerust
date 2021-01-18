use std::{cell::RefCell, rc::Rc, sync::mpsc::Sender};

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
    let name_sink: Sender<String> = prog.get_sink_name();
    let text_sink: Sender<String> = prog.get_sink_text();
    let index_sink: Sender<usize> = prog.get_sink_index();
    let observer = Rc::new(RefCell::new(observer_cb)) as Rc<_>;
    prog.observe_selected_room(Rc::downgrade(&observer));
    name_sink.send(format!("Alice"));
    text_sink.send(format!("Hi bob!"));
    text_sink.send(format!("My name is Alice ;)"));
    for _ in 0..2 {
        prog.run();
    }
    name_sink.send(format!("Bob"));
    text_sink.send(format!("Hi Alice, nice to meet you!"));
    //index_sink.send(1);
    prog.run();
}

fn observer_cb(history: &Vec<String>) {
    println!("history: {:?}", history);
}

