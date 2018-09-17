pub fn main() -> u32 {
    let mut primes :Vec<u64> = Vec::new();
    let mut number :u64 = 600851475143;

    let mut current :u64 = 2;

    while current <= number {
        if number % current == 0 {
            primes.push(current);
            number = number / current;
            current = 2;
        } else {
            current += 1;
        }
    }

    let &max :&u64 = primes.iter().max().unwrap();
    return max as u32;
}
