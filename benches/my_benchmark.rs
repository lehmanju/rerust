use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

mod natgraph {
    use rerust::rerust_gen;
    rerust_gen! {
        let source = Var::<i32>(0i32);
        let c1 = source;
        let b1 = source.map(|v: i32| -> i32 { v + 1 });
        let b2 = b1.map(|v: i32| -> i32 { v + 1 });
        let b3 = b2.map(|v: i32| -> i32 { v + 1 });
        let c2 = b3.map(|v: i32| -> i32 { v + 1 });
        let c3 = c2.map(|v: i32| -> i32 { 0 });
        let c4 = c3.map(|v: i32| -> i32 { v + 1 });
        let a1 = b2.map(|v: i32| -> i32 { v + 1 });
        let a2 = a1.map(|v: i32| -> i32 { v + 1 });
        let a3 = (a2,b2).map(|(a,b) : (i32,i32)| -> i32 { a + b });
        let a4 = a3.map(|v: i32| -> i32 { v + 1 });
        let b4 = (a4,b3).map(|(a,b) : (i32,i32)| -> i32 { 0 });
        let b5 = b4.map(|v: i32| -> i32 { v + 1 });
        let b6 = b5.map(|v: i32| -> i32 { v + 1 });
        let b7 = b6.map(|v: i32| -> i32 { v + 1 });
        let b8 = (b7,c2).map(|(a,b) : (i32,i32)| -> i32 { a + b });
        let c5 = (c4,b8).map(|(a,b) : (i32,i32)| -> i32 { a + b });
        let d1 = c2.map(|v: i32| -> i32 { v + 1 });
        let e1 = c1.map(|v: i32| -> i32 { 0 });
        let e2 = e1.map(|v: i32| -> i32 { v + 1 });
        let e3 = e2.map(|v: i32| -> i32 { v + 1 });
        let e4 = e3.map(|v: i32| -> i32 { v + 1 });
        let e5 = (e4,c2).map(|(a,b) : (i32,i32)| -> i32 { a + b });
        let e6 = c2.map(|v: i32| -> i32 { v + 1 });
        let e7 = (e6,d1).map(|(a,b) : (i32,i32)| -> i32 { a + b });
    }
}

#[derive(Default, Clone)]
struct State {
    source: i32,
    c1: i32,
    b1: i32,
    b2: i32,
    b3: i32,
    c2: i32,
    c3: i32,
    c4: i32,
    a1: i32,
    a2: i32,
    a3: i32,
    a4: i32,
    b4: i32,
    b5: i32,
    b6: i32,
    b7: i32,
    b8: i32,
    c5: i32,
    d1: i32,
    e1: i32,
    e2: i32,
    e3: i32,
    e4: i32,
    e5: i32,
    e6: i32,
    e7: i32,
}

#[derive(Default, Clone)]
struct Change {
    source: bool,
    c1: bool,
    b1: bool,
    b2: bool,
    b3: bool,
    c2: bool,
    c3: bool,
    c4: bool,
    a1: bool,
    a2: bool,
    a3: bool,
    a4: bool,
    b4: bool,
    b5: bool,
    b6: bool,
    b7: bool,
    b8: bool,
    c5: bool,
    d1: bool,
    e1: bool,
    e2: bool,
    e3: bool,
    e4: bool,
    e5: bool,
    e6: bool,
    e7: bool,
}

fn natgraph_manual(value: i32, state: &mut State, change: &mut Change) {
    if value != state.source {
        let source = value;
        change.source = true;
        state.source = source;
        let c1 = source;
        change.c1 = true;
        state.c1 = c1;
        let b1 = source + 1;
        change.b1 = true;
        state.b1 = b1;
        let b2 = b1 + 1;
        change.b2 = true;
        state.b2 = b2;
        let b3 = b2 + 1;
        change.b3 = true;
        state.b3 = b3;
        let c2 = b3 + 1;
        change.c2 = true;
        state.c2 = c2;
        let c3 = c2 - c2;
        if state.c3 != c3 {
            change.c3 = true;
            state.c3 = c3;
            let c4 = c3 + 1;
            change.c4 = true;
            state.c4 = c4;
        }
        let a1 = b2 + 1;
        change.a1 = true;
        state.a1 = a1;
        let a2 = a1 + 1;
        change.a2 = true;
        state.a2 = a2;
        let a3 = a2 + b2;
        if a3 != state.a3 {
            change.a3 = true;
            state.a3 = a3;
            let a4 = a3 + 1;
            change.a4 = true;
            state.a4 = a4;
        }
        let b4 = state.a4 + b3;
        if b4 != state.b4 {
            change.b4 = true;
            state.b4 = b4;

            let b5 = b4 + 1;
            change.b5 = true;
            state.b5 = b5;
            let b6 = b5 + 1;
            change.b6 = true;
            state.b6 = b6;
            let b7 = b6 + 1;
            change.b7 = true;
            state.b7 = b7;
        }
        let b8 = state.b7 + c2;
        if b8 != state.b8 {
            change.b8 = true;
            state.b8 = b8;
        }
        let c5 = state.c4 + b8;
        if c5 != state.c5 {
            change.c5 = true;
            state.c5 = c5;
        }
        let d1 = c2 + 1;
        change.d1 = true;
        state.d1 = d1;
        let e1 = c1 - c1;
        if state.e1 != e1 {
            change.e1 = true;
            state.e1 = e1;
            let e2 = e1 + 1;
            change.e2 = true;
            state.e2 = e2;
            let e3 = e2 + 1;
            change.e3 = true;
            state.e3 = e3;
            let e4 = e3 + 1;
            change.e4 = true;
            state.e4 = e4;
        }
        let e5 = state.e4 + c2;
        if e5 != state.e5 {
            change.e5 = true;
            state.e5 = e5;
        }
        let e6 = c2 + 1;
        change.e6 = true;
        state.e6 = e6;
        let e7 = e6 + d1;
        if e7 != state.e7 {
            change.e7 = true;
            state.e7 = e7;
        }
    }
}

pub fn natural_graph_rerust(c: &mut Criterion) {
    let mut state = natgraph::State::default();
    let init = natgraph::Input::initial();
    let mut change = natgraph::Change::default();
    natgraph::Program::update(&mut state, init, &mut change);
    let mut updated_input = natgraph::Input::default();
    updated_input.set_source(1);

    // let mut group = c.benchmark_group("natgraph_rerust");
    // for v in 0..30 {
    // 	group.bench_with_input(BenchmarkId::from_parameter(v), &v, |b,&v| b.iter(|| {
    // 		sink.send_source(v);
    // 		black_box(prog.run());
    // 	}));
    // }
    // group.finish();

    c.bench_function("natgraph_rerust", move |b| {
        b.iter_batched(
            || {
                (
                    state.clone(),
                    updated_input.clone(),
                    natgraph::Change::default(),
                )
            },
            |(mut state, input, mut change)| {
                natgraph::Program::update(&mut state, input, &mut change);
            },
            BatchSize::SmallInput,
        )
    });
}

pub fn natural_graph_manual(c: &mut Criterion) {
    let mut state = State::default();
    let mut change = Change::default();
    natgraph_manual(1, &mut state, &mut change);
    natgraph_manual(0, &mut state, &mut change);

    c.bench_function("natgraph_manual", move |b| {
        b.iter_batched(
            || (state.clone(), change.clone()),
            |(mut state, mut change)| {
                natgraph_manual(black_box(1), &mut state, &mut change);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, natural_graph_rerust, natural_graph_manual);
criterion_main!(benches);
