extern crate difference_of_squares as dos;

fn main() {
    let sum1 = dos::square_of_sum(5);
    let sum2 = dos::sum_of_squares(5);
    println!("{} - {}", sum1, sum2);
}
