use crate::{ProblemInput, Solution};

pub struct Q3;

// impl FromProblemInput for Vec<Vec<bool>> {
//     fn from(lines: &ProblemInput) -> Vec<Vec<bool>> {
//         lines
//             .lines
//             .iter()
//             .map(|line| line.chars().map(|c| c == '#').collect())
//             .collect()
//     }
// }

impl Solution for Q3 {
    fn part1(&self, _lines: &ProblemInput) -> String {
        String::new()
        // slope_counter(&lines.parse::<Vec<Vec<bool>>>(), 3, 1).to_string()
    }

    fn part2(&self, _lines: &ProblemInput) -> String {
        String::new()
    }
}
