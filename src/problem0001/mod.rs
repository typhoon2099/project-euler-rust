pub fn main() -> u32 {
    let mut sum :u32 = 0;

    for value in 1..1000 {
        if value % 3 == 0 || value % 5 == 0 {
            sum += value;
        }
    }

    sum
}
