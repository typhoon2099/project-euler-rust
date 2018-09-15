pub fn main() -> u32 {
    let nth = 10001;
    let mut primes :Vec<u32> = Vec::new();

    let mut current :u32 = 2;
    while primes.len() < nth {
        let mut is_prime = true;

        for test in &primes {
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
