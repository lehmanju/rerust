use rerust::rerust;

fn main() {
    let graphobject = rerust! {{
        let v = var!(0u32);
        let map = map!(v, { v });
        let fold = fold!(v, map, {}, 1i32);
        let choice = choice!(v, map);}
    };
}
