use rand::Rng;
use std::io;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Scissors,
    Paper,
}

impl Move {
    fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Rock),
            1 => Some(Self::Scissors),
            2 => Some(Self::Paper),
            _ => None,
        }
    }

    fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..3) {
            0 => Self::Rock,
            1 => Self::Scissors,
            _ => Self::Paper,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Rock => "Rock",
            Self::Scissors => "Scissors",
            Self::Paper => "Paper",
        }
    }

    fn beats(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors) | (Self::Scissors, Self::Paper) | (Self::Paper, Self::Rock)
        )
    }
}

fn read_player_move() -> Option<Move> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse::<i32>().ok().and_then(Move::from_int)
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut player_score = 0_u8;
    let mut computer_score = 0_u8;

    println!("Rock-Paper-Scissors Game (First to 2 wins)");
    println!("0 = Rock, 1 = Scissors, 2 = Paper");

    let mut round = 1;
    while player_score < 2 && computer_score < 2 {
        println!("\nRound {}", round);
        println!("Enter your choice:");

        let Some(player) = read_player_move() else {
            println!("Invalid input, this round is counted as a loss.");
            computer_score += 1;
            println!("Score => You: {}, Computer: {}", player_score, computer_score);
            round += 1;
            continue;
        };

        let computer = Move::random(&mut rng);

        println!("You: {}", player.name());
        println!("Computer: {}", computer.name());

        if player == computer {
            println!("Result: Draw");
        } else if player.beats(computer) {
            println!("Result: You win this round!");
            player_score += 1;
        } else {
            println!("Result: Computer wins this round!");
            computer_score += 1;
        }

        println!("Score => You: {}, Computer: {}", player_score, computer_score);
        round += 1;
    }

    println!("\nFinal Result:");
    match player_score.cmp(&computer_score) {
        std::cmp::Ordering::Greater => println!("You win the game!"),
        std::cmp::Ordering::Less => println!("Computer wins the game!"),
        std::cmp::Ordering::Equal => println!("The game is a draw!"),
    }
}
