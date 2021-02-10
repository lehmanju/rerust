use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};


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
		let result = (c5, e5, e7);
	}
}

fn natgraph_manual(value: i32) -> (i32, i32, i32) {
	let source = value;
	let c1 = source;
	let b1 = source + 1;
	let b2 = b1 + 1;
	let b3 = b2 + 1;
	let c2 = b3 + 1;
	let c3 = c2 - c2;
	let c4 = c3 + 1;
	let a1 = b2 + 1;
	let a2 = a1 + 1;
	let a3 = a2 + b2;
	let a4 = a3 + 1;
	let b4 = a4 + b3;
	let b5 = b4 + 1;
	let b6 = b5 + 1;
	let b7 = b6 + 1;
	let b8 = b7 + c2;
	let c5 = c4 + b8;
	let d1 = c2 + 1;
	let e1 = c1 - c1;
	let e2 = e1 +1;
	let e3 = e2 + 1;
	let e4 = e3 + 1;
	let e5 = e4 + c2;
	let e6 = c2 + 1;
	let e7 = e6 + d1;
	(c5, e5, e7)
}

pub fn natural_graph_rerust(c: &mut Criterion) {
	let mut state = natgraph::State::default();
	let init = natgraph::Input::initial();
	natgraph::Program::update(&mut state, init);
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

	c.bench_function("natgraph_rerust", move |b| b.iter_batched(
		|| (state.clone(), updated_input.clone()),
		|(mut state, input)| {
			black_box(natgraph::Program::update(&mut state, input));
		},
		BatchSize::SmallInput
	));
}

pub fn natural_graph_manual(c: &mut Criterion) {
	c.bench_function("natural graph manual", |b| b.iter(|| {
		black_box(natgraph_manual(1));
	}));
}

criterion_group!(benches, natural_graph_rerust, natural_graph_manual);
criterion_main!(benches);
