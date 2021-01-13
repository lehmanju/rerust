
mod generated {
    use rerust::rerust_gen;

    rerust_gen! {
        let a = Var::<i32>(0i32);
        let b = Var::<u32>(0u32);
        let evt = Evt::<i32>();
        let c = (a,b,evt).map(|(a, b, evt) : (i32, u32, i32)| -> u32 { (a + b as i32 + evt) as u32 }) || (a,b).map(|(a, b): (i32, u32)| -> u32 { a as u32 - b });
        let evt_fold = evt.fold(String::new(), |string: String, evt: i32| -> String { string });
    }
}

/// struct ReProgram
///
/// impl ReTrait for ReProgram

fn main() {}
