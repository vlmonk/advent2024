use eyre::Result;

#[derive(Debug)]
struct Report(Vec<i32>);
#[derive(Debug)]
struct Game(Vec<Report>);

struct PairIter<'a> {
    numbers: &'a [i32],
    current: usize,
    skip: Option<usize>,
}

impl<'a> PairIter<'a> {
    pub fn new(numbers: &'a [i32], skip: Option<usize>) -> Self {
        let (numbers, skip) = match skip {
            Some(0) => (&numbers[1..], None),
            Some(n) if n == numbers.len() - 1 => (&numbers[..numbers.len() - 1], None),
            _ => (numbers, skip),
        };

        Self {
            numbers,
            current: 0,
            skip,
        }
    }
}

impl<'a> Iterator for PairIter<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let total = self.numbers.len();

        if (self.current + 1) < total {
            let a_idx = self.current;
            let mut b_idx = self.current + 1;

            match self.skip {
                Some(n) if n == b_idx => {
                    self.current += 2;
                    b_idx += 1;
                }
                _ => {
                    self.current += 1;
                }
            }

            Some((self.numbers[a_idx], self.numbers[b_idx]))
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
        (self.all_up(None) || self.all_down(None)) && self.safe_margin(None)
    }

    fn is_safe_skip(&self, i: usize) -> bool {
        (self.all_up(Some(i)) || self.all_down(Some(i))) && self.safe_margin(Some(i))
    }

    pub fn is_safe_loose(&self) -> bool {
        self.is_safe() || (0..self.0.len()).any(|n| self.is_safe_skip(n))
    }

    fn pairs(&self, skip: Option<usize>) -> PairIter {
        PairIter::new(&self.0, skip)
    }

    fn all_up(&self, skip: Option<usize>) -> bool {
        self.pairs(skip).all(|(a, b)| a < b)
    }

    fn all_down(&self, skip: Option<usize>) -> bool {
        self.pairs(skip).all(|(a, b)| a > b)
    }

    fn safe_margin(&self, skip: Option<usize>) -> bool {
        self.pairs(skip).all(|(a, b)| {
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

    pub fn solve_b(&self) -> usize {
        self.0.iter().filter(|rep| rep.is_safe_loose()).count()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter_skip_none() {
        let numbers = vec![1, 2, 3];
        let mut iter = PairIter::new(&numbers, None);
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), Some((2, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_2() {
        let numbers = vec![1, 2, 3];
        let mut iter = PairIter::new(&numbers, Some(0));
        assert_eq!(iter.next(), Some((2, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_3() {
        let numbers = vec![1, 2, 3];
        let mut iter = PairIter::new(&numbers, Some(1));
        assert_eq!(iter.next(), Some((1, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_skip_last() {
        let numbers = vec![1, 2, 3];
        let mut iter = PairIter::new(&numbers, Some(2));
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), None);
    }
}
