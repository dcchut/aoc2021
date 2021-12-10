use crate::{ProblemInput, Solution};
use itertools::Itertools;

pub struct Q10;

fn left_to_right(c: char) -> char {
    match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => panic!(),
    }
}

fn illegal_score(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn autocomplete_score(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!(),
    }
}

/// Returns a bool indicating whether processing the given character corrupted the chunk.
fn chunk_process(chunk: &mut Vec<char>, c: char) -> bool {
    if ['(', '{', '[', '<'].contains(&c) {
        chunk.push(c);
        return true;
    }

    if let Some(last_open) = chunk.pop() {
        left_to_right(last_open) == c
    } else {
        false
    }
}

impl Solution for Q10 {
fn part1(&self, lines: &ProblemInput) -> String {
    lines
        .lines
        .iter()
        .filter_map(|line| {
            let mut chunk = Vec::new();
            line.chars()
                .find(|c| !chunk_process(&mut chunk, *c))
                .map(illegal_score)
        })
        .sum::<i64>()
        .to_string()
}

    fn part2(&self, lines: &ProblemInput) -> String {
        let scores: Vec<_> = lines
            .lines
            .iter()
            .filter_map(|line| {
                let mut chunk = Vec::new();
                line.chars()
                    .all(|c| chunk_process(&mut chunk, c))
                    .then(|| chunk)
            })
            .map(|chunk| {
                chunk.into_iter().rev().fold(0, |score, c| {
                    (score * 5) + autocomplete_score(left_to_right(c))
                })
            })
            .sorted()
            .collect();

        scores[scores.len() / 2].to_string()
    }
}
