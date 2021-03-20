use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};

mod unpin {
    use rerust::rerust;
    rerust! {
        let constant_a = Var::<i32>(6490);
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
        let pin stringout = (constant_a, constant_n, euclid).map(|a: &i32, n: &i32, v: &i32| -> String {
            format!("GCD of {} and {} is {}", a, n, v)
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
        let pin stringout = (constant_a, constant_n, euclid).map(|a: &i32, n: &i32, v: &i32| -> String {
            format!("GCD of {} and {} is {}", a, n, v)
        });
    }
}

pub fn rerust_expensive(c: &mut Criterion) {
    let mut group = c.benchmark_group("rerust-expensive");
    group.throughput(Throughput::Elements(1));

    let state = unpin::State::default();
    let updated_input = unpin::Input::default();

    group.bench_function("unpin", move |b| {
        b.iter_batched(
            || (state.clone(), updated_input.clone()),
            |(mut state, input)| {
                unpin::Program::update(&mut state, input);
            },
            BatchSize::SmallInput,
        );
    });

    let state = pin::State::default();
    let updated_input = pin::Input::default();

    group.bench_function("pin", move |b| {
        b.iter_batched(
            || (state.clone(), updated_input.clone()),
            |(mut state, input)| {
                pin::Program::update(&mut state, input);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, rerust_expensive);
criterion_main!(benches);
