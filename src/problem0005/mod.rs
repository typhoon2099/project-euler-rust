use std::{collections::HashMap, cmp};

// Create a HashMap containing counts of each prime in a number
// Loop through the HashMap for the number and the running total and take the largest of them.
// This creates a list of all Primes which need multiplying

pub fn main() -> u32 {
    let mut primes = HashMap::new();

    for i in 2..=20 {
        let current_primes = get_primes_as_hashmap(i);

        for prime in current_primes.into_iter() {
            let current_value = primes.entry(prime.0).or_insert(0);
            *current_value = cmp::max(prime.1, *current_value);
        }
    }

    primes
        .into_iter()
        .map(|x| x.0.pow(x.1))
        .product()
}

fn get_primes(number: u32) -> Vec<u32> {
    let mut results = vec![];

    let mut looped_number = number;
    let mut i = 2;
    while i <= looped_number {
        if looped_number % i== 0 {
            results.push(i);
            looped_number /= i;
        } else {
            i += 1;
        }
    }

    results
}

fn get_primes_as_hashmap(number: u32) -> HashMap<u32, u32> {
    let primes = get_primes(number);

    let mut hashmap = HashMap::new();

    for prime in primes {
        let counter = hashmap.entry(prime).or_insert(0);
        *counter += 1;
    }

    hashmap
}
