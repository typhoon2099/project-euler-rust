pub fn main() -> i32 {
    let mut sum = 0;

    for value in 1..1000 {
        if value % 3 == 0 || value % 5 == 0 {
            sum += value;
        }
    }

    return sum;
}
