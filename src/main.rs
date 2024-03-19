fn _bang_functional(mut s: String) -> String {
    s += "!!1!!111";
    s
}

fn _bang_mutating(s: &mut String) {
    s.push_str("!!1!!111");
}

fn _strbang() {
    let mut s = "hello".to_string();
    _bang_mutating(&mut s);
    println!("{}", s);
    let s = _bang_functional(s);
    println!("{}", s);
}

fn _foreach() {
    let a = [1u8, 2, 3];

    for v in a {
        println!("{}", v);
    }
    a.into_iter().for_each(|v| println!("{}", v));
}

fn main() {
    let a = ['a', 'b', 'c'];
    let mapped: Vec<_> = a
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i & 1 == 0)
        .collect();
    println!("{:?}", mapped);
}
