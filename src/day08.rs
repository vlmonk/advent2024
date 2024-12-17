use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

struct PointIter {
    x0: i32,
    y0: i32,

    x_step: i32,
    y_step: i32,

    n: i32,
}

impl PointIter {
    fn new(a: &Point, b: &Point) -> Self {
        Self {
            x0: a.x,
            y0: a.y,
            x_step: b.x - a.x,
            y_step: b.y - a.y,
            n: 1,
        }
    }
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x0 + self.x_step * self.n;
        let y = self.y0 + self.y_step * self.n;

        self.n += 1;

        Some(Point { x, y })
    }
}

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
    antennas: HashMap<Freq, Vec<Point>>,
    b_box: BBox,
}

impl Game {
    fn parse(input: &str) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut antennas: HashMap<Freq, Vec<Point>> = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            max_y = max_y.max(y);
            for (x, sym) in line.chars().enumerate() {
                max_x = max_x.max(x);
                let point = Point::new(x, y);
                match sym {
                    '.' => {}
                    f => {
                        let freq = Freq(f);
                        antennas.entry(freq).or_default().push(point);
                    }
                }
            }
        }

        let b_box = BBox::new(max_x, max_y);

        Self { b_box, antennas }
    }

    fn solve_a(&self) -> usize {
        let total: HashSet<Point> = self
            .antennas
            .values()
            .flat_map(|a| PairsIter::new(a))
            .flat_map(|(a, b)| {
                let s1 = PointIter::new(a, b)
                    .take_while(|p| self.b_box.within(p))
                    .take(1);
                let s2 = PointIter::new(b, a)
                    .take_while(|p| self.b_box.within(p))
                    .take(1);
                s1.chain(s2)
            })
            .collect();

        total.len()
    }

    fn solve_b(&self) -> usize {
        let total: HashSet<Point> = self
            .antennas
            .values()
            .flat_map(|a| PairsIter::new(a))
            .flat_map(|(a, b)| {
                let s1 = PointIter::new(a, b).take_while(|p| self.b_box.within(p));
                let s2 = PointIter::new(b, a).take_while(|p| self.b_box.within(p));
                s1.chain(s2)
            })
            .collect();

        // dbg!(&total);
        total.len()
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day08.txt").unwrap();
    let game = Game::parse(&data);

    let a = game.solve_a();
    println!("A: {}", a);

    let b = game.solve_b();
    println!("B: {}", b);
}
