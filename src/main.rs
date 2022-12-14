#![warn(
bad_style,
unused,
unused_extern_crates,
unused_import_braces,
unused_qualifications,
unused_results
)]

mod helpers;
mod primes;
mod problem0001;
mod problem0002;
mod problem0003;
mod problem0004;
mod problem0005;
mod problem0007;
mod problem0018;
mod problem0067;

fn main() {
    println!("Problem 1: {}", problem0001::main());
    println!("Problem 2: {}", problem0002::main());
    println!("Problem 3: {}", problem0003::main());
    println!("Problem 4: {}", problem0004::main());
    println!("Problem 5: {}", problem0005::main());
    println!("Problem 7: {}", problem0007::main());
    println!("Problem 18: {}", problem0018::main());
    println!("Problem 67: {}", problem0067::main());
}
