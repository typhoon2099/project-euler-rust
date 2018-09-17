pub fn main() -> u32 {
    let nth = 10001;
    let mut primes :Vec<u32> = Vec::new();

    let mut current :u32 = 2;
    while primes.len() < nth {
        let mut is_prime = true;

        let sqrt = (current as f64).sqrt() as u32;
        for &test in &primes {
//          No point in testing if we've gone past the square root
            if test > sqrt {
                break;
            }

//          Stop testing once we find a factor
            if current % test == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(current)
        }

        current += 1
    }

    return primes[nth - 1];
}
