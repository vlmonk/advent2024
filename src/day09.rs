use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
enum Block {
    Empty,
    File(usize),
}

impl Block {
    fn is_empty(self) -> bool {
        self == Block::Empty
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Empty => write!(f, "."),
            Block::File(id) => write!(f, "{id}"),
        }
    }
}

struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    pub fn parse(input: &str) -> Self {
        let mut id = 0;
        let mut blocks = Vec::new();
        let mut is_block = true;

        for i in input.chars() {
            let count = if let Some(i) = i.to_digit(10) {
                i
            } else {
                break;
            };

            let block = if is_block {
                Block::File(id)
            } else {
                Block::Empty
            };

            for _ in 0..count {
                blocks.push(block);
            }

            if is_block {
                id += 1
            }

            is_block = !is_block;
        }

        Self { blocks }
    }

    pub fn defrag(&mut self) {
        let mut a = 0;
        let mut b = self.blocks.len() - 1;

        loop {
            let a_next = (a..=b).find(|i| self.blocks[*i].is_empty());
            let b_next = (a..=b).rev().find(|i| !self.blocks[*i].is_empty());

            a = if let Some(i) = a_next { i } else { break };
            b = if let Some(i) = b_next { i } else { break };

            if a > b {
                break;
            }

            self.blocks[a] = self.blocks[b];
            self.blocks[b] = Block::Empty;

            // println!("S: {self} / {a} <-> {b}");
        }
    }

    pub fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(idx, block)| match block {
                Block::File(id) => Some((idx, id)),
                Block::Empty => None,
            })
            .map(|(idx, value)| idx * value)
            .sum()
    }
}

// 02.111....222.2
// 012345678901234

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.blocks {
            write!(f, "{b}")?
        }

        Ok(())
    }
}

fn main() {
    // let input = "12345";
    let input = std::fs::read_to_string("data/day09.txt").unwrap();
    let mut disk = Disk::parse(&input);
    // println!("{disk}");
    disk.defrag();
    // println!("{disk}");
    println!("A: {}", disk.checksum());
}
