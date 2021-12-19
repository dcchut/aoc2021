use crate::{FromProblemInput, ProblemInput, Solution};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Q13;

#[derive(Copy, Clone, Debug)]
enum Fold {
    Y(usize),
    X(usize),
}

impl Fold {
    fn apply(self, points: &mut HashSet<(usize, usize)>) {
        let folder = |(x, y)| -> (usize, usize) {
            match (self, x, y) {
                (Fold::Y(f), x, y) if y >= f => (x, f - (y - f)),
                (Fold::X(f), x, y) if x >= f => (f - (x - f), y),
                _ => (x, y),
            }
        };
        *points = points.iter().copied().map(folder).collect();
    }
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('=') {
            let (_, n) = s.split('=').collect_tuple().unwrap();
            let n: usize = n.parse().unwrap();

            if s.contains("y=") {
                Ok(Fold::Y(n))
            } else {
                Ok(Fold::X(n))
            }
        } else {
            Err(())
        }
    }
}

struct Origami(HashSet<(usize, usize)>, Vec<Fold>);

impl FromProblemInput<'_> for Origami {
    fn from(lines: &ProblemInput) -> Self {
        let mut points = HashSet::new();
        let mut folds = Vec::new();

        for line in lines.iter() {
            if line.is_empty() {
                continue;
            }
            if let Ok(fold) = Fold::from_str(line) {
                folds.push(fold);
            } else {
                let (l, r) = line
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                points.insert((l, r));
            }
        }

        Self(points, folds)
    }
}

impl Solution for Q13 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let Origami(mut points, folds) = lines.parse();
        folds[0].apply(&mut points);

        points.len().to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let Origami(mut points, folds) = lines.parse();
        for fold in folds {
            fold.apply(&mut points);
        }

        let x_max = points.iter().map(|(x, _)| *x).max().unwrap();
        let y_max = points.iter().map(|(_, y)| *y).max().unwrap();

        let mut output = vec![String::new()];
        for y in 0..=y_max {
            let mut line = String::with_capacity(x_max + 1);
            for x in 0..=x_max {
                if points.contains(&(x, y)) {
                    line.push('#');
                } else {
                    line.push(' ');
                }
            }
            output.push(line);
        }

        output.into_iter().join("\n")
    }
}
