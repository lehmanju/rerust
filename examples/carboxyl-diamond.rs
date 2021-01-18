use carboxyl::Sink;

fn main() {
    let x_sink = Sink::new();
    let x = x_sink.stream().hold(1);
    let y = x.map(|x| x * 2);
    let z = x.map(|x| x * 3);
    let t = z.map(move |x| x + y.sample());
    
    println!("{}", t.sample()); // 1*2 + 1*3 = 5
    x_sink.send(5);
    println!("{}", t.sample()); // 5*2 + 5*3 = 25

    // fails glitch-freedom
}