use std::io;

fn score_to_grade(score: i32) -> Option<u8> {
    if !(0..=100).contains(&score) {
        return None;
    }

    // Index by deciles to avoid long if-else chains.
    let table = [1_u8, 1, 1, 1, 1, 1, 2, 3, 4, 5, 5];
    Some(table[(score / 10) as usize])
}

fn main() {
    let mut input = String::new();

    println!("Enter a score (0-100):");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let score: i32 = input.trim().parse().expect("Please enter a number");
    let Some(grade) = score_to_grade(score) else {
        println!("Invalid score!");
        return;
    };

    println!("Five-level grade: {}", grade);
}
