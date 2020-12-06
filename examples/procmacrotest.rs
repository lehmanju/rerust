use rerust::rerust;

rerust! {
    let a = Var(0u32);
    let b = Var(0u32);
    let evt = Evt(i32);
    let c = (a,b).map(|ref mut a, mut b| -> u32 { a + b }) || (a,b).map(|a, b| -> u32 { a - b });
    let evt_fold = evt.fold(String::new(), |mut string, evt| -> String { string });
}

fn main() {}
