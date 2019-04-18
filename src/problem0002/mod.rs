pub fn main() -> i32 {
    // Find numbers
    let mut numbers :Vec<i32> = vec![1, 2];

    while numbers.iter().max() < Some(&4000000) {
        let new = numbers[numbers.len() - 1] + numbers[numbers.len() - 2];
        numbers.push(new );
    }

    let mut total:i32 = 0;
    for number in numbers {
        if is_even(number) {
            total += number;
        }
    }

    return total;
}

fn is_even(value:i32) -> bool {
    return value % 2 == 0;
}