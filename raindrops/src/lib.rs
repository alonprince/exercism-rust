pub fn raindrops(n: usize) -> String {
    let mut name = String::new();
    if n % 3 == 0 {
        name.push_str("Pling");
    }
    if n % 5 == 0 {
        name.push_str("Plang");
    }
    if n % 7 == 0 {
        name.push_str("Plong");
    }
    if name.is_empty() {
        name = n.to_string();
    }
    name
}
