use std::io::Cursor;

trait KeyboardLike {
    fn gap(&self) -> (i32, i32);
    fn get(&self, c: char) -> (i32, i32);
    fn initial(&self) -> (i32, i32) {
        self.get('A')
    }
}

struct NumKeypad;
struct DirKeypad;

impl KeyboardLike for NumKeypad {
    fn gap(&self) -> (i32, i32) {
        (0, 3)
    }

    fn get(&self, c: char) -> (i32, i32) {
        match c {
            'A' => (2, 3),
            '0' => (1, 3),
            '1' => (0, 2),
            '2' => (1, 2),
            '3' => (2, 2),
            '4' => (0, 1),
            '5' => (1, 1),
            '6' => (2, 1),
            '7' => (0, 0),
            '8' => (1, 0),
            '9' => (2, 0),
            _ => panic!("Invalid char"),
        }
    }
}

impl KeyboardLike for DirKeypad {
    fn gap(&self) -> (i32, i32) {
        (0, 0)
    }

    fn get(&self, c: char) -> (i32, i32) {
        match c {
            'A' => (2, 0),
            '^' => (1, 0),
            '>' => (2, 1),
            'v' => (1, 1),
            '<' => (0, 1),
            _ => panic!("Invalid char"),
        }
    }
}

const NUM: NumKeypad = NumKeypad;
const DIR: DirKeypad = DirKeypad;

#[derive(Clone, Debug)]
struct Keypress(Vec<char>);
impl Keypress {
    fn new(input: Vec<char>) -> Self {
        Self(input)
    }
}

#[derive(Debug, Clone)]
struct Seq(Vec<Keypress>);

impl Seq {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn multiply(&self, keys: Vec<Keypress>) -> Vec<Self> {
        keys.into_iter()
            .map(|kp| {
                let mut me = self.clone();
                me.add(kp);
                me
            })
            .collect()
    }

    pub fn add(&mut self, kp: Keypress) {
        self.0.push(kp)
    }
}

fn initial(input: &str) -> Vec<Seq> {
    let positions = input.chars().map(|c| NUM.get(c)).collect::<Vec<_>>();
    let mut current = NUM.initial();
    let mut out = vec![Seq::empty()];

    for i in 0..positions.len() {
        let keys = process_key(current, positions[i], &NUM);
        let keypress = keys.into_iter().map(Keypress::new).collect::<Vec<_>>();
        out = out
            .into_iter()
            .flat_map(|o| o.multiply(keypress.clone()))
            .collect();

        current = positions[i];
    }

    out
}

fn expand(input: &[char], current: (i32, i32)) -> Vec<Vec<Vec<char>>> {
    if input.is_empty() {
        return vec![vec![]];
    }

    dbg!(input, current);
    let first = input[0];
    let target = DIR.get(first);
    let keys = process_key(current, target, &NUM);
    dbg!(&keys);
    let rest = expand(&input[1..], target);
    dbg!(&rest);

    let mut output = vec![];

    for k in keys {
        for mut r in rest.clone() {
            let mut element = vec![k.clone()];
            element.append(&mut r);
            output.push(element);
        }
    }

    output
}

// fn step(input: Vec<String>, keyboard: &Keyboard) -> Vec<String> {
//     let all = input
//         .into_iter()
//         .flat_map(|v| process(&v, keyboard))
//         .collect::<Vec<_>>();

//     let before_filter = all.len();

//     let min_cost = all.iter().map(|v| cost(v)).min().unwrap_or_default();
//     let selected: Vec<_> = all
//         .into_iter()
//         .filter(|v| cost(v) == min_cost)
//         .take(1)
//         .collect();

//     let after_filter = selected.len();

//     println!("B: {}, A: {}", before_filter, after_filter);

//     dbg!(&selected);
//     selected
// }

// fn cost(input: &str) -> usize {
//     // dbg!(input);
//     let mut chars = vec!['A'];
//     for c in input.chars() {
//         chars.push(c)
//     }
//     chars.push('A');

//     let total = chars.len();

//     (0..total - 1)
//         .map(|i| match (chars[i], chars[i + 1]) {
//             ('A', 'A') => 0,
//             ('A', '^') => 1,
//             ('A', '>') => 1,
//             ('A', 'v') => 2,
//             ('A', '<') => 3,
//             ('^', 'A') => 1,
//             ('^', '^') => 0,
//             ('^', '>') => 2,
//             ('^', 'v') => 1,
//             ('^', '<') => 2,
//             ('>', 'A') => 1,
//             ('>', '^') => 2,
//             ('>', '>') => 0,
//             ('>', 'v') => 1,
//             ('>', '<') => 2,
//             ('v', 'A') => 2,
//             ('v', '^') => 1,
//             ('v', '>') => 1,
//             ('v', 'v') => 0,
//             ('v', '<') => 1,
//             ('<', 'A') => 3,
//             ('<', '^') => 2,
//             ('<', '>') => 2,
//             ('<', 'v') => 1,
//             ('<', '<') => 0,
//             (_, _) => panic!("Invalid input"),
//         })
//         .sum()
// }

// struct Keyboard {
//     keys: HashMap<char, (i32, i32)>,
//     gap: (i32, i32),
//     initial: (i32, i32),
// }

// struct Code<'a> {
//     value: &'a str,
// }

// impl<'a> Code<'a> {
//     fn new(value: &'a str) -> Self {
//         Self { value }
//     }

//     fn complexity(&self) -> usize {
//         self.l3_code_len() * self.num_value()
//     }

//     fn l3_code_len(&self) -> usize {
//         let numeric = Keyboard::numeric();
//         let directional = Keyboard::directional();

//         let mut l1 = process(self.value, &numeric);

//         for _ in 0..2 {
//             l1 = step(l1, &directional);
//         }

//         l1.into_iter().map(|s| s.len()).min().unwrap_or_default()
//     }

//     fn num_value(&self) -> usize {
//         self.value[0..3].parse().expect("Invalid value")
//     }
// }

fn variants(
    current: (i32, i32),
    target: (i32, i32),
    keyboard: &impl KeyboardLike,
) -> impl Iterator<Item = (char, (i32, i32))> {
    let mut variants = vec![];

    if current.1 < target.1 && keyboard.gap() != (current.0, current.1 + 1) {
        variants.push(('v', (current.0, current.1 + 1)));
    }

    if current.1 > target.1 && keyboard.gap() != (current.0, current.1 - 1) {
        variants.push(('^', (current.0, current.1 - 1)));
    }

    if current.0 < target.0 && keyboard.gap() != (current.0 + 1, current.1) {
        variants.push(('>', (current.0 + 1, current.1)));
    }

    if current.0 > target.0 && keyboard.gap() != (current.0 - 1, current.1) {
        variants.push(('<', (current.0 - 1, current.1)));
    }

    variants.into_iter()
}

fn process_key(
    current: (i32, i32),
    target: (i32, i32),
    keyboard: &impl KeyboardLike,
) -> Vec<Vec<char>> {
    if current == target {
        return vec![vec![]];
    }

    variants(current, target, keyboard)
        .flat_map(|(c, n)| {
            process_key(n, target, keyboard)
                .into_iter()
                .map(move |mut rest| {
                    let mut total = vec![c];
                    total.append(&mut rest);
                    total
                })
        })
        .collect()
}

// fn process_part(before: String, current: (i32, i32), keys: &[char], keyboard: &Keyboard) -> String {
//     if keys.is_empty() {
//         return "".into();
//     }

//     let key = keys[0];
//     let rest = &keys[1..];
//     let target = keyboard.get(key).expect("Key not found");

//     let mut all_variants = process_key(current, target, keyboard);

//     all_variants.sort_by_key(|v| {
//         let total = format!("{}{}A", before, v);
//         cost(&total)
//     });

//     let selected_variant = all_variants[0].clone();
//     let prefix = format!("{}{}", before, selected_variant);
//     let rest_part = process_part(prefix, target, rest, keyboard);

//     return format!("{}A{}", selected_variant, rest_part);

//     // process_key(current, target, keyboard)
//     //     .into_iter()
//     //     .map(|s| format!("{}A", s))
//     //     .flat_map(move |first| {
//     //         dbg!(&first);
//     //         process_part(before.clone(), target, rest, keyboard)
//     //             .into_iter()
//     //             .map(move |last| format!("{}{}", first, last))
//     //     })
//     //     .collect()
// }

// fn process(value: &str, keyboard: &Keyboard) -> Vec<String> {
//     let keys = value.chars().collect::<Vec<_>>();
//     let current = keyboard.initial;

//     let all = process_part("".into(), current, &keys, keyboard);
//     // let min = all.iter().map(|v| v.len()).min().unwrap_or_default();
//     // all.into_iter().filter(|v| v.len() == min).collect()
//     vec![all]
// }

fn main() {
    let input = "029A";
    let output = initial(input);
    dbg!(&output);

    // let first = &output[0].0[0].0;
    // let expanded = expand(&['<', 'A'], DIR.get('A'));
    // dbg!(&expanded);
}
