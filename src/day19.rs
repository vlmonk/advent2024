use std::collections::HashSet;

fn is_possible_partial<'a>(
    collection: &Collection,
    pattern: &'a [char],
    cache: &mut Cache<'a>,
) -> bool {
    // println!("{}", pattern.iter().collect::<String>());
    collection.0.iter().any(|towel| {
        let result = towel.is_match(pattern);
        match result {
            Some(&[]) => true,
            Some(rest) => {
                if cache.has(rest) {
                    false
                } else {
                    let result = is_possible_partial(collection, rest, cache);
                    if !result {
                        cache.add(rest)
                    }
                    result
                }
            }
            None => false,
        }
    })
}

struct Cache<'a>(HashSet<&'a [char]>);

impl<'a> Cache<'a> {
    fn new() -> Self {
        Self(HashSet::new())
    }

    fn add(&mut self, input: &'a [char]) {
        self.0.insert(input);
    }

    fn has(&self, input: &'a [char]) -> bool {
        self.0.contains(input)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pattern(Vec<char>);

#[derive(Debug)]
struct Collection(HashSet<Pattern>);

#[derive(Debug)]
struct Towel(Vec<char>);

impl Pattern {
    fn parse(input: &str) -> Self {
        let chars = input.chars().collect();
        Self(chars)
    }

    fn is_match<'a>(&self, pattern: &'a [char]) -> Option<&'a [char]> {
        let my_len = self.0.len();

        if my_len > pattern.len() {
            None
        } else {
            let matched_part = &pattern[0..my_len];

            if self.0 == matched_part {
                Some(&pattern[my_len..])
            } else {
                None
            }
        }
    }
}

impl Collection {
    fn parse(input: &str) -> Self {
        let patterns = input.split(", ").map(Pattern::parse).collect();
        Self(patterns)
    }

    fn is_possible(&self, towel: &Towel) -> bool {
        let mut cache = Cache::new();
        is_possible_partial(self, &towel.0, &mut cache)
    }
}

impl Towel {
    fn parse(input: &str) -> Self {
        let chars = input.chars().collect();
        Self(chars)
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day19.txt").expect("Invalid file");
    let mut parts = data.split("\n\n");
    let patterns = Collection::parse(parts.next().expect("invalid input"));
    let towels = parts
        .next()
        .expect("invalid input")
        .lines()
        .map(Towel::parse)
        .collect::<Vec<_>>();

    let total = towels.iter().filter(|t| patterns.is_possible(t)).count();
    dbg!(total);
}
