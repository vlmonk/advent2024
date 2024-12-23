use std::collections::HashSet;

fn around(current: &Point, grid: &Grid) -> impl Iterator<Item = Point> {
    let mut points = Vec::new();

    if current.0 > 0 {
        points.push(Point::new(current.0 - 1, current.1))
    }

    if current.1 > 0 {
        points.push(Point::new(current.0, current.1 - 1))
    }

    if current.0 < grid.size - 1 {
        points.push(Point::new(current.0 + 1, current.1))
    }

    if current.1 < grid.size - 1 {
        points.push(Point::new(current.0, current.1 + 1))
    }

    points.into_iter()
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    fn parse(input: &str) -> Self {
        let mut iter = input.split(',');
        let x = iter
            .next()
            .and_then(|part| part.parse::<usize>().ok())
            .expect("Invalid input");

        let y = iter
            .next()
            .and_then(|part| part.parse::<usize>().ok())
            .expect("Invalid input");

        Self(x, y)
    }
}

#[derive(Debug)]
struct Grid {
    size: usize,
    corrupted: HashSet<Point>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let corrupted = HashSet::new();
        Self { size, corrupted }
    }

    fn add(&mut self, point: Point) {
        self.corrupted.insert(point);
    }

    fn solve(&self) -> usize {
        Solver::new(self).process()
    }
}

#[derive(Debug)]
struct Solver<'a> {
    grid: &'a Grid,
    visited: HashSet<Point>,
    queue: HashSet<Point>,
}

impl<'a> Solver<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            visited: HashSet::new(),
            queue: HashSet::new(),
        }
    }

    fn process(&mut self) -> usize {
        self.queue.insert(Point::new(0, 0));
        let mut step = 0;

        loop {
            let is_found = self.process_step();
            if is_found {
                break;
            }

            step += 1;
        }

        step
    }

    fn process_step(&mut self) -> bool {
        let mut next_visited = HashSet::new();

        if self.queue.is_empty() {
            return true;
        }

        for point in self.queue.iter() {
            if self.is_target(point) {
                return true;
            }

            self.visited.insert(*point);

            around(point, self.grid)
                .filter(|p| !self.is_visited(p) && !self.is_corrupted(p))
                .for_each(|p| {
                    next_visited.insert(p);
                });
        }

        self.queue = next_visited;

        false
    }

    fn is_corrupted(&self, p: &Point) -> bool {
        self.grid.corrupted.contains(p)
    }

    fn is_visited(&self, p: &Point) -> bool {
        self.visited.contains(p)
    }

    fn is_target(&self, p: &Point) -> bool {
        p.0 == (self.grid.size - 1) && p.1 == (self.grid.size - 1)
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day18.txt").expect("Reading error");
    let points = data.lines().map(Point::parse).collect::<Vec<_>>();

    let mut grid = Grid::new(71);
    points.iter().take(1024).for_each(|p| grid.add(*p));

    let result = grid.solve();
    println!("A: {}", result);
}
