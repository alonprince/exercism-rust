extern crate prime_factors as pf;

fn main() {
    let res: Vec<usize> = pf::factors(93819012551);
    println!("{:?}", res);
}
