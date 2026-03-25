use std::io;

fn term(k: i32) -> f64 {
    let x = k as f64;
    1.0 / (x * x)
}

fn harmonic_square_sum(n: i32) -> f64 {
    (1..=n).map(term).sum()
}

fn main() {
    let mut input = String::new();

    println!("Enter a positive integer n:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let n: i32 = input.trim().parse().expect("Please enter a valid number");
    let sum = if n >= 1 { harmonic_square_sum(n) } else { 0.0 };

    println!("The sum S is: {}", sum);
}
