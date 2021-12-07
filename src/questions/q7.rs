use crate::{ProblemInput, Solution};

pub struct Q7;

fn crab_cost<F: Fn(i64, i64) -> i64>(crabs: &[i64], cost: F) -> i64 {
    let min_position = crabs.iter().copied().min().unwrap();
    let max_position = crabs.iter().copied().max().unwrap();

    (min_position..=max_position)
        .map(move |pos| {
            crabs
                .iter()
                .copied()
                .map(|crab_pos| cost(pos, crab_pos))
                .sum::<i64>()
        })
        .min()
        .unwrap()
}

impl Solution for Q7 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let crabs: Vec<i64> = lines.parse();
        crab_cost(&crabs, |p1, p2| (p2 - p1).abs()).to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let crabs: Vec<i64> = lines.parse();
        crab_cost(&crabs, |p1, p2| {
            let n = (p2 - p1).abs();
            n * (n + 1) / 2
        })
        .to_string()
    }
}
