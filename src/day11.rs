use std::collections::HashMap;

const TARGET_A: usize = 25;
const TARGET_B: usize = 75;

fn digits_n(input: i64) -> u32 {
    match input {
        0 => 1,
        i => i.ilog10() + 1,
    }
}

enum TickResult {
    Single(i64),
    Double(i64, i64),
}

fn tick_stone(input: i64) -> TickResult {
    match (input, digits_n(input)) {
        (0, _) => TickResult::Single(1),
        (i, n) if n % 2 == 0 => {
            let log = 10_i64.pow(n / 2);
            let left = i / log;
            let right = i % log;

            TickResult::Double(left, right)
        }
        (i, _) => TickResult::Single(i * 2024),
    }
}

#[derive(Debug)]
struct Field {
    stones: HashMap<i64, usize>,
}

impl Field {
    fn parse(input: &str) -> Self {
        let stones = input
            .split(' ')
            .map(|num| num.trim().parse().expect("invalid num"))
            .map(|num| (num, 1))
            .collect();

        Self { stones }
    }

    fn tick(&mut self) {
        let mut new_rack = HashMap::new();

        for (&stone, count) in &self.stones {
            match tick_stone(stone) {
                TickResult::Single(n) => *new_rack.entry(n).or_default() += count,
                TickResult::Double(a, b) => {
                    *new_rack.entry(a).or_default() += count;
                    *new_rack.entry(b).or_default() += count;
                }
            }
        }

        self.stones = new_rack;
    }

    fn len(&self) -> usize {
        self.stones.values().sum()
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day11.txt").expect("Can't read input");
    let mut field = Field::parse(&data);

    (0..TARGET_A).for_each(|_| field.tick());
    let a = field.len();
    (0..(TARGET_B - TARGET_A)).for_each(|_| field.tick());
    let b = field.len();

    println!("A: {}", a);
    println!("B: {}", b);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digits_n() {
        assert_eq!(1, digits_n(8));
        assert_eq!(2, digits_n(10));
        assert_eq!(5, digits_n(12345));
    }
}
