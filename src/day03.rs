use eyre::Result;

#[derive(PartialEq, Debug)]
struct Mul(i32, i32);

impl Mul {
    pub fn result(&self) -> i32 {
        self.0 * self.1
    }
}

fn parse_tag<'a>(input: &'a str, tag: &str) -> Option<((), &'a str)> {
    if input.len() >= tag.len() && &input[0..tag.len()] == tag {
        Some(((), &input[tag.len()..]))
    } else {
        None
    }
}

fn parse_num(input: &str) -> Option<(i32, &str)> {
    let digits = input.chars().take_while(|c| c.is_ascii_digit()).count();
    let digits = digits.min(3);

    if digits > 0 {
        let value = input[..digits].parse::<i32>().expect("can't be here");
        Some((value, &input[digits..]))
    } else {
        None
    }
}

fn parse_mul(input: &str) -> Option<(Mul, &str)> {
    let (_, input) = parse_tag(input, "mul(")?;
    let (a, input) = parse_num(input)?;
    let (_, input) = parse_tag(input, ",")?;
    let (b, input) = parse_num(input)?;
    let (_, input) = parse_tag(input, ")")?;

    Some((Mul(a, b), input))
}

struct Game<'a>(&'a str);

impl<'a> Game<'a> {
    pub fn new(input: &'a str) -> Self {
        Self(input)
    }

    pub fn solve_a(&self) -> i32 {
        let mut items = vec![];
        let mut data = self.0;

        while data.len() > 0 {
            let result = parse_mul(data);

            match result {
                Some((mul, rest)) => {
                    data = rest;
                    items.push(mul);
                }
                None => data = &data[1..],
            }
        }

        items.iter().map(|i| i.result()).sum()
    }

    pub fn solve_b(&self) -> i32 {
        42
    }
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("data/day03.txt")?;
    let game = Game::new(&data);

    let a = game.solve_a();
    let b = game.solve_b();

    println!("A: {}\nB: {}", a, b);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_pre() {
        assert_eq!(parse_tag("abc", "mul("), None);
        assert_eq!(parse_tag("mul(", "mul("), Some(((), "")));
        assert_eq!(parse_tag("mul(123)", "mul("), Some(((), "123)")));
    }

    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num("abc"), None);
        assert_eq!(parse_num("1"), Some((1, "")));
        assert_eq!(parse_num("1abc"), Some((1, "abc")));
        assert_eq!(parse_num("123abc"), Some((123, "abc")));
        assert_eq!(parse_num("12345"), Some((123, "45")));
    }

    #[test]
    fn test_parse_mul() {
        assert_eq!(parse_mul("foobar"), None);
        assert_eq!(parse_mul("mul(10,11)abc"), Some((Mul(10, 11), "abc")));
    }
}
