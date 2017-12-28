pub fn find() -> Option<u32> {
    for a in 1..999 {
        for b in 1..999 - a {
            let c = 1000 - a - b;
            if (a as u32).pow(2) + (b as u32).pow(2) == (c as u32).pow(2) {
                let res: u32 = a * b * c;
                return Some(res);
            }
        }
    }
    None
}
