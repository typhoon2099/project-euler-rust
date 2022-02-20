use primes;

pub fn main() -> u32 {
    let prime_vec = primes::primes(10_001);

    *prime_vec.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(main(), 104743);
    }
}
