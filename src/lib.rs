#![feature(map_first_last)]
#![feature(drain_filter)]
#![feature(array_windows)]

use anyhow::{Context, Result};
use std::ops::RangeBounds;

use once_cell::sync::OnceCell;
use regex::Regex;
use std::path::Path;

pub mod grid;
pub mod questions;

pub trait FromProblemInput {
    fn from(lines: &ProblemInput) -> Self;
}

pub trait FromProblemInputLine {
    fn from_line(line: &str) -> Self;
}

/// A trait representing a generic solution to an AoC problem.
pub trait Solution: Send + Sync {
    fn part1(&self, _lines: &ProblemInput) -> String {
        String::new()
    }

    fn part2(&self, _lines: &ProblemInput) -> String {
        String::new()
    }
}

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

    pub fn split<R: RangeBounds<usize>>(&self, range: R) -> ProblemInput {
        // This is not a good way to do this
        let lines: Vec<_> = self
            .lines
            .iter()
            .enumerate()
            .filter(|(idx, _)| range.contains(idx))
            .map(|(_, val)| val)
            .cloned()
            .collect();
        ProblemInput::from(lines)
    }

    pub fn parse<T: FromProblemInput>(&self) -> T {
        FromProblemInput::from(self)
    }
}

fn number_regex() -> &'static Regex {
    static REGEX: OnceCell<Regex> = OnceCell::new();
    REGEX.get_or_init(|| Regex::new(r"\d+").unwrap())
}

impl FromProblemInput for Vec<Vec<i64>> {
    fn from(lines: &ProblemInput) -> Self {
        fn parse_line(line: &str) -> Vec<i64> {
            if line.contains(',') || line.contains(' ') {
                // this is probably a list of numbers
                number_regex()
                    .captures_iter(line)
                    .map(|v| v[0].parse().unwrap())
                    .collect()
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
            .map(|line| parse_line(line.as_str().trim()))
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

impl<T: FromProblemInputLine> FromProblemInput for Vec<T> {
    fn from(lines: &ProblemInput) -> Self {
        lines
            .lines
            .iter()
            .map(|s| T::from_line(s.as_str()))
            .collect()
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

#[derive(Debug, Clone)]
pub struct Digits {
    pub digits: Vec<u32>,
}

impl FromProblemInputLine for Digits {
    fn from_line(line: &str) -> Self {
        let digits = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        Digits { digits }
    }
}
