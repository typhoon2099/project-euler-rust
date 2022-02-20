pub fn primes(count: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();

    let mut current: u32 = 2;
    while primes.len() < (count as usize) {
        if is_prime(current, &primes) {
            primes.push(current)
        }

        current += 1
    }

    primes
}

fn is_prime(number: u32, primes: &[u32]) -> bool {
    let sqrt = (number as f64).sqrt() as u32;
    for test in primes {
        // No point in testing if we've gone past the square root
        if test > &sqrt {
            return true
        }

        // Stop testing once we find a factor
        if number % test == 0 {
            return false
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        let expected = vec![2, 3, 5, 7];

        assert_eq!(primes(4), expected);
    }
}
