/// Largest prime factor of the number 600_851_475_143

fn main() {
    let mut n: u64 = 600_851_475_143;
    let mut divisors: Vec<u64> = Vec::new();

    let upper_bound_check: u64 = (n as f64).sqrt().ceil() as u64;

    let mut div = 2;
    while div <= upper_bound_check {
        if n % div == 0 {
            divisors.push(div);
            n /= div;
            // We don't want to increment because it may be a multiple divisor
        } else {
            div += 1;
        }
    }

    let biggest = divisors.last().unwrap_or(&n); // In case we haven't found a divisor then the number itself is a prime.
    println!("{biggest}");
}
