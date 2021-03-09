use std::{cell::RefCell, rc::Rc};

mod generated {
    use rerust::rerust;

    rerust! {
        let name = Var::<String>(String::new());
        let text = Evt::<String>();
        let message = (text, name).map(|t: &String, n: &String| -> String { format!("{}: {}", n, t) });
        let room1 = message.fold(Vec::new(),|mut vec: Vec<String>, msg: &String| -> Vec<String> { vec.push(msg.clone()); vec });
        let room2 = Var::<Vec<String>>(vec![String::from("Me: a constant message")]);
        let index = Var::<usize>(0);
        let room_list = (room1, room2).map(|room1: &Vec<String>, room2: &Vec<String>| -> Vec<Vec<String>> { vec![room1.clone(), room2.clone()] });
        let pin selected_room = (room_list, index).map(|room_list: &Vec<Vec<String>>, index: &usize| -> Vec<String> { room_list[*index].clone() });
    }
}

fn main() {
    let mut prog = generated::Program::new();
    let mut sink = prog.sink();

    let observer = Rc::new(RefCell::new(observer_cb)) as Rc<_>;
    prog.observe_selected_room(Rc::downgrade(&observer));

    prog.init();

    sink.send_name(format!("Alice"));
    sink.send_text(format!("Hi bob!"));
    sink.send_text(format!("My name is Alice ;)"));
    sink.send_name(format!("Bob"));
    sink.send_text(format!("Hi Alice, nice to meet you!"));
    sink.send_index(1);
    for _ in 0..6 {
        prog.run();
    }
}

fn observer_cb(history: &Vec<String>) {
    println!("history: {:?}", history);
}
