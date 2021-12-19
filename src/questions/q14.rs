use crate::{FromProblemInput, ProblemInput, Solution};
use defaultmap::DefaultHashMap;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Q14;

#[derive(Clone, Debug)]
struct Instructions<'a> {
    template: &'a str,
    rules: HashMap<(char, char), char>,
}

impl<'a> FromProblemInput<'a> for Instructions<'a> {
    fn from(lines: &'a ProblemInput) -> Self {
        let template = lines.lines[0].as_str();

        let mut rules = HashMap::new();
        for line in lines.iter().skip(2) {
            // yuck
            let (l, r) = line.split(" -> ").collect_tuple().unwrap();
            let l1 = l.chars().next().unwrap();
            let l2 = l.chars().nth(1).unwrap();
            rules.insert((l1, l2), r.chars().next().unwrap());
        }
        Self { template, rules }
    }
}

#[derive(Debug, Clone)]
struct PairTracker {
    count: HashMap<(char, char), usize>,
    start: char,
    end: char,
}

impl PairTracker {
    fn new(s: &str) -> Self {
        let chars: Vec<_> = s.chars().collect();
        let mut count = HashMap::new();
        for (l, r) in chars.iter().tuple_windows() {
            *count.entry((*l, *r)).or_default() += 1;
        }

        Self {
            count,
            start: chars[0],
            end: chars[chars.len() - 1],
        }
    }

    fn progress(&mut self, rules: &HashMap<(char, char), char>) {
        let count = std::mem::take(&mut self.count);

        for ((l, r), count) in count {
            let middle = rules[&(l, r)];
            *self.count.entry((l, middle)).or_default() += count;
            *self.count.entry((middle, r)).or_default() += count;
        }
    }

    fn min_max(&self) -> usize {
        // This counts every interior character twice
        let mut char_counter: DefaultHashMap<char, usize> = DefaultHashMap::default();
        for ((l, r), count) in &self.count {
            char_counter[*l] += *count;
            char_counter[*r] += *count;
        }
        // Update the end characters (which never change)
        char_counter[self.start] += 1;
        char_counter[self.end] += 1;

        let max_value = char_counter.values().copied().max().unwrap();
        let min_value = char_counter.values().copied().min().unwrap();
        (max_value - min_value) / 2
    }
}

impl Solution for Q14 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let instructions: Instructions = lines.parse();
        let mut pairs = PairTracker::new(instructions.template);
        for _ in 0..10 {
            pairs.progress(&instructions.rules);
        }
        pairs.min_max().to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let instructions: Instructions = lines.parse();
        let mut pairs = PairTracker::new(instructions.template);
        for _ in 0..40 {
            pairs.progress(&instructions.rules);
        }
        pairs.min_max().to_string()
    }
}
