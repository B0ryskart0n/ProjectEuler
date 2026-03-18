/// Find the largest palindrome made from the product of two 3-digit numbers.
fn main() {
    let mut products = Vec::with_capacity(900 * 900);

    for m in 100..=999u32 {
        for n in 100..=999u32 {
            let product = m * n;

            // Naive string reversing, probably something like dividing by 10 and creating a new number from the back should be faster
            let rev = product
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            if product == rev {
                products.push(m * n);
            }
        }
    }

    products.sort();

    let answer = products.last().expect("Expected non-empty vec");

    println!("{answer}");
}
