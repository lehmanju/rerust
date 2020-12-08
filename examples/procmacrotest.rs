use rerust::rerust;

rerust! {
    let a = Var(Box::new::<FnMut>());
    let b = Var(0u32);
    let evt = Evt(i32);
    let c = (a,b,evt).map(|ref mut a, mut b, evt| -> u32 { a + b + evt }) || (a,b).map(|a, b| -> u32 { a - b });
    let evt_fold = evt.fold(String::new(), |mut string, evt| -> String { string });
}

/// struct ReProgram
/// 
/// impl ReTrait for ReProgram

fn main() {

    
    let program = ReProgram::default();
    let program2 = ReProgram::default();
    program.a.input(program2);
    program.a.register(out_listener);

}
