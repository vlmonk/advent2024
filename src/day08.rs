use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

struct PairsIter<'a, T> {
    items: &'a [T],
    a: usize,
    b: usize,
    total: usize,
}

impl<'a, T> PairsIter<'a, T> {
    fn new(items: &'a [T]) -> Self {
        Self {
            items,
            a: 0,
            b: 1,
            total: items.len(),
        }
    }
}

impl<'a, T> Iterator for PairsIter<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.a < self.total && self.b < self.total {
            let item = (&self.items[self.a], &self.items[self.b]);
            self.b += 1;

            if self.b >= self.total {
                self.a += 1;
                self.b = self.a + 1
            }
            Some(item)
        } else {
            None
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Freq(char);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    fn diff(&self, next: &Self) -> Vector {
        Vector::new(next.x - self.x, next.y - self.y)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug)]
struct BBox {
    x: i32,
    y: i32,
}
impl BBox {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    fn within(&self, p: &Point) -> bool {
        p.x >= 0 && p.x <= self.x && p.y >= 0 && p.y <= self.y
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Antenna {
    freq: Freq,
    point: Point,
}
impl Antenna {
    fn new(freq: Freq, point: Point) -> Self {
        Self { freq, point }
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Game {
    antennas: HashSet<Antenna>,
    b_box: BBox,
}

impl Game {
    fn parse(input: &str) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut antennas = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            max_y = max_y.max(y);
            for (x, sym) in line.chars().enumerate() {
                max_x = max_x.max(x);
                let point = Point::new(x, y);
                match sym {
                    '.' => {}
                    f => {
                        let freq = Freq(f);
                        let antenna = Antenna::new(freq, point);
                        antennas.insert(antenna);
                    }
                }
            }
        }

        let b_box = BBox::new(max_x, max_y);

        Self { b_box, antennas }
    }

    fn solve_a(&self) -> usize {
        let mut by_type: HashMap<Freq, Vec<Point>> = HashMap::new();
        let mut antinodes: HashSet<Point> = HashSet::new();

        for antenna in &self.antennas {
            by_type.entry(antenna.freq).or_default().push(antenna.point);
        }

        for (_, antennas) in by_type {
            let pairs = PairsIter::new(&antennas);
            for (a, b) in pairs {
                let vec = a.diff(b);

                let p1 = *a - vec;
                let p2 = *b + vec;

                if self.b_box.within(&p1) {
                    antinodes.insert(p1);
                }

                if self.b_box.within(&p2) {
                    antinodes.insert(p2);
                }
            }
        }

        antinodes.len()
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day08.txt").unwrap();
    let game = Game::parse(&data);

    let a = game.solve_a();
    println!("A: {}", a);
}
