use std::{cell::RefCell, rc::Rc};

mod generated {
    use rerust::rerust_gen;

    rerust_gen! {
        let x = Var::<u32>(1u32);
        let y = x.map(|x: u32| -> u32 {x * 2});
        let z = x.map(|x: u32| -> u32 {x * 3});
        let t = (y,z).map(|(y,z): (u32,u32)| -> u32 {y + z});
    }
}

fn main() {
    let mut prog = generated::Program::new();
    let mut sink = prog.sink().clone();
    sink.take_all(prog.sink());

    let observer = Rc::new(RefCell::new(observer_cb)) as Rc<_>;
    prog.observe_t(Rc::downgrade(&observer));

    sink.send_x(2);
    for _ in 0..5 {
        prog.run();
    }
}

fn observer_cb(t: &u32) {
    println!("t: {:?}", t);
}
