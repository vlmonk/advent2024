use eyre::Result;

#[derive(Debug)]
struct Report(Vec<i32>);
#[derive(Debug)]
struct Game(Vec<Report>);

struct PairIter<'a> {
    numbers: &'a [i32],
    current: usize,
}

impl<'a> PairIter<'a> {
    pub fn new(numbers: &'a [i32]) -> Self {
        Self {
            numbers,
            current: 0,
        }
    }
}

impl<'a> Iterator for PairIter<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current + 1) < self.numbers.len() {
            let a = self.numbers[self.current];
            let b = self.numbers[self.current + 1];

            self.current += 1;
            Some((a, b))
        } else {
            None
        }
    }
}

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

    fn pairs(&self) -> PairIter {
        PairIter::new(&self.0)
    }

    fn all_up(&self) -> bool {
        self.pairs().all(|(a, b)| a < b)
    }

    fn all_down(&self) -> bool {
        self.pairs().all(|(a, b)| a > b)
    }

    fn safe_margin(&self) -> bool {
        self.pairs().all(|(a, b)| {
            let margin = (a - b).abs();
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
