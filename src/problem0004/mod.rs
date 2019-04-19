pub fn main() -> u32 {
    let mut answer:u32 = 0;

    for x in (100..999).rev() {
        for y in (x..999).rev() {
            let mut current = x * y;
            if is_palindrome(current) && current > answer {
                answer = current;
            }
        }
    }

    return answer;
}

fn is_palindrome(value:u32) -> bool {
    let string = value.to_string();

    return string.eq(&string.chars().rev().collect::<String>());
}
