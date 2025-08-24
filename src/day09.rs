use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Copy, PartialEq)]
enum Block {
    Empty,
    File(usize),
}

impl Block {
    fn is_empty(self) -> bool {
        self == Block::Empty
    }

    fn is_block(self) -> bool {
        !self.is_empty()
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

struct BlockIter<'a> {
    current: usize,
    disk: &'a Disk,
}

impl<'a> Iterator for BlockIter<'a> {
    // block id, index, len
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let total = self.disk.blocks.len();

        // first, seek for next block
        let (current, id) = (self.current..total).find_map(|idx| match self.disk.blocks[idx] {
            Block::File(id) => Some((idx, id)),
            _ => None,
        })?;

        // and calculate len of block
        let len = (current..total)
            .take_while(
                |idx| matches!(self.disk.blocks[*idx], Block::File(id_len) if (id_len == id)),
            )
            .count();
        self.current = current + len;
        Some((id, current, len))
    }
}

struct HolesIter<'a> {
    current: usize,
    disk: &'a Disk,
}

impl<'a> Iterator for HolesIter<'a> {
    // index, len
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let total = self.disk.blocks.len();

        // first, seek for next hole
        let current = (self.current..total).find(|idx| self.disk.blocks[*idx].is_empty())?;
        // and calculate len
        let len = (current..total)
            .take_while(|idx| self.disk.blocks[*idx].is_empty())
            .count();
        self.current = current + len;
        Some((current, len))
    }
}

#[derive(Clone)]
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
        }
    }

    pub fn defrag_file(&mut self) {
        let hashed_info: HashMap<usize, (usize, usize)> = self
            .blocks()
            .map(|(id, idx, len)| (id, (idx, len)))
            .collect();

        let max_id = *hashed_info.keys().max().unwrap();

        for id in (0..=max_id).rev() {
            let (idx, len) = hashed_info.get(&id).unwrap();

            // println!("Start moving block {id} (pos: {idx}, len {len})");
            // println!("B: {self}");

            if let Some((h_idx, h_len)) = self.holes().find(|(_, hole_len)| hole_len >= len) {
                if h_idx < *idx {
                    for i in 0..*len {
                        self.blocks[h_idx + i] = Block::File(id);
                        self.blocks[idx + i] = Block::Empty;
                    }
                }
            }
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

    fn blocks(&self) -> BlockIter {
        BlockIter {
            current: 0,
            disk: self,
        }
    }

    fn holes(&self) -> HolesIter {
        HolesIter {
            current: 0,
            disk: self,
        }
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.blocks {
            write!(f, "{b}")?
        }

        Ok(())
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day09.txt").unwrap();
    let mut disk_a = Disk::parse(&input);
    let mut disk_b = disk_a.clone();

    disk_a.defrag();
    disk_b.defrag_file();

    println!("A: {}", disk_a.checksum());
    println!("B: {}", disk_b.checksum());
}
