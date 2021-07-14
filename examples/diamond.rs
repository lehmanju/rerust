extern crate alloc;
use alloc::collections::vec_deque::VecDeque;
use alloc::rc::Rc;
use core::cell::RefCell;

mod generated {
    use rerust::rerust;

    rerust! {
        let x = Var::<u32>(1u32);
        let y = x.map(|x: &u32| -> u32 {x * 2});
        let z = x.map(|x: &u32| -> u32 {x * 3});
        let pin t = (y,z).map(|y: &u32, z: &u32| -> u32 {y + z});
    }
}

fn main() {
    let mut prog = generated::Program::new();
    let sink: Rc<RefCell<VecDeque<generated::Input>>> = prog.sink();

    let observer = Rc::new(RefCell::new(observer_cb)) as Rc<_>;
    prog.observe_t(Rc::downgrade(&observer));

    sink.borrow_mut()
        .push_back(generated::Input { var_0: Some(8u32) });
    prog.init();
    for _ in 0..5 {
        prog.run();
    }
}

fn observer_cb(t: &u32) {
    println!("observer {}", t)
}
