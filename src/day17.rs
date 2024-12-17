use std::fs;

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_i64(input: i64) -> Self {
        match input {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid opcode!: {}", input),
        }
    }
}

#[derive(Debug)]
enum Combo {
    Literal(i64),
    RegA,
    RegB,
    RegC,
}

impl Combo {
    fn from_i64(input: i64) -> Self {
        match input {
            n @ 0..=3 => Self::Literal(n),
            4 => Self::RegA,
            5 => Self::RegB,
            6 => Self::RegC,
            _ => panic!("Invalid combo: {}", input),
        }
    }

    fn value(&self, cpu: &Cpu) -> i64 {
        match &self {
            Self::Literal(n) => *n,
            Self::RegA => cpu.reg_a,
            Self::RegB => cpu.reg_b,
            Self::RegC => cpu.reg_c,
        }
    }
}

#[derive(Debug)]
struct Programm(Vec<i64>);

impl Programm {
    fn new(input: Vec<i64>) -> Self {
        Self(input)
    }

    fn parse(input: &str) -> Self {
        let data = input.split(": ").nth(1).expect("invalid input").trim();
        let numbers = data
            .split(',')
            .map(|raw| raw.parse::<i64>().expect("Bad number"))
            .collect();

        Self::new(numbers)
    }

    fn get_inst(&self, index: usize) -> Option<Instruction> {
        self.0.get(index).map(|n| Instruction::from_i64(*n))
    }

    fn get_combo(&self, index: usize) -> Option<Combo> {
        self.0.get(index).map(|n| Combo::from_i64(*n))
    }

    fn get_literal(&self, index: usize) -> Option<i64> {
        self.0.get(index).copied()
    }
}

fn parse_reg(input: &str) -> i64 {
    input
        .split(": ")
        .nth(1)
        .and_then(|raw| raw.parse::<i64>().ok())
        .expect("wrong number")
}

#[derive(Debug, Clone)]
struct Cpu {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,

    ip: usize,
    output: Vec<i64>,
}

impl Cpu {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let reg_a = parse_reg(lines.next().expect("Invalid input"));
        let reg_b = parse_reg(lines.next().expect("Invalid input"));
        let reg_c = parse_reg(lines.next().expect("Invalid input"));

        Self::new(reg_a, reg_b, reg_c)
    }

    fn new(reg_a: i64, reg_b: i64, reg_c: i64) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            ip: 0,
            output: vec![],
        }
    }

    fn run(&mut self, programm: &Programm) {
        loop {
            let result = self.tick(programm);
            if result.is_none() {
                break;
            }
            // if self.output.len() > 0 && self.output[0] != 2 {
            //     break;
            // }
        }
    }

    fn tick(&mut self, programm: &Programm) -> Option<()> {
        let inst = programm.get_inst(self.ip)?;

        match inst {
            Instruction::Bst => {
                let value = programm.get_combo(self.ip + 1)?.value(self) % 8;
                self.reg_b = value;
                self.ip += 2;
            }
            Instruction::Out => {
                let value = programm.get_combo(self.ip + 1)?.value(self) % 8;
                // let value = programm.get_combo(self.ip + 1)?.value(self);
                self.output.push(value);
                self.ip += 2;
            }
            Instruction::Adv => {
                let value = programm.get_combo(self.ip + 1)?.value(self);
                self.reg_a /= 2_i64.pow(value as u32);
                self.ip += 2;
            }
            Instruction::Bdv => {
                let value = programm.get_combo(self.ip + 1)?.value(self);
                self.reg_b = self.reg_a / 2_i64.pow(value as u32);
                self.ip += 2;
            }
            Instruction::Cdv => {
                let value = programm.get_combo(self.ip + 1)?.value(self);
                self.reg_c = self.reg_a / 2_i64.pow(value as u32);
                self.ip += 2;
            }
            Instruction::Jnz => {
                let value = programm.get_literal(self.ip + 1)?;
                if self.reg_a != 0 {
                    self.ip = value as usize;
                } else {
                    self.ip += 2;
                }
            }
            Instruction::Bxl => {
                let value = programm.get_literal(self.ip + 1)?;
                self.reg_b ^= value;
                self.ip += 2;
            }
            Instruction::Bxc => {
                self.reg_b ^= self.reg_c;
                self.ip += 2;
            }
        }

        Some(())
    }

    fn output_str(&self) -> String {
        self.output
            .iter()
            .map(|f| format!("{}", f))
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn parse(input: &str) -> (Cpu, Programm) {
    let mut parts = input.split("\n\n");
    let cpu = Cpu::parse(parts.next().expect("Not found"));
    let programm = Programm::parse(parts.next().expect("Not found"));

    (cpu, programm)
}

fn main() {
    let data = std::fs::read_to_string("data/day17.txt").unwrap();
    let (mut cpu, programm) = parse(&data);
    let orig = cpu.clone();
    // cpu.run(&programm);
    // println!("{}", cpu.output_str());

    // let mut start = 123179200000000000;
    // let step = 100_000_000_000;
    let mut start = 0o7026424520000000;
    let mut step = 1;
    loop {
        let mut cpu = orig.clone();
        cpu.reg_a = start;
        cpu.run(&programm);
        // println!("{} / {:o}-> {}", start, start, cpu.output_str());

        start += step;
        // break;

        if cpu.output == programm.0 {
            println!("{}", start - 1);
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cpu_1() {
        let mut cpu = Cpu::new(0, 0, 9);
        let programm = Programm::new(vec![2, 6]);

        cpu.run(&programm);
        assert_eq!(cpu.reg_b, 1);
    }

    #[test]
    fn test_cpu_2() {
        let mut cpu = Cpu::new(10, 20, 99);
        let programm = Programm::new(vec![5, 0, 5, 1, 5, 4]);

        cpu.run(&programm);
        assert_eq!(cpu.output, [0, 1, 2]);
    }

    #[test]
    fn test_cpu_3() {
        let mut cpu = Cpu::new(2024, 9, 99);
        let programm = Programm::new(vec![0, 1, 5, 4, 3, 0]);

        cpu.run(&programm);
        assert_eq!(cpu.output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(cpu.reg_a, 0);
    }

    #[test]
    fn test_cpu_4() {
        let mut cpu = Cpu::new(2024, 9, 99);
        let programm = Programm::new(vec![0, 1, 5, 4, 3, 0]);

        cpu.run(&programm);
        assert_eq!(cpu.output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(cpu.reg_a, 0);
    }
}
