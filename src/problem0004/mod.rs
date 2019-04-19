pub fn main() -> u32 {
    let mut palindromes :Vec<u32> = vec![];

    for x in 100..999 {
        for y in 100..999 {
            if is_palindrome(x * y) {
                palindromes.push(x * y);
            }
        }
    }

    return *palindromes.iter().max().unwrap();
}

fn is_palindrome(value:u32) -> bool {
    let string = value.to_string();

    return string.eq(&string.chars().rev().collect::<String>());
}
