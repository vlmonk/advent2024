use std::collections::HashSet;

fn around(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].into_iter()
}

#[derive(Debug, Clone, Copy, Default)]
enum Place {
    Present(i32),
    #[default]
    None,
}

impl Place {
    fn is_start_point(&self) -> bool {
        if let Self::Present(0) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Field {
    width: i32,
    points: Vec<Place>,
}

impl Field {
    pub fn parse(input: &str) -> Self {
        let mut width = 0;
        let mut points = vec![];

        for part in input.lines() {
            for (x, i) in part.chars().enumerate() {
                width = width.max(x as i32 + 1);
                let place = match i {
                    '.' => Place::None,
                    num @ '0'..='9' => {
                        Place::Present(num.to_digit(10).expect("Can't be here") as i32)
                    }
                    _ => panic!("Invalid input"),
                };

                points.push(place);
            }
        }

        Self { width, points }
    }

    pub fn at(&self, x: i32, y: i32) -> Place {
        if x < 0 || y < 0 || x >= self.width {
            return Place::None;
        }

        let idx = (y * self.width + x) as usize;
        self.points.get(idx).copied().unwrap_or_default()
    }

    pub fn score(&self, x: i32, y: i32) -> usize {
        let all = self.available(x, y, 0);
        all.into_iter().collect::<HashSet<_>>().len()
    }

    pub fn available(&self, x: i32, y: i32, target: i32) -> Vec<(i32, i32)> {
        match self.at(x, y) {
            Place::Present(9) if target == 9 => {
                vec![(x, y)]
            }
            Place::Present(n) if target == n => around(x, y)
                .flat_map(|(x, y)| self.available(x, y, target + 1))
                .collect(),
            _ => {
                vec![]
            }
        }
    }

    pub fn start_points(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        let height = self.points.len() as i32 / self.width;
        (0..height)
            .flat_map(move |y| (0..self.width).map(move |x| (x, y)))
            .filter(|(x, y)| self.at(*x, *y).is_start_point())
    }

    pub fn score_a(&self) -> usize {
        self.start_points().map(|(x, y)| self.score(x, y)).sum()
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day10.txt").expect("file not found");
    let field = Field::parse(&data);
    let a = field.score_a();

    println!("A: {}", a);
}
