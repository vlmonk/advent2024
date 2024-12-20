fn digits_n(input: i32) -> u32 {
    match input {
        0 => 1,
        i => i.ilog10() + 1,
    }
}

fn tick_stone(input: i32) -> [Option<i32>; 2] {
    match (input, digits_n(input)) {
        (0, _) => [Some(1), None],
        (i, n) if n % 2 == 0 => {
            let log = 10_i32.pow(n / 2);
            let left = i / log;
            let right = i % log;

            [Some(left), Some(right)]
        }
        (i, _) => [Some(i * 2024), None],
    }
}

#[derive(Debug)]
struct Field {
    stones: Vec<i32>,
}

impl Field {
    fn parse(input: &str) -> Self {
        let stones = input
            .split(' ')
            .map(|num| num.trim().parse().expect("invalid num"))
            .collect();
        Self { stones }
    }

    fn tick(&mut self) {
        self.stones = self
            .stones
            .iter()
            .flat_map(|n| tick_stone(*n))
            .flatten()
            .collect()
    }

    fn len(&self) -> usize {
        self.stones.len()
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day11.txt").expect("Can't read input");
    let mut field = Field::parse(&data);
    for _ in 0..25 {
        field.tick();
    }

    let a = field.len();
    println!("A: {}", a);
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
