extern crate nth_prime as np;

fn main() {
  let result = np::nth(0);
  println!("{:?}", result.is_err());
}