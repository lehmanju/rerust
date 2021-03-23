use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};

mod var {
    use rerust::rerust;
    rerust! {
        let source = Var::<i32>(0i32);
        let a = source.map(|v: &i32| -> i32 { v + 1 });
        let b = a.map(|v: &i32| -> i32 { v + 1 });
        let c = b.map(|v: &i32| -> i32 { v + 1 });
        let d = c.map(|v: &i32| -> i32 { v + 1 });
        let e = d.map(|v: &i32| -> i32 { v + 1 });
        let pin f = e.map(|v: &i32| -> i32 { v + 1 });
    }
}

mod evt {
    use rerust::rerust;
    rerust! {
        let source = Evt::<i32>();
        let a = source.map(|v: &i32| -> i32 { v + 1 });
        let b = a.map(|v: &i32| -> i32 { v + 1 });
        let c = b.map(|v: &i32| -> i32 { v + 1 });
        let d = c.map(|v: &i32| -> i32 { v + 1 });
        let e = d.map(|v: &i32| -> i32 { v + 1 });
        let pin f = e.map(|v: &i32| -> i32 { v + 1 });
    }
}

#[derive(Clone)]
struct ManualState {
    source: i32,
    f: i32,
}

#[derive(Clone)]
struct ManualChange {
    source: bool,
    f: bool,
}

fn manual(value: i32, state: &mut ManualState, change: &mut ManualChange) {
    if value != state.source {
        state.source = value;
        change.source = true;
        let source = state.source;
        let a = source + 1;
        let b = a + 1;
        let c = b + 1;
        let d = c + 1;
        let e = d + 1;
        let f = e + 1;
        if f != state.f {
            state.f = f;
            change.f = true;
        }
    }
}

pub fn manual_line(c: &mut Criterion) {
    let mut state = ManualState { source: 0, f: 0 };
    let change = ManualChange {
        source: false,
        f: false,
    };
    manual(1, &mut state, &mut change.clone());
    manual(0, &mut state, &mut change.clone());

    let mut group = c.benchmark_group("chain");
    group.throughput(Throughput::Elements(1));

    group.bench_function("manual", move |b| {
        b.iter_batched(
            || (state.clone(), change.clone()),
            |(mut state, mut change)| {
                manual(black_box(1), &mut state, &mut change);
            },
            BatchSize::SmallInput,
        );
    });

    let state = var::State::default();
    let mut updated_input = var::Input::default();
    updated_input.set_source(1);

    group.bench_function("rerust-var", move |b| {
        b.iter_batched(
            || (state.clone(), updated_input.clone()),
            |(mut state, input)| {
                var::Program::update(&mut state, input);
            },
            BatchSize::SmallInput,
        );
    });

    let state = evt::State::default();
    let mut updated_input = evt::Input::default();
    updated_input.set_source(1);

    group.bench_function("rerust-evt", move |b| {
        b.iter_batched(
            || (state.clone(), updated_input.clone()),
            |(mut state, input)| {
                evt::Program::update(&mut state, input);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, manual_line);
criterion_main!(benches);
