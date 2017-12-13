pub fn raindrops(n: usize) -> String {
    let mut name = "".to_string();
    let mut is_work = false;
    if n % 3 == 0 {
        name = name + "Pling";
        is_work = true;
    }
    if n % 5 == 0 {
        name = name + "Plang";
        is_work = true;
    }
    if n % 7 == 0 {
        name = name + "Plong";
        is_work = true;
    }
    if !is_work {
        name = n.to_string();
    }
    name.to_string()
}
