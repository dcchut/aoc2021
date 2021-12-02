use crate::{FromProblemInputLine, ProblemInput, Solution};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Movement {
    direction: Direction,
    quantity: i64,
}

impl FromProblemInputLine for Movement {
    fn from_line(line: &str) -> Self {
        let parts: Vec<_> = line.split(' ').collect();
        let direction = Direction::from_str(parts[0]).unwrap();
        let quantity = parts[1].parse().unwrap();

        Movement {
            direction,
            quantity,
        }
    }
}

pub struct Q2;

impl Solution for Q2 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let movements: Vec<Movement> = lines.parse();

        let (h, d) = movements
            .into_iter()
            .fold((0, 0), |(h, d), m| match m.direction {
                Direction::Up => (h, d - m.quantity),
                Direction::Down => (h, d + m.quantity),
                Direction::Forward => (h + m.quantity, d),
            });

        (h * d).to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let movements: Vec<Movement> = lines.parse();

        let (h, d, _) = movements
            .into_iter()
            .fold((0, 0, 0), |(h, d, aim), m| match m.direction {
                Direction::Up => (h, d, aim - m.quantity),
                Direction::Down => (h, d, aim + m.quantity),
                Direction::Forward => (h + m.quantity, d + aim * m.quantity, aim),
            });

        (h * d).to_string()
    }
}
