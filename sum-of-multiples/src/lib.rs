pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let first = factors.get(0);
    match first {
        None => return 0,
        Some(num) => return do_sum(*num, limit, factors),
    }
}

fn do_sum(first: u32, limit: u32, factors: &[u32]) -> u32 {
    let mut res: Vec<u32> = Vec::new();
    for i in first..limit {
        let mut is_right = false;
        for j in factors {
            if i % j == 0 {
                is_right = true;
            }
        }
        if is_right {
            res.push(i);
        }
    }
    res.into_iter().sum()
}
