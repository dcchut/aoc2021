use crate::{FromProblemInputLine, ProblemInput, Solution};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

use once_cell::sync::OnceCell;
pub struct Q8;

struct Disp {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl Disp {
    fn words(&self) -> Vec<String> {
        self.inputs
            .iter()
            .cloned()
            .chain(self.outputs.iter().cloned())
            .collect()
    }
}

impl FromProblemInputLine for Disp {
    fn from_line(line: &str) -> Self {
        let parts: Vec<_> = line.split(" | ").collect();
        let inputs: Vec<_> = parts[0].split_whitespace().map(String::from).collect();
        let outputs: Vec<_> = parts[1].split_whitespace().map(String::from).collect();
        Self { inputs, outputs }
    }
}

fn digits() -> &'static HashMap<&'static str, &'static str> {
    static DIGITS: OnceCell<HashMap<&'static str, &'static str>> = OnceCell::new();
    DIGITS.get_or_init(|| {
        HashMap::from([
            ("abcefg", "0"),
            ("cf", "1"),
            ("acdeg", "2"),
            ("acdfg", "3"),
            ("bcdf", "4"),
            ("abdfg", "5"),
            ("abdefg", "6"),
            ("acf", "7"),
            ("abcdefg", "8"),
            ("abcdfg", "9"),
        ])
    })
}

impl Solution for Q8 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let parts: Vec<Disp> = lines.parse();

        parts
            .into_iter()
            .flat_map(|p| p.outputs)
            .filter(|w| [2, 3, 4, 7].contains(&w.len()))
            .count()
            .to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let parts: Vec<Disp> = lines.parse();
        let digits = digits();

        let is_valid_permutation = |words: &[String], p: &[char]| {
            words
                .iter()
                .all(|w| digits.contains_key(permute_sort(w.as_str(), p).as_str()))
        };

        parts
            .into_par_iter()
            .map(|part| {
                let words = part.words();

                let permutation = ('a'..='g')
                    .permutations(7)
                    .find(|p| is_valid_permutation(&words, p))
                    .unwrap();

                part.outputs
                    .iter()
                    .map(|w| digits[permute_sort(w, &permutation).as_str()])
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap()
            })
            .sum::<i64>()
            .to_string()
    }
}

fn permute_sort(w: &str, permutation: &[char]) -> String {
    w.chars()
        .map(|c| permutation[(c as usize) - ('a' as usize)])
        .sorted()
        .collect()
}
