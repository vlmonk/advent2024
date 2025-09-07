fn parse_num(input: &str) -> Option<(usize, &str)> {
    let (first_pos, _) = input
        .chars()
        .enumerate()
        .find(|(_, c)| c.is_ascii_digit())?;

    let (last_pos, _) = input[first_pos..]
        .chars()
        .enumerate()
        .take_while(|(_, c)| c.is_ascii_digit())
        .last()?;

    let number = input[first_pos..=first_pos + last_pos]
        .parse::<usize>()
        .ok()?;

    Some((number, &input[first_pos + last_pos + 1..]))
}

#[derive(Debug)]
struct Machine {
    a_x: usize,
    a_y: usize,
    b_x: usize,
    b_y: usize,
    prize_x: usize,
    prize_y: usize,
}

impl Machine {
    pub fn parse(input: &str) -> Self {
        let (a_x, input) = parse_num(input).unwrap();
        let (a_y, input) = parse_num(input).unwrap();

        let (b_x, input) = parse_num(input).unwrap();
        let (b_y, input) = parse_num(input).unwrap();

        let (prize_x, input) = parse_num(input).unwrap();
        let (prize_y, _) = parse_num(input).unwrap();

        Self {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        }
    }

    pub fn solve(&self) -> Option<usize> {
        let range = self.prize_x / self.a_x;
        (0..=range.min(100))
            .filter_map(|a| {
                let a_x = a * self.a_x;
                let rest_x = self.prize_x - a_x;
                let b = rest_x / self.b_x;
                let b_div = rest_x % self.b_x;
                let y = a * self.a_y + b * self.b_y;

                if b <= 100 && b_div == 0 && y == self.prize_y {
                    Some((a, b))
                } else {
                    None
                }
            })
            .map(|(a, b)| a * 3 + b)
            .min()
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();
    let machines = input
        .split("\n\n")
        .map(|part| Machine::parse(part))
        .collect::<Vec<_>>();

    let score: usize = machines.iter().filter_map(|m| m.solve()).sum();
    dbg!(score);
}
