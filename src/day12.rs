use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn around(&self) -> impl Iterator<Item = (Self, Direction)> {
        [
            (Self::new(self.x, self.y - 1), Direction::Up),
            (Self::new(self.x + 1, self.y), Direction::Right),
            (Self::new(self.x, self.y + 1), Direction::Down),
            (Self::new(self.x - 1, self.y), Direction::Left),
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Label(char);

impl Label {
    fn new(input: char) -> Self {
        Self(input)
    }
}

#[derive(Debug)]
struct Field(HashMap<Point, Label>);

impl Field {
    fn parse(input: &str) -> Self {
        let mut data = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            let y = y as i32;

            for (x, sym) in line.chars().enumerate() {
                let x = x as i32;
                let label = Label::new(sym);
                let point = Point::new(x, y);

                data.insert(point, label);
            }
        }

        Self(data)
    }

    fn get(&self, point: &Point) -> Option<Label> {
        self.0.get(point).copied()
    }

    fn delete(&mut self, point: &Point) {
        self.0.remove(point);
    }

    fn into_regions(mut self) -> Vec<Region> {
        let mut regions = vec![];
        while let Some(&p) = self.0.keys().next() {
            let region = Region::construct(p, &mut self);
            regions.push(region)
        }

        regions
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Border {
    position: Point,
    btype: Direction,
}

impl Border {
    fn new(position: Point, btype: Direction) -> Self {
        Self { position, btype }
    }
}

#[derive(Debug)]
struct Region {
    label: Label,
    points: HashSet<Point>,
    borders: HashSet<Border>,
}

impl Region {
    pub fn construct(start: Point, pool: &mut Field) -> Self {
        let label = pool.get(&start).expect("can't be empty");

        let mut points: HashSet<Point> = HashSet::new();
        let mut borders: HashSet<Border> = HashSet::new();

        let mut queue = VecDeque::new();

        queue.push_back(start);
        pool.delete(&start);
        points.insert(start);

        while let Some(p) = queue.pop_front() {
            pool.delete(&p);
            points.insert(p);

            for (n, d) in p.around() {
                match (pool.get(&n), points.get(&n)) {
                    (Some(current), _) if current == label => {
                        if !queue.iter().any(|&p| p == n) {
                            queue.push_back(n);
                        }
                    }
                    (_, Some(_)) => {}
                    (_, None) => {
                        let border = Border::new(n, d);
                        borders.insert(border);
                    }
                }
            }
        }

        Self {
            label,
            points,
            borders,
        }
    }

    fn price(&self) -> usize {
        let area = self.points.len();
        let perimeter = self.borders.len();

        area * perimeter
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day12.txt").expect("Invalid input data");
    let field = Field::parse(&data);
    let regions = field.into_regions();

    let a: usize = regions.iter().map(|r| r.price()).sum();
    println!("Task A: {}", a);
}
