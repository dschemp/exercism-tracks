pub fn raindrops(n: u32) -> String {
    let mut str = String::new();

    if n % 3 == 0 {
        str.push_str("Pling");
    }
    if n % 5 == 0 {
        str.push_str("Plang");
    }
    if n % 7 == 0 {
        str.push_str("Plong");
    }

    if str.len() == 0 {
        str = format!("{}", n);
    }

    str
}
