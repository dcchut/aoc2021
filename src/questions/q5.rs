use crate::{FromProblemInputLine, ProblemInput, Solution};
use std::cmp::Ordering;
use std::collections::HashMap;
pub struct Q5;

struct PointIter {
    point: (i64, i64),
    dx: i64,
    dy: i64,
    target: (i64, i64),
}

impl Iterator for PointIter {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.point == self.target {
            return None;
        }

        self.point = (self.point.0 + self.dx, self.point.1 + self.dy);
        Some(self.point)
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    src: (i64, i64),
    dst: (i64, i64),
}

impl Line {
    fn dx(&self) -> i64 {
        match self.dst.0.cmp(&self.src.0) {
            Ordering::Greater => 1,
            Ordering::Equal => 0,
            Ordering::Less => -1,
        }
    }

    fn dy(&self) -> i64 {
        match self.dst.1.cmp(&self.src.1) {
            Ordering::Greater => 1,
            Ordering::Equal => 0,
            Ordering::Less => -1,
        }
    }

    fn iter(&self) -> impl Iterator<Item = (i64, i64)> {
        PointIter {
            point: (self.src.0 - self.dx(), self.src.1 - self.dy()),
            dx: self.dx(),
            dy: self.dy(),
            target: self.dst,
        }
    }

    fn is_diagonal(&self) -> bool {
        self.src.0 != self.dst.0 && self.src.1 != self.dst.1
    }
}

impl FromProblemInputLine for Line {
    fn from_line(line: &str) -> Self {
        let parts: Vec<_> = line.split(" -> ").collect();
        let src: Vec<_> = parts[0]
            .split(',')
            .map(|c| c.parse::<i64>().unwrap())
            .collect();
        let dst: Vec<_> = parts[1]
            .split(',')
            .map(|c| c.parse::<i64>().unwrap())
            .collect();

        Line {
            src: (src[0], src[1]),
            dst: (dst[0], dst[1]),
        }
    }
}

impl Solution for Q5 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let mut lines: Vec<Line> = lines.parse();
        lines.retain(|l| !l.is_diagonal());

        let mut point_map = HashMap::new();
        for line in lines {
            for (x, y) in line.iter() {
                *point_map.entry((x, y)).or_insert(0) += 1;
            }
        }

        point_map.values().filter(|c| **c >= 2).count().to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let lines: Vec<Line> = lines.parse();

        let mut point_map = HashMap::new();
        for line in lines {
            for (x, y) in line.iter() {
                *point_map.entry((x, y)).or_insert(0) += 1;
            }
        }

        point_map.values().filter(|c| **c >= 2).count().to_string()
    }
}
