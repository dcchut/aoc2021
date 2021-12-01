#![feature(map_first_last)]

use anyhow::{Context, Result};

use std::path::Path;

pub mod grid;
pub mod questions;

pub fn load_problem_input(number: usize) -> ProblemInput {
    let path = format!("data/q{}.txt", number);
    ProblemInput::new(path).unwrap()
}

pub fn binary_search_by_key<F, T>(low: i64, high: i64, value: T, key: F) -> i64
where
    F: Fn(i64) -> T,
    T: PartialOrd,
{
    let mut low = low;
    let mut high = high;

    while low < high {
        let mid = (low + high) / 2;
        let v = key(mid);

        if v < value {
            low = mid + 1;
        } else if v > value {
            high = mid - 1;
        } else if v == value {
            return mid;
        }
    }

    low
}

#[derive(Debug, Clone)]
pub struct ProblemInput {
    pub lines: Vec<String>,
}

pub trait Digits {
    fn digits(&self) -> Vec<i64>; // TODO: maybe consider a different return type here (usize? digit?)
}

impl Digits for i64 {
    fn digits(&self) -> Vec<i64> {
        self.to_string()
            .chars()
            .map(|v| v.to_digit(10).unwrap() as i64)
            .collect()
    }
}

pub trait FromDigits {
    fn from_digits(&self) -> i64;
}

impl FromDigits for &[i64] {
    fn from_digits(&self) -> i64 {
        self.iter()
            .cloned()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse()
            .unwrap()
    }
}

impl FromDigits for Vec<i64> {
    fn from_digits(&self) -> i64 {
        self.as_slice().from_digits()
    }
}

impl ProblemInput {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // Read our input file into a vector of strings
        let lines: Vec<String> = std::fs::read_to_string(path)
            .with_context(|| format!("unable to load problem input from {}", path.display()))?
            .lines()
            .map(String::from)
            .collect();

        Ok(Self { lines })
    }

    pub fn parse<T: FromProblemInput>(&self) -> T {
        FromProblemInput::from(self)
    }

    pub fn as_csv(&self) -> Vec<String> {
        self.lines
            .iter()
            .map(|line| line.split(',').map(String::from).collect::<Vec<_>>())
            .flatten()
            .collect()
    }

    pub fn digits(&self) -> Vec<u32> {
        self.lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
}

impl FromProblemInput for Vec<Vec<i64>> {
    fn from(lines: &ProblemInput) -> Self {
        fn parse_with_sep(line: &str, sep: char) -> Vec<i64> {
            line.split(sep).map(|v| v.parse().unwrap()).collect()
        }
        fn parse_line(line: &str) -> Vec<i64> {
            if line.contains(',') {
                // parse the line as a comma separated list
                parse_with_sep(line, ',')
            } else if line.contains(' ') {
                // parse the line as a whitespace separated list
                parse_with_sep(line, ' ')
            } else if let Ok(parsed) = line.parse() {
                vec![parsed]
            } else {
                // potentially empty line
                vec![]
            }
        }

        lines
            .lines
            .iter()
            .map(|line| parse_line(line.as_str()))
            .collect()
    }
}

impl FromProblemInput for Vec<i64> {
    fn from(lines: &ProblemInput) -> Self {
        lines
            .parse::<Vec<Vec<i64>>>()
            .into_iter()
            .flatten()
            .collect()
    }
}

impl From<Vec<String>> for ProblemInput {
    fn from(lines: Vec<String>) -> Self {
        Self { lines }
    }
}

impl From<Vec<&str>> for ProblemInput {
    fn from(lines: Vec<&str>) -> Self {
        Self::from(lines.into_iter().map(String::from).collect::<Vec<_>>())
    }
}

pub trait FromProblemInput {
    fn from(lines: &ProblemInput) -> Self;
}

pub trait FromProblemInputLine {
    fn from_line(line: &str) -> Self;
}

impl<T: FromProblemInputLine> FromProblemInput for Vec<T> {
    fn from(lines: &ProblemInput) -> Self {
        lines
            .lines
            .iter()
            .map(|s| T::from_line(s.as_str()))
            .collect()
    }
}

/// A trait representing a generic solution to an AoC problem.
// TODO: might want to be generic over return type
// or perhaps Box<dyn ToString> or something like that.
pub trait Solution: Send + Sync {
    fn part1(&self, _lines: &ProblemInput) -> String {
        String::new()
    }

    fn part2(&self, _lines: &ProblemInput) -> String {
        String::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl FromProblemInputLine for Point {
    fn from_line(line: &str) -> Self {
        let mut split = line.split(", ");

        let part1 = &(split.next().unwrap())[3..];
        let part2 = &(split.next().unwrap())[2..];
        let part3 = split.next().unwrap();
        let part3 = &(part3)[2..(part3.len() - 1)];

        Point::new(
            part1.parse().unwrap(),
            part2.parse().unwrap(),
            part3.parse().unwrap(),
        )
    }
}

/// Helper struct for parsing problem inputs which consist of multiple related inputs,
/// separated by a newline between them.
///
/// # Example usage
/// let parsed: Vec<T> = lines.parse::<Skip<T>>().unwrap();
pub struct Skip<T> {
    t: Vec<T>,
}

impl<T> Skip<T> {
    pub fn unwrap(self) -> Vec<T> {
        self.t
    }
}

impl<T: FromProblemInput> FromProblemInput for Skip<T> {
    fn from(lines: &ProblemInput) -> Self {
        // The idea is that we want to split `lines.lines` at every newline:
        // everything in between should be parsed as problem input.
        Self {
            t: lines
                .lines
                .split(String::is_empty)
                .map(|v| ProblemInput::from(v.to_vec()))
                .map(|pi| T::from(&pi))
                .collect(),
        }
    }
}
