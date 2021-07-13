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

mod parallel {
    use rerust::rerust;
    rerust! {
        let source1 = Evt::<i32>();
        let source2 = Evt::<i32>();
        let source3 = Evt::<i32>();
        let source4 = Evt::<i32>();
        let source5 = Evt::<i32>();
        let source6 = Evt::<i32>();
        let a = source1.map(|v: &i32| -> i32 { v + 1 });
        let  b = source2.map(|v: &i32| -> i32 { v + 1 });
        let  c = source3.map(|v: &i32| -> i32 { v + 1 });
        let d = source4.map(|v: &i32| -> i32 { v + 1 });
        let  e = source5.map(|v: &i32| -> i32 { v + 1 });
        let pin f = source6.map(|v: &i32| -> i32 { v + 1 });
    }
}

mod parallel_manual {
    #[derive(Clone, Default)]
    pub struct ManualState {
        pub source1: i32,
        pub source2: i32,
        pub source3: i32,
        pub source4: i32,
        pub source5: i32,
        pub source6: i32,
        pub f: i32,
    }

    #[derive(Clone, Default)]
    pub struct ManualChange {
        pub source1: bool,
        pub source2: bool,
        pub source3: bool,
        pub source4: bool,
        pub source5: bool,
        pub source6: bool,
        pub f: bool,
    }

    pub fn manual(
        s1: i32,
        s2: i32,
        s3: i32,
        s4: i32,
        s5: i32,
        s6: i32,
        state: &mut ManualState,
        change: &mut ManualChange,
    ) {
        if s1 != state.source1 {
            state.source1 = s1;
            change.source1 = true;
            let source1 = state.source1;

            let a = source1 + 1;
        }
        if s2 != state.source2 {
            state.source2 = s2;
            change.source2 = true;
            let source2 = state.source2;
            let a = source2 + 1;
        }

        if s3 != state.source3 {
            state.source3 = s3;
            change.source3 = true;
            let source3 = state.source3;
            let a = source3 + 1;
        }
        if s4 != state.source4 {
            state.source4 = s4;
            change.source4 = true;
            let source4 = state.source4;
            let a = source4 + 1;
        }
        if s5 != state.source5 {
            state.source5 = s5;
            change.source5 = true;
            let source5 = state.source5;
            let a = source5 + 1;
        }
        if s6 != state.source6 {
            state.source6 = s6;
            change.source6 = true;
            let source6 = state.source6;
            let a = source6 + 1;
        }
    }
}

mod pinned {
    use rerust::rerust;
    rerust! {
        let source = Evt::<i32>();
        let pin a = source.map(|v: &i32| -> i32 { v + 1 });
        let pin b = a.map(|v: &i32| -> i32 { v + 1 });
        let pin c = b.map(|v: &i32| -> i32 { v + 1 });
        let pin d = c.map(|v: &i32| -> i32 { v + 1 });
        let pin e = d.map(|v: &i32| -> i32 { v + 1 });
        let pin f = e.map(|v: &i32| -> i32 { v + 1 });
    }
}

#[derive(Clone)]
struct ManualState {
    f: i32,
}

#[derive(Clone)]
struct ManualChange {
    f: bool,
}

fn manual(value: i32, state: &mut ManualState, change: &mut ManualChange) {
    let source = value;
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

pub fn manual_line(c: &mut Criterion) {
    let mut state = ManualState { f: 0 };
    let change = ManualChange { f: false };
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

    let state = pinned::State::default();
    let mut updated_input = pinned::Input::default();
    updated_input.set_source(1);

    group.bench_function("rerust-pinned", move |b| {
        b.iter_batched(
            || (state.clone(), updated_input.clone()),
            |(mut state, input)| {
                pinned::Program::update(&mut state, input);
            },
            BatchSize::SmallInput,
        );
    });

    let state = parallel::State::default();
    let mut updated_input = parallel::Input::default();
    updated_input.set_source1(1);
    updated_input.set_source2(1);
    updated_input.set_source3(1);
    updated_input.set_source4(1);
    updated_input.set_source5(1);
    updated_input.set_source6(1);

    group.bench_function("rerust-parallel", move |b| {
        b.iter_batched(
            || (state.clone(), updated_input.clone()),
            |(mut state, input)| {
                parallel::Program::update(&mut state, input);
            },
            BatchSize::SmallInput,
        );
    });

    let state = parallel_manual::ManualState::default();
    let change = parallel_manual::ManualChange::default();

    group.bench_function("manual-parallel", move |b| {
        b.iter_batched(
            || (state.clone(), change.clone()),
            |(mut state, mut change)| {
                parallel_manual::manual(
                    black_box(1),
                    black_box(1),
                    black_box(1),
                    black_box(1),
                    black_box(1),
                    black_box(1),
                    &mut state,
                    &mut change,
                );
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, manual_line);
criterion_main!(benches);
