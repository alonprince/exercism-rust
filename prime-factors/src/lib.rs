pub fn factors(num: usize) -> Vec<usize> {
  let mut factor = 2usize;
  let mut n = num;
  let mut res: Vec<usize> = Vec::new();
  loop {
    if n % factor == 0 {
      n = n / factor;
      res.push(factor);
    } else {
      if n < factor {
        break;
      } else {
        factor += 1;
      }
    }
  }
  res
}