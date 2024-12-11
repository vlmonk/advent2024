use eyre::Result;

struct GridIter {
    width: i32,
    height: i32,
    current: i32,
}

impl GridIter {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            current: 0,
        }
    }
}

impl Iterator for GridIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.width * self.height {
            let y = self.current / self.width;
            let x = self.current % self.width;
            let point = Point::new(x, y);
            self.current += 1;
            Some(point)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn all() -> impl Iterator<Item = Self> {
        (-1..=1)
            .flat_map(|y| (-1..=1).map(move |x| (x, y)))
            .map(|p| Direction::new(p.0, p.1))
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn step(&self, d: &Direction, steps: i32) -> Self {
        Self::new(self.x + (d.x * steps), self.y + (d.y * steps))
    }
}

#[derive(Debug)]
struct Game {
    data: Vec<char>,
    width: i32,
    height: i32,
}

fn is_xmas(input: (Option<char>, Option<char>, Option<char>, Option<char>)) -> bool {
    matches!(input, (Some('X'), Some('M'), Some('A'), Some('S')))
}

impl Game {
    fn new(data: &str) -> Self {
        let lines = data.lines();
        let mut width = 0;
        let mut height = 0;
        let mut data = vec![];

        for l in lines {
            height += 1;
            width = 0;
            for c in l.chars() {
                width += 1;
                data.push(c);
            }
        }

        Self {
            data,
            width,
            height,
        }
    }

    fn solve_a(&self) -> usize {
        GridIter::new(self.width, self.height)
            .flat_map(|p| Direction::all().map(move |d| self.get_4(&p, &d)))
            .filter(|abc| is_xmas(*abc))
            .count()
    }

    fn solve_b(&self) -> i32 {
        42
    }

    fn get(&self, p: &Point) -> Option<char> {
        if p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height {
            let idx = p.y * self.width + p.x;
            Some(self.data[idx as usize])
        } else {
            None
        }
    }

    fn get_4(
        &self,
        p: &Point,
        d: &Direction,
    ) -> (Option<char>, Option<char>, Option<char>, Option<char>) {
        (
            self.get(p),
            self.get(&p.step(d, 1)),
            self.get(&p.step(d, 2)),
            self.get(&p.step(d, 3)),
        )
    }
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("data/day04.txt")?;
    let game = Game::new(&data);

    let a = game.solve_a();
    let b = game.solve_b();

    println!("A: {}\nB: {}", a, b);

    Ok(())
}
