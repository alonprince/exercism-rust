extern crate proverb;

fn main() {
    let input = vec!["nail", "shoe", "horse"];
    let res = proverb::build_proverb(input);
    println!("{}", res);
}