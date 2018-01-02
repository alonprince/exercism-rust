pub fn series(_digits: &str, _len: usize) -> Vec<String> {
    let letters: Vec<&str> = _digits.split("").collect();
    let total_len = letters.len();
    let mut result: Vec<String> = Vec::new();
    for i in 1..total_len - _len {
        let mut res = String::new();
        for j in 0.._len {
            res += letters[i + j]
        }
        result.push(res);
    }
    result
}
