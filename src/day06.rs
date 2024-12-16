use eyre::Result;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
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

        Game {
            width,
            height,
            walls,
            player,
        }
    }

    pub fn tick(&mut self) {
        let next_point = self.player.position.to(self.player.direction);

        if self.have_vall(&next_point) {
            self.player.direction = self.player.direction.rotate();
        } else {
            self.player.position = next_point
        }
    }

    pub fn is_out(&self) -> bool {
        self.player.position.x < 0
            || self.player.position.x >= self.width
            || self.player.position.y < 0
            || self.player.position.y >= self.height
    }

    fn have_vall(&self, point: &Position) -> bool {
        self.walls.contains(point)
    }
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("data/day06.txt")?;
    let mut game = Game::parse(&data);

    let mut positions = HashSet::new();
    loop {
        positions.insert(game.player.position);
        game.tick();

        if game.is_out() {
            break;
        }
    }

    println!("{}", positions.len());

    Ok(())
}
