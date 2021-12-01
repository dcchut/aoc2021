use crate::{ProblemInput, Solution};

pub struct Q9;

impl Solution for Q9 {
    fn part1(&self, _lines: &ProblemInput) -> String {
        String::new()
    }

    fn part2(&self, _lines: &ProblemInput) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q9 = Q9;
        assert_eq!(q9.part1(&load_problem_input(9)), 3199139634_i64.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q9 = Q9;
        assert_eq!(q9.part2(&load_problem_input(9)), 438559930_i64.to_string());
    }
}
