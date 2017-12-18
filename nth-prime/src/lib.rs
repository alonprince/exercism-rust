use std::result::Result;

pub fn nth(n: usize) -> Result<usize, &'static str> {
  find_prime(n)
}

fn find_prime(n: usize) -> Result<usize, &'static str> {
  if n == 0 {
    return Err("invalid number");
  }
  let mut primes: Vec<usize> = vec![2, 3, 5, 7, 11, 13];
  let mut num: usize = 14;
  loop {
      if primes.len() >= n {
        let result = primes[n - 1];
        return Ok(result);
      }
      let is_prime_bool = is_prime(num);
      if is_prime_bool {
        let result = num;
        println!("new prime number is {}", num);
        primes.push(result);
      }
      num += 1;
  }
}

fn is_prime(n: usize) -> bool {
  for i in 2..n {
    if n % i == 0 {
      return false;
    }
  }
  true
}