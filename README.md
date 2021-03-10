# Rerust

This is the source code for *ReRust*, a functional reactive programming language (FRP) that provides dataflow programming in Rust.

## Usage

To get started, install the Rust toolchain and add following line to your `Cargo.toml` file:

```Toml
[dependencies]
rerust = "0.1.0"
```

*ReRust* uses procedural macros to generate native Rust code. Each macro invocation should be placed inside a separate module to avoid name conflicts. Look at `examples/chat.rs` and `examples/diamond.rs` to get some inspiration.

## Getting started

*ReRust* code is placed inside a procedural macro to generate structs and functions that can be used from plain Rust. A macro invocation typically looks like this:

```Rust
mod macrodemo {
	use rerust::rerust;
	rerust! {
        let x = Var::<u32>(1u32);
        let y = x.map(|x: &u32| -> u32 {x * 2});
        let z = x.map(|x: &u32| -> u32 {x * 3});
        let pin t = (y,z).map(|y: &u32, z: &u32| -> u32 {y + z});
	}
}
```

Afterwards you can use as many instances of your program as you would like. To update your program state, you need to retrieve a `Sink` which allows updating reactives with new values. If the state has changed between two iterations all registered observers are notified, this is useful for updating GUIs for example. Finally, you need to regularly call `Program::run()` to poll the sink and update the state. Pushing values to a sink is done via `Sink::send_<source_name>(<val>)`.

```Rust
    let mut prog = generated::Program::new();
    let mut sink = prog.sink();

	// register observer, takes reference of t's type
    let observer = Rc::new(RefCell::new(observer_cb)) as Rc<_>;
    prog.observe_t(Rc::downgrade(&observer));

	// update x value
    sink.send_x(2);
   
    // initialize program and call observers with initial values
	prog.init();
    for _ in 0..5 {
		// check for new input, udpate state and notify observers
        prog.run();
    }
```

## Available primitives

- **Var/Evt**: Source reactives that either preserve state for the next iteration (*Variable*) or are invalidated after one evaluation (*Event*)
- **Map**: Maps (multiple) reactives to a new output reactive by calling the provided closure on their values. Available for *Events* and *Variables*. If at least one *Event* is present as input, Map will be an *Event* reactive as well.
- **Fold**: Takes at least one *Event* and any number of *Variables* as input and is of type *Variable*. Accumulates a value over time.
- **Changed**: Takes exactly one *Variable* and transforms it into an *Event*, firing only if the incoming reactive has changed.
- **Filter**: Filters events from an *Event* stream. Can depend on additional *Variables* for decision making. If the closure returns true, the event is forwarded, otherwise no event is fired.

## Benchmarks

*ReRust* has some predefined benchmarks available in `benches/`. To run them all type `cargo bench`.
