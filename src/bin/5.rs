/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20 ?
fn main() {
    let upper_bound: u128 = 20;
    // An algorithm could be to iterate n and add only it's smallest prime factor to the list
    // Checking each n by dividing it by each already stored factor and reaching 1 means that it's
    // not needed, but if 1 is not reached then adding the accumulated (by division) factor should be added.
    // 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    // 2, 3, 2, 5,    7, 2, 3,     11,     13,         02, 17,     19

    let mut factors = Vec::<u128>::new();

    for n in 2..=upper_bound {
        let mut potential_factor = n;

        for known_factor in &factors {
            if potential_factor % known_factor == 0 {
                potential_factor /= known_factor;
            }
        }

        factors.push(potential_factor);
    }

    // 2. Or maybe do this from the biggest to the smallest number, but then one would need
    // a good factorization algorithm.
    // 3. Or using the gcd, but that would require implementing the gcd algo and:
    // lcm(a, b, ...) = (a * b * ...) / gcd(a, b, ...)

    let answer = factors.iter().product::<u128>();

    println!("{answer}");
}
