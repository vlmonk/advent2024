use eyre::Result;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum RunResult {
    Out,
    Loop,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn from_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    fn to(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Player {
    position: Position,
    direction: Direction,
}

#[derive(Debug)]
struct Game {
    width: i32,
    height: i32,
    walls: HashSet<Position>,
    player: Player,
    initial_player: Player,
    moves: HashSet<Player>,
    additional_wall: Option<Position>,
}

impl Game {
    pub fn parse(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut walls = HashSet::new();
        let mut player: Option<Player> = None;

        for (y, line) in input.lines().enumerate() {
            height += 1;

            for (x, c) in line.chars().enumerate() {
                width = (x as i32 + 1).max(width);
                match c {
                    '#' => {
                        let point = Position::from_usize(x, y);
                        walls.insert(point);
                    }
                    '^' => {
                        let point = Position::from_usize(x, y);
                        player = Some(Player {
                            position: point,
                            direction: Direction::Up,
                        });
                    }
                    _ => {}
                }
            }
        }

        let player = player.expect("Player not found in map");
        let initial_player = player.clone();

        Game {
            width,
            height,
            walls,
            player,
            initial_player,
            moves: HashSet::new(),
            additional_wall: None,
        }
    }

    pub fn reset(&mut self) {
        self.player = self.initial_player.clone();
        self.moves = HashSet::new();
        self.additional_wall = None;
    }

    pub fn tick(&mut self) {
        let next_point = self.player.position.to(self.player.direction);

        if self.have_vall(&next_point) {
            self.player.direction = self.player.direction.rotate();
        } else {
            self.player.position = next_point
        }
    }

    pub fn run(&mut self) -> RunResult {
        loop {
            self.moves.insert(self.player.clone());
            self.tick();
            if self.is_out() {
                return RunResult::Out;
            }
            if self.moves.contains(&self.player) {
                return RunResult::Loop;
            }
        }
    }

    pub fn is_out(&self) -> bool {
        self.player.position.x < 0
            || self.player.position.x >= self.width
            || self.player.position.y < 0
            || self.player.position.y >= self.height
    }

    fn have_vall(&self, point: &Position) -> bool {
        let is_match_additional = match self.additional_wall {
            Some(ref wall) => wall == point,
            _ => false,
        };

        self.walls.contains(point) || is_match_additional
    }

    pub fn uniq_positions(&self) -> usize {
        let positions: HashSet<_> = self.moves.iter().map(|m| m.position).collect();
        positions.len()
    }

    pub fn possible_walls(&self) -> impl Iterator<Item = Position> {
        let height = self.height;
        let width = self.width;
        let player = self.initial_player.clone();
        let walls = self.walls.clone();

        (0..height)
            .flat_map(move |y| (0..width).map(move |x| Position::new(x, y)))
            .filter(move |p| !walls.contains(p) && &player.position != p)
    }
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("data/day06.txt")?;
    let mut game = Game::parse(&data);
    let r = game.run();

    dbg!(r);
    println!("{}", game.uniq_positions());

    let abc = game
        .possible_walls()
        .map(|p| {
            game.reset();
            game.additional_wall = Some(p);
            game.run()
        })
        .filter(|r| r == &RunResult::Loop)
        .count();

    dbg!(abc);

    Ok(())
}
