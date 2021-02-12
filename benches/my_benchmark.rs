use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

mod natgraph {
    use rerust::rerust_gen;
    rerust_gen! {
        let source = Var::<i32>(0i32);
        let c1 = source;
        let b1 = source.map(|v: &i32| -> i32 { v + 1 });
        let b2 = b1.map(|v: &i32| -> i32 { v + 1 });
        let b3 = b2.map(|v: &i32| -> i32 { v + 1 });
        let c2 = b3.map(|v: &i32| -> i32 { v + 1 });
        let c3 = c2.map(|v: &i32| -> i32 { 0 });
        let c4 = c3.map(|v: &i32| -> i32 { v + 1 });
        let a1 = b2.map(|v: &i32| -> i32 { v + 1 });
        let a2 = a1.map(|v: &i32| -> i32 { v + 1 });
        let a3 = (a2,b2).map(|(a,b) : &(i32,i32)| -> i32 { *a + *b });
        let a4 = a3.map(|v: &i32| -> i32 { v + 1 });
        let b4 = (a4,b3).map(|(a,b) : &(i32,i32)| -> i32 { 0 });
        let b5 = b4.map(|v: &i32| -> i32 { v + 1 });
        let b6 = b5.map(|v: &i32| -> i32 { v + 1 });
        let b7 = b6.map(|v: &i32| -> i32 { v + 1 });
        let b8 = (b7,c2).map(|(a,b) : &(i32,i32)| -> i32 { *a + *b });
        let c5 = (c4,b8).map(|(a,b) : &(i32,i32)| -> i32 { *a + *b });
        let d1 = c2.map(|v: &i32| -> i32 { v + 1 });
        let e1 = c1.map(|v: &i32| -> i32 { 0 });
        let e2 = e1.map(|v: &i32| -> i32 { v + 1 });
        let e3 = e2.map(|v: &i32| -> i32 { v + 1 });
        let e4 = e3.map(|v: &i32| -> i32 { v + 1 });
        let e5 = (e4,c2).map(|(a,b) : &(i32,i32)| -> i32 { *a + *b });
        let e6 = c2.map(|v: &i32| -> i32 { v + 1 });
        let e7 = (e6,d1).map(|(a,b) : &(i32,i32)| -> i32 { *a + *b });
    }
}

#[derive(Default, Clone)]
struct State {
    source: Option<i32>,
    c1: Option<i32>,
    b1: Option<i32>,
    b2: Option<i32>,
    b3: Option<i32>,
    c2: Option<i32>,
    c3: Option<i32>,
    c4: Option<i32>,
    a1: Option<i32>,
    a2: Option<i32>,
    a3: Option<i32>,
    a4: Option<i32>,
    b4: Option<i32>,
    b5: Option<i32>,
    b6: Option<i32>,
    b7: Option<i32>,
    b8: Option<i32>,
    c5: Option<i32>,
    d1: Option<i32>,
    e1: Option<i32>,
    e2: Option<i32>,
    e3: Option<i32>,
    e4: Option<i32>,
    e5: Option<i32>,
    e6: Option<i32>,
    e7: Option<i32>,
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
    if state.source.is_none() || value != *state.source.as_ref().unwrap() {
        let source = value;
        change.source = true;
        state.source = Some(source);
    }
    if change.source {
        let c1 = state.source;
        change.c1 = true;
        state.c1 = c1;
    }
    if change.source {
        let b1 = *state.source.as_ref().unwrap() + 1;
        change.b1 = true;
        state.b1 = Some(b1);
    }
    if change.source {
        let b2 = *state.source.as_ref().unwrap() + 1;
        change.b2 = true;
        state.b2 = Some(b2);
    }
    if change.b2 {
        let b3 = *state.b2.as_ref().unwrap() + 1;
        change.b3 = true;
        state.b3 = Some(b3);
    }
    if change.b3 {
        let c2 = *state.b3.as_ref().unwrap() + 1;
        change.c2 = true;
        state.c2 = Some(c2);
    }
    if change.c2 {
        let c3 = *state.c2.as_ref().unwrap() - *state.c2.as_ref().unwrap();
        if state.c3.is_none() || *state.c3.as_ref().unwrap() != c3 {
            change.c3 = true;
            state.c3 = Some(c3);
        }
    }
    if change.c3 {
        let c4 = *state.c3.as_ref().unwrap() + 1;
        change.c4 = true;
        state.c4 = Some(c4);
    }
    if change.b2 {
        let a1 = *state.b2.as_ref().unwrap() + 1;
        change.a1 = true;
        state.a1 = Some(a1);
    }
    if change.a1 {
        let a2 = *state.a1.as_ref().unwrap() + 1;
        change.a2 = true;
        state.a2 = Some(a2);
    }
    if change.a2 || change.b2 {
        let a3 = *state.a2.as_ref().unwrap() + *state.b2.as_ref().unwrap();
        if state.a3.is_none() || a3 != *state.a3.as_ref().unwrap() {
            change.a3 = true;
            state.a3 = Some(a3);
        }
    }
    if change.a3 {
        let a4 = *state.a3.as_ref().unwrap() + 1;
        change.a4 = true;
        state.a4 = Some(a4);
    }
    if change.a4 || change.b3 {
        let b4 = *state.a4.as_ref().unwrap() + *state.b3.as_ref().unwrap();
        if state.b4.is_none() || b4 != *state.b4.as_ref().unwrap() {
            change.b4 = true;
            state.b4 = Some(b4);
        }
    }
    if change.b4 {
        let b5 = *state.b4.as_ref().unwrap() + 1;
        change.b5 = true;
        state.b5 = Some(b5);
    }
    if change.b5 {
        let b6 = *state.b5.as_ref().unwrap() + 1;
        change.b6 = true;
        state.b6 = Some(b6);
    }
    if change.b6 {
        let b7 = *state.b6.as_ref().unwrap() + 1;
        change.b7 = true;
        state.b7 = Some(b7);
    }
    if change.b7 || change.c2 {
        let b8 = *state.b7.as_ref().unwrap() + *state.c2.as_ref().unwrap();
        if state.b8.is_none() || b8 != *state.b8.as_ref().unwrap() {
            change.b8 = true;
            state.b8 = Some(b8);
        }
    }
    if change.c4 || change.b8 {
        let c5 = *state.c4.as_ref().unwrap() + *state.b8.as_ref().unwrap();
        if state.c5.is_none() || c5 != *state.c5.as_ref().unwrap() {
            change.c5 = true;
            state.c5 = Some(c5);
        }
    }
    if change.c2 {
        let d1 = *state.c2.as_ref().unwrap() + 1;
        change.d1 = true;
        state.d1 = Some(d1);
    }
    if change.c1 {
        let e1 = *state.c1.as_ref().unwrap() - *state.c1.as_ref().unwrap();
        if state.e1.is_none() || *state.e1.as_ref().unwrap() != e1 {
            change.e1 = true;
            state.e1 = Some(e1);
        }
    }
    if change.e1 {
        let e2 = *state.e1.as_ref().unwrap() + 1;
        change.e2 = true;
        state.e2 = Some(e2);
    }
    if change.e2 {
        let e3 = *state.e2.as_ref().unwrap() + 1;
        change.e3 = true;
        state.e3 = Some(e3);
    }
    if change.e3 {
        let e4 = *state.e3.as_ref().unwrap() + 1;
        change.e4 = true;
        state.e4 = Some(e4);
    }
    if change.e4 {
        let e5 = *state.e4.as_ref().unwrap() + *state.c2.as_ref().unwrap();
        if state.e5.is_none() || e5 != *state.e5.as_ref().unwrap() {
            change.e5 = true;
            state.e5 = Some(e5);
        }
    }
    if change.c2 {
        let e6 = *state.c2.as_ref().unwrap() + 1;
        change.e6 = true;
        state.e6 = Some(e6);
    }
    if change.e6 || change.d1 {
        let e7 = *state.e6.as_ref().unwrap() + *state.d1.as_ref().unwrap();
        if state.e7.is_none() || e7 != *state.e7.as_ref().unwrap() {
            change.e7 = true;
            state.e7 = Some(e7);
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
