pub fn find() -> Option<u32> {
    let mut res: u32 = 0;
    for a in 1..999 {
        for b in 1..999 - a {
            let c = 1000 - a - b;
            if (a as u32).pow(2) + (b as u32).pow(2) == (c as u32).pow(2) {
                res = a * b * c;
                break;
            }
        }
        if res != 0 {
            break;
        }
    }
    Some(res)
}
