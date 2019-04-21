pub fn main() -> i32 {
    // Find numbers
    let mut numbers :Vec<i32> = vec![1, 2];

    while numbers.iter().max() < Some(&4000000) {
        let value:i32 = numbers[numbers.len() - 1] + numbers[numbers.len() - 2];
        numbers.push(value);
    }

    numbers.iter().filter(|&x| x % 2 == 0).fold(0, |sum, number| sum + number)
}
