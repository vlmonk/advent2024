use std::collections::HashMap;

struct Keyboard {
    keys: HashMap<char, (i32, i32)>,
    gap: (i32, i32),
    initial: (i32, i32),
}

impl Keyboard {
    fn numeric() -> Self {
        let mut keys = HashMap::new();

        keys.insert('7', (0, 0));
        keys.insert('8', (1, 0));
        keys.insert('9', (2, 0));

        keys.insert('4', (0, 1));
        keys.insert('5', (1, 1));
        keys.insert('6', (2, 1));

        keys.insert('1', (0, 2));
        keys.insert('2', (1, 2));
        keys.insert('3', (2, 2));

        keys.insert('0', (1, 3));
        keys.insert('A', (2, 3));

        let initial = (2, 3);
        let gap = (0, 3);

        Self { keys, initial, gap }
    }

    fn directional() -> Self {
        let mut keys = HashMap::new();

        keys.insert('^', (1, 0));
        keys.insert('A', (2, 0));

        keys.insert('<', (0, 1));
        keys.insert('v', (1, 1));
        keys.insert('>', (2, 1));

        let initial = (2, 0);
        let gap = (0, 0);

        Self { keys, initial, gap }
    }

    fn get(&self, key: char) -> Option<(i32, i32)> {
        self.keys.get(&key).copied()
    }
}

struct Code<'a> {
    value: &'a str,
}

impl<'a> Code<'a> {
    fn new(value: &'a str) -> Self {
        Self { value }
    }

    fn complexity(&self) -> usize {
        self.l3_code_len() * self.num_value()
    }

    fn l3_code_len(&self) -> usize {
        let numeric = Keyboard::numeric();
        let directional = Keyboard::directional();

        let l1 = process(self.value, &numeric);
        dbg!(&l1.len());

        let l2 = l1
            .into_iter()
            .flat_map(|v| process(&v, &directional))
            .collect::<Vec<_>>();
        dbg!(&l2.len());

        let l3 = l2
            .into_iter()
            .flat_map(|v| process(&v, &directional))
            .collect::<Vec<_>>();
        dbg!(&l3.len());

        l3.into_iter().map(|s| s.len()).min().unwrap_or_default()
    }

    fn num_value(&self) -> usize {
        self.value[0..3].parse().expect("Invalid value")
    }
}

fn variants(
    current: (i32, i32),
    target: (i32, i32),
    keyboard: &Keyboard,
) -> impl Iterator<Item = (char, (i32, i32))> {
    let mut variants = vec![];

    if current.0 < target.0 && keyboard.gap != (current.0 + 1, current.1) {
        variants.push(('>', (current.0 + 1, current.1)));
    }

    if current.0 > target.0 && keyboard.gap != (current.0 - 1, current.1) {
        variants.push(('<', (current.0 - 1, current.1)));
    }

    if current.1 < target.1 && keyboard.gap != (current.0, current.1 + 1) {
        variants.push(('v', (current.0, current.1 + 1)));
    }

    if current.1 > target.1 && keyboard.gap != (current.0, current.1 - 1) {
        variants.push(('^', (current.0, current.1 - 1)));
    }

    variants.into_iter()
}

fn process_key(current: (i32, i32), target: (i32, i32), keyboard: &Keyboard) -> Vec<String> {
    if current == target {
        return vec!["".into()];
    }

    variants(current, target, keyboard)
        .flat_map(|(c, n)| {
            process_key(n, target, keyboard)
                .into_iter()
                .map(move |rest| format!("{}{}", c, rest))
        })
        .collect()
}

fn process_part(current: (i32, i32), keys: &[char], keyboard: &Keyboard) -> Vec<String> {
    if keys.is_empty() {
        return vec!["".into()];
    }

    let key = keys[0];
    let rest = &keys[1..];
    let target = keyboard.get(key).expect("Key not found");

    process_key(current, target, keyboard)
        .into_iter()
        .map(|s| format!("{}A", s))
        .flat_map(move |first| {
            process_part(target, rest, keyboard)
                .into_iter()
                .map(move |last| format!("{}{}", first, last))
        })
        .collect()
}

fn process(value: &str, keyboard: &Keyboard) -> Vec<String> {
    let keys = value.chars().collect::<Vec<_>>();
    let current = keyboard.initial;

    let all = process_part(current, &keys, keyboard);
    let min = all.iter().map(|v| v.len()).min().unwrap_or_default();
    all.into_iter().filter(|v| v.len() == min).collect()
}

fn main() {
    let data = std::fs::read_to_string("data/day21.txt").expect("File not found");
    let codes: Vec<_> = data.lines().map(Code::new).collect();

    // for c in &codes {
    //     println!("{} -> {} / {}", c.value, c.l3_code_len(), c.num_value());
    // }

    let a: usize = codes.iter().map(|c| c.complexity()).sum();
    println!("A: {}", a);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_variants() {
        let keyboard = Keyboard::numeric();
        let current = (0, 0);
        let target = (1, 1);

        let mut iter = variants(current, target, &keyboard);

        assert_eq!(iter.next(), Some(('>', (1, 0))));
        assert_eq!(iter.next(), Some(('v', (0, 1))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_variants_with_blank() {
        let keyboard = Keyboard::numeric();
        let current = (0, 2);
        let target = (2, 4);

        let mut iter = variants(current, target, &keyboard);

        assert_eq!(iter.next(), Some(('>', (1, 2))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_process_vec() {
        let keyboard = Keyboard::numeric();
        let current = (0, 0);
        let target = (1, 1);
        let result = process_key(current, target, &keyboard);

        assert_eq!(result, vec![">v", "v>"]);
    }

    #[test]
    fn test_process_vec_with_ignore() {
        let keyboard = Keyboard::numeric();
        let current = (0, 2);
        let target = (2, 3);
        let result = process_key(current, target, &keyboard);

        assert_eq!(result, vec![">>v", ">v>"]);
    }

    #[test]
    fn test_process_part() {
        let keyboard = Keyboard::numeric();
        let current = (1, 3);
        let keys = vec!['1', '5'];
        let resutl = process_part(current, &keys, &keyboard);

        assert_eq!(resutl, vec!["^<A>^A", "^<A^>A"]);
    }
}
