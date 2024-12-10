use eyre::Result;

#[derive(Debug)]
struct Report(Vec<i32>);
#[derive(Debug)]
struct Game(Vec<Report>);

impl Report {
    pub fn parse(input: &str) -> Result<Self> {
        let numbers = input
            .split(' ')
            .map(|part| part.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(numbers))
    }

    pub fn is_safe(&self) -> bool {
        (self.all_up() || self.all_down()) && self.safe_margin()
    }

    fn all_up(&self) -> bool {
        (0..self.0.len() - 1).all(|idx| self.0[idx] < self.0[idx + 1])
    }

    fn all_down(&self) -> bool {
        (0..self.0.len() - 1).all(|idx| self.0[idx] > self.0[idx + 1])
    }

    fn safe_margin(&self) -> bool {
        (0..self.0.len() - 1).all(|idx| {
            let margin = (self.0[idx] - self.0[idx + 1]).abs();
            (1..=3).contains(&margin)
        })
    }
}

impl Game {
    pub fn parse(input: &str) -> Result<Self> {
        let reports = input
            .lines()
            .map(Report::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(reports))
    }

    pub fn solve_a(&self) -> usize {
        self.0.iter().filter(|rep| rep.is_safe()).count()
    }

    pub fn solve_b(&self) -> i32 {
        42
    }
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("data/day02.txt")?;
    let game = Game::parse(&data)?;

    let a = game.solve_a();
    let b = game.solve_b();

    println!("A: {}\nB: {}", a, b);

    Ok(())
}
