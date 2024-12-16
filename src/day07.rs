fn is_match_rest(target: i64, a: i64, b: &[i64]) -> bool {
    if b.is_empty() {
        return target == a;
    }

    if a > target {
        return false;
    }

    let sum = a + b[0];
    let mul = a * b[0];

    is_match_rest(target, sum, &b[1..]) || is_match_rest(target, mul, &b[1..])
}

#[derive(Debug)]
struct Equation(i64, Vec<i64>);

impl Equation {
    pub fn parse(input: &str) -> Self {
        let mut parts = input.split(": ");
        let first = parts.next().expect("Invalid input");
        let rest = parts.next().expect("Invalid input");

        let first = first.parse::<i64>().expect("Invalid number");
        let rest = rest
            .split(" ")
            .map(|f| f.parse::<i64>().expect("Invalid number"))
            .collect();

        Self(first, rest)
    }

    pub fn is_match(&self) -> bool {
        match self.1.len() {
            0 => false,
            1 => self.0 == self.1[0],
            _ => is_match_rest(self.0, self.1[0], &self.1[1..]),
        }
    }
}

#[derive(Debug)]
struct Game(Vec<Equation>);

impl Game {
    pub fn parse(input: &str) -> Self {
        let inner = input.lines().map(Equation::parse).collect();

        Self(inner)
    }

    pub fn solve_a(&self) -> i64 {
        self.0.iter().filter(|e| e.is_match()).map(|e| e.0).sum()
    }

    pub fn solve_b(&self) -> usize {
        42
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day07.txt").unwrap();
    let game = Game::parse(&data);

    let a = game.solve_a();
    let b = game.solve_b();

    println!("A: {}\nB: {}", a, b);
}
