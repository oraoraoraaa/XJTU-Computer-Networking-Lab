use std::io::{ self, Write };

fn combination(n: usize, k: usize) -> u64 {
    if k == 0 || k == n {
        return 1;
    }

    let k = k.min(n - k);
    let mut result = 1_u64;
    for i in 1..=k {
        result = (result * ((n - k + i) as u64)) / (i as u64);
    }
    result
}

fn main() {
    let mut input = String::new();
    print!("Input an integer n: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Input not valid");

    if n == 0 {
        return;
    }

    for row in 0..n {
        let line = (0..=row)
            .map(|col| combination(row, col))
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", line);
    }
}
