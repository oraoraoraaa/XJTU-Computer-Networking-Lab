use rand::Rng;
use std::io;

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..4) {
            0 => Self::Add,
            1 => Self::Sub,
            2 => Self::Mul,
            _ => Self::Div,
        }
    }

    fn symbol(self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
        }
    }
}

fn build_problem(rng: &mut impl Rng) -> (i32, i32, Operation, i32) {
    match Operation::random(rng) {
        Operation::Add => {
            let (a, b) = (rng.gen_range(0..=20), rng.gen_range(0..=20));
            (a, b, Operation::Add, a + b)
        }
        Operation::Sub => {
            let (a, b) = (rng.gen_range(0..=20), rng.gen_range(0..=20));
            let (x, y) = if a >= b { (a, b) } else { (b, a) };
            (x, y, Operation::Sub, x - y)
        }
        Operation::Mul => {
            let (a, b) = (rng.gen_range(0..=10), rng.gen_range(0..=10));
            (a, b, Operation::Mul, a * b)
        }
        Operation::Div => {
            let divisor = rng.gen_range(1..=10);
            let quotient = rng.gen_range(0..=10);
            let dividend = divisor * quotient;
            (dividend, divisor, Operation::Div, quotient)
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let (a, b, operation, answer) = build_problem(&mut rng);

    println!("Solve the problem:");
    println!("{} {} {} = ?", a, operation.symbol(), b);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    let user_answer = input.trim().parse::<i32>().ok();

    if user_answer == Some(answer) {
        println!("Correct!");
    } else {
        println!("Wrong. The correct answer is {}", answer);
    }
}
