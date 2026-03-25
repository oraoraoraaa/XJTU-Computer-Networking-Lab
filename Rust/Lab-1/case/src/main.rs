use std::io;

fn swap_case_ascii(ch: char) -> char {
    if ch.is_ascii_lowercase() {
        ch.to_ascii_uppercase()
    } else if ch.is_ascii_uppercase() {
        ch.to_ascii_lowercase()
    } else {
        ch
    }
}

fn main() {
    println!("Input the string:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("String read failed.");

    let converted = input.chars().fold(String::new(), |mut acc, ch| {
        acc.push(swap_case_ascii(ch));
        acc
    });

    println!("String after transformation: {}", converted);
}
