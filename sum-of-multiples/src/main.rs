extern crate sum_of_multiples as som;

fn main() {
    let res = som::sum_of_multiples(10000, &vec![]);
    println!("{}", res);
}