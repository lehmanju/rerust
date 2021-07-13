use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};

mod unpin {
    use rerust::rerust;
    rerust! {
        let constant_a = Var::<i32>(649073);
        let constant_n = Var::<i32>(598);
        let euclid = (constant_a, constant_n).map(|a: &i32, n: &i32| -> i32 {
            let mut val_a = *a;
            let mut val_n = *n;
            while val_a != val_n {
                if val_a > val_n {
                    val_a = val_a - val_n;
                } else {
                    val_n = val_n - val_a;
                }
            }
            val_a
        });
    }
}

mod pin {
    use rerust::rerust;
    rerust! {
        let constant_a = Var::<i32>(649073);
        let constant_n = Var::<i32>(598);
        let pin euclid = (constant_a, constant_n).map(|a: &i32, n: &i32| -> i32 {
            let mut val_a = *a;
            let mut val_n = *n;
            while val_a != val_n {
                if val_a > val_n {
                    val_a = val_a - val_n;
                } else {
                    val_n = val_n - val_a;
                }
            }
            val_a
        });
    }
}

#[derive(Clone, Default)]
struct ManualState {
    sourcea: i32,
    sourceb: i32,
    f: i32,
}

#[derive(Clone, Default)]
struct ManualChange {
    sourcea: bool,
    sourceb: bool,
    f: bool,
}

fn manual(a: i32, b: i32, state: &mut ManualState, change: &mut ManualChange) {
    if a != state.sourcea {
        state.sourcea = a;
        change.sourcea = true;
        if b != state.sourceb {
            state.sourceb = b;
            change.sourceb = true;
        }
        if change.sourcea || change.sourceb {
            let mut val_a = state.sourcea;
            let mut val_n = state.sourceb;
            while val_a != val_n {
                if val_a > val_n {
                    val_a = val_a - val_n;
                } else {
                    val_n = val_n - val_a;
                }
            }
            let f = val_a;
            if f != state.f {
                state.f = f;
                change.f = true;
            }
        }
    }
}

pub fn rerust_expensive(c: &mut Criterion) {
    let mut group = c.benchmark_group("rerust-expensive");
    group.throughput(Throughput::Elements(1));

    let mut state = unpin::State::default();
    let mut updated_input = unpin::Input::default();

    group.bench_function("unpin", move |b| {
        b.iter_batched(
            || (state.clone(), updated_input.clone()),
            |(mut state, input)| {
                unpin::Program::update(&mut state, input);
            },
            BatchSize::SmallInput,
        );
    });

    let mut state = pin::State::default();
    let mut updated_input = pin::Input::default();
	
    group.bench_function("pin", move |b| {
        b.iter_batched(
            || (state.clone(), updated_input.clone()),
            |(mut state, input)| {
                pin::Program::update(&mut state, input);
            },
            BatchSize::SmallInput,
        );
    });

    let state = ManualState::default();
    let change = ManualChange::default();
    group.bench_function("manual", move |b| {
        b.iter_batched(
            || (state.clone(), change.clone()),
            |(mut state, mut change)| {
                manual(black_box(649073), black_box(598), &mut state, &mut change);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, rerust_expensive);
criterion_main!(benches);
