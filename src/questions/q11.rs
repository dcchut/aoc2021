use crate::{FromProblemInput, ProblemInput, Solution};
use std::collections::{HashMap, HashSet};

pub struct Q11;

#[derive(Debug)]
struct Grid {
    levels: HashMap<(i64, i64), i64>,
}

impl Grid {
    fn size(&self) -> usize {
        self.levels.len()
    }

    fn step(&mut self) -> usize {
        // Energy level of each octopus increases by one
        for v in self.levels.values_mut() {
            *v += 1;
        }

        // Flashing
        let mut already_flashed = HashSet::new();
        loop {
            let flashes: Vec<_> = self
                .levels
                .iter()
                .filter(|(node, level)| **level > 9 && !already_flashed.contains(*node))
                .map(|(n, _)| *n)
                .collect();

            if flashes.is_empty() {
                break;
            }

            for node in flashes {
                already_flashed.insert(node);
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        // this also increases the level of the flashing octopus
                        // but we're going to set it to zero later
                        if let Some(level) = self.levels.get_mut(&(node.0 + dx, node.1 + dy)) {
                            *level += 1;
                        }
                    }
                }
            }
        }

        // Reset any octopus that flashed
        for node in &already_flashed {
            *self.levels.get_mut(node).unwrap() = 0;
        }

        already_flashed.len()
    }
}

impl FromProblemInput<'_> for Grid {
    fn from(lines: &ProblemInput) -> Self {
        let mut levels = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let value = c.to_digit(10).unwrap() as i64;
                levels.insert((y as i64, x as i64), value);
            }
        }

        Grid { levels }
    }
}

impl Solution for Q11 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let mut grid: Grid = lines.parse();
        (0..100).map(|_| grid.step()).sum::<usize>().to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let mut grid: Grid = lines.parse();
        (1..)
            .find(|_| grid.step() == grid.size())
            .unwrap()
            .to_string()
    }
}
