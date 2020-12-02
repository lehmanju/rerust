mod compiler {

    use std::{cell::RefCell, rc::Rc};

    trait Reactive {}

    struct Var {
        initiale: &'static str,
        reactives: Vec<Rc<RefCell<dyn Reactive>>>,
    }

    impl Reactive for Var {}

    struct Map {
        closure: &'static str,
        reactives: Vec<Rc<RefCell<dyn Reactive>>>,
    }

    impl Reactive for Map {}

    struct Fold {
        closure: &'static str,
        reactives: Vec<Rc<RefCell<dyn Reactive>>>,
        initiale: &'static str,
    }

    impl Reactive for Fold {}

    struct Filter {
        closure: &'static str,
        reactives: Vec<Rc<RefCell<dyn Reactive>>>,
    }

    impl Reactive for Filter {}

    struct Choice {
        reactives: Vec<Rc<RefCell<dyn Reactive>>>,
    }

    impl Reactive for Choice {}

    macro_rules! var {
        ($init:expr) => {
            Rc::new(RefCell::new(Var {
                initiale: stringify!($init),
                reactives: Vec::new(),
            }))
        };
    }

    macro_rules! map {
        ($($id:ident), +, $closure:block) => {
            {
                let rc = Rc::new(RefCell::new(Map { closure: stringify!($closure), reactives: Vec::new() }));
                $($id.borrow_mut().reactives.push(rc.clone());)+
                rc
            }
        }
    }

    macro_rules! fold {
        ($($id:ident), +, $closure:block, $init:expr) => {
            {
                let rc = Rc::new(RefCell::new(Fold { closure: stringify!($closure), reactives: Vec::new(), initiale: stringify!($init)}));
                $($id.borrow_mut().reactives.push(rc.clone());)+
                rc
            }
        }
    }

    macro_rules! choice {
        ($ida:ident, $idb:ident) => {{
            let rc = Rc::new(RefCell::new(Choice {
                reactives: Vec::new(),
            }));
            $ida.borrow_mut().reactives.push(rc.clone());
            $idb.borrow_mut().reactives.push(rc.clone());
            rc
        }};
    }

    fn test() {
        let v = var!(0u32);
        let map = map!(v, { v });
        let fold = fold!(v, map, {}, 1i32);
        let choice = choice!(v, map);
    }
}
