use eyre::Result;
use std::collections::HashSet;

#[derive(Debug)]
struct Rules(HashSet<(i32, i32)>);

impl Rules {
    pub fn parse(input: &str) -> Self {
        let hash = input.lines().map(Self::parse_rule).collect();
        Self(hash)
    }

    fn parse_rule(input: &str) -> (i32, i32) {
        let mut values = input.split('|');
        let a = values.next().and_then(|num| num.parse().ok());
        let b = values.next().and_then(|num| num.parse().ok());

        match (a, b) {
            (Some(a), Some(b)) => (a, b),
            _ => panic!("Incorrect input: {}", input),
        }
    }

    fn is_match(&self, a: i32, b: i32) -> bool {
        self.0.contains(&(a, b))
    }
}

#[derive(Debug)]
struct Produce(Vec<i32>);

impl Produce {
    pub fn parse(input: &str) -> Self {
        let pages = input
            .split(',')
            .filter_map(|num| num.parse().ok())
            .collect();

        Self(pages)
    }

    pub fn is_valid(&self, rules: &Rules) -> bool {
        let total = self.0.len();

        (0..total - 1)
            .flat_map(|a| (a + 1..total).map(move |b| (a, b)))
            .all(|(a, b)| rules.is_match(self.0[a], self.0[b]))
    }

    fn get_middle(&self) -> i32 {
        let idx = self.0.len() / 2;
        self.0[idx]
    }
}

struct Game {
    rules: Rules,
    produce: Vec<Produce>,
}

impl Game {
    pub fn new(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let rules = parts.next().map(Rules::parse);
        let produce = parts
            .next()
            .map(|part| part.lines().map(Produce::parse).collect());

        match (rules, produce) {
            (Some(rules), Some(produce)) => Self { rules, produce },
            _ => panic!("incorrect input"),
        }
    }

    fn solve_a(&self) -> i32 {
        self.produce
            .iter()
            .filter(|p| p.is_valid(&self.rules))
            .map(|p| p.get_middle())
            .sum()
    }

    fn solve_b(&self) -> i32 {
        self.produce
            .iter()
            .filter(|p| !p.is_valid(&self.rules))
            .map(|p| reorder(p, &self.rules))
            .map(|p| p.get_middle())
            .sum()
    }
}

fn reorder(input: &Produce, rules: &Rules) -> Produce {
    let mut items = vec![];
    let mut rest: HashSet<_> = input.0.iter().cloned().collect();

    while !rest.is_empty() {
        let founded = find_head(&rest, rules);
        items.push(founded);
        rest.remove(&founded);
    }

    Produce(items)
}

fn find_head(input: &HashSet<i32>, rules: &Rules) -> i32 {
    let founded = input.iter().find(|&&a| {
        input
            .iter()
            .filter(|&&b| b != a)
            .all(|&b| rules.is_match(a, b))
    });

    *founded.expect("AHAHA")
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("data/day05.txt")?;
    let game = Game::new(&data);

    let a = game.solve_a();
    let b = game.solve_b();

    println!("A: {}\nB: {}", a, b);

    Ok(())
}
