use std::cmp;

pub fn main() -> u32 {
    let mut answer:u32 = 0;

    for x in (100..999).rev() {
        if x < (answer as f64).sqrt() as u32 {
            break;
        }

        for y in (100..999).rev() {
            let mut current = x * y;
            if is_palindrome(current) {
                answer = cmp::max(answer, current);
                break;
            }
        }
    }

    answer
}

fn is_palindrome(value:u32) -> bool {
    let string = value.to_string();

    string.eq(&string.chars().rev().collect::<String>())
}
