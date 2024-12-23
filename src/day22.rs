use std::collections::HashMap;

const N_COUNT: usize = 2001;

fn step(input: i64) -> i64 {
    let a = input * 64;
    let input = input ^ a;
    let input = input % 16777216;

    let b = input / 32;
    let input = input ^ b;
    let input = input % 16777216;

    let c = input * 2048;
    let input = input ^ c;
    input % 16777216
}

fn step_n(input: i64, n: usize) -> i64 {
    let mut value = input;
    for _ in 0..n {
        value = step(value)
    }

    value
}

fn seq_generator() -> impl Iterator<Item = Seq> {
    (-9..=9).flat_map(move |a| {
        (-9..=9).flat_map(move |b| {
            (-9..=9).flat_map(move |c| (-9..=9).map(move |d| Seq::new(a, b, c, d)))
        })
    })
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Seq(i8, i8, i8, i8);

impl Seq {
    fn new(a: i8, b: i8, c: i8, d: i8) -> Self {
        Self(a, b, c, d)
    }
}

#[derive(Debug)]
struct PriceItem {
    num: i64,
    price: i8,
    change: i8,
}

#[derive(Debug)]
struct SecretNumber {
    last_num: i64,
    changes: HashMap<Seq, i8>,
}

impl SecretNumber {
    fn new(input: i64) -> Self {
        let mut num = input;
        let mut last_price: Option<i8> = None;
        let mut numbers = vec![];

        for _ in 0..N_COUNT {
            let price = (num - (num / 10 * 10)) as i8;
            let change = match last_price {
                Some(n) => price - n,
                None => 0,
            };

            let item = PriceItem { num, price, change };
            numbers.push(item);

            num = step(num);
            last_price = Some(price);
        }

        let last_num = numbers[N_COUNT - 1].num;

        let changes = (1..N_COUNT - 3)
            .map(|idx| {
                let seq = Seq::new(
                    numbers[idx].change,
                    numbers[idx + 1].change,
                    numbers[idx + 2].change,
                    numbers[idx + 3].change,
                );

                let price = numbers[idx + 3].price;
                (seq, price)
            })
            .collect::<HashMap<_, _>>();

        Self { last_num, changes }
    }

    fn find_price(&self, seq: &Seq) -> Option<i8> {
        self.changes.get(seq).copied()
    }
}

struct Buyers(Vec<SecretNumber>);

impl Buyers {
    fn parse(input: &str) -> Self {
        let numbers = input
            .lines()
            .map(|l| l.parse::<i64>().expect("invalid number"))
            .map(SecretNumber::new)
            .collect();

        Self(numbers)
    }

    fn task_a(&self) -> i64 {
        self.0.iter().map(|s| s.last_num).sum()
    }

    fn task_b(&self) -> i64 {
        seq_generator()
            .enumerate()
            .map(|(idx, seq)| {
                // println!("{}", idx);
                self.price_at(&seq)
            })
            .max()
            .unwrap_or_default()
    }

    fn price_at(&self, seq: &Seq) -> i64 {
        self.0
            .iter()
            .map(|num| num.find_price(seq).unwrap_or_default())
            .map(|n| n as i64)
            .sum()
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day22.txt").expect("Can't read");
    let buyers = Buyers::parse(&data);
    println!("A: {}", buyers.task_a());
    println!("B: {}", buyers.task_b());

    // let seq = seq_generator();
}
