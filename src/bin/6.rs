/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
fn main() {
    let n: u64 = 100;

    let answer = (n * (n + 1)).pow(2) / 4 - (n * (n + 1) * (2 * n + 1)) / 6;

    println!("{answer}");
}
