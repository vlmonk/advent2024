use eyre::{eyre, Result};

#[derive(Debug)]
struct Game {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn parse_line(input: &str) -> Option<(i32, i32)> {
    let idx_space = input.find(' ')?;
    let idx_b = input[idx_space..].find(|c| c != ' ')?;

    let a = input[..idx_space].parse::<i32>().ok()?;
    let b = input[(idx_space + idx_b)..].parse::<i32>().ok()?;

    Some((a, b))
}

impl Game {
    pub fn parse(input: &str) -> Result<Self> {
        let mut left = vec![];
        let mut right = vec![];

        for line in input.lines() {
            let (a, b) = parse_line(line).ok_or_else(|| eyre!("Invalid input: {}", line))?;
            left.push(a);
            right.push(b);
        }

        let game = Game { left, right };
        Ok(game)
    }

    pub fn solve_a(&self) -> i32 {
        let mut left = self.left.clone();
        let mut right = self.right.clone();

        left.sort();
        right.sort();

        let total = left.into_iter().zip(right);
        total.map(|(a, b)| (a - b).abs()).sum()
    }
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("data/day01.txt")?;
    let game = Game::parse(&data)?;
    let a = game.solve_a();

    println!("A: {}", a);
    Ok(())
}
