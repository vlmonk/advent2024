use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn around(&self) -> impl Iterator<Item = Self> {
        [
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x - 1, self.y),
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

#[derive(Debug)]
struct RegionEntry {
    position: Point,
    borders: usize,
}

impl RegionEntry {
    fn new(position: Point) -> Self {
        Self {
            position,
            borders: 0,
        }
    }
}

#[derive(Debug)]
struct Region(Vec<RegionEntry>);

impl Region {
    pub fn construct(start: Point, pool: &mut Field) -> Self {
        let label = pool.get(&start).expect("can't be empty");

        let mut queue = VecDeque::new();
        let mut entries: Vec<RegionEntry> = vec![];
        queue.push_back(start);

        while let Some(p) = queue.pop_front() {
            let mut entry = RegionEntry::new(p);

            for n in p.around() {
                match pool.get(&n) {
                    Some(current) if current == label => {
                        if !queue.iter().any(|&p| p == n) {
                            queue.push_back(n);
                        }
                    }

                    _ => {
                        let is_current_region = entries.iter().any(|e| e.position == n);
                        if !is_current_region {
                            entry.borders += 1;
                        }
                    }
                }
            }

            entries.push(entry);
            pool.delete(&p);
        }

        Self(entries)
    }

    fn price(&self) -> usize {
        let area = self.0.len();
        let perimeter: usize = self.0.iter().map(|e| e.borders).sum();
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
