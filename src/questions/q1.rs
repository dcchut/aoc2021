use crate::{ProblemInput, Solution};
use itertools::Itertools;

pub struct Q1;

impl Solution for Q1 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let nums: Vec<i64> = lines.parse();

        nums.into_iter()
            .tuple_windows()
            .filter(|(prev, curr)| curr > prev)
            .count()
            .to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let nums: Vec<i64> = lines.parse();

        nums.into_iter()
            .tuple_windows()
            // Using the problem terminology our variables refer to:
            //
            // 199 | a
            // 200 | _ _
            // 208 | _ _
            // 210 |   d
            .filter(|(a, _, _, d)| d > a)
            .count()
            .to_string()
    }
}
