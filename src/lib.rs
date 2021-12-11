#![feature(map_first_last)]
#![feature(drain_filter)]
#![feature(array_windows)]

use anyhow::{Context, Result};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::RangeBounds;

use once_cell::sync::OnceCell;
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableGraph;
use petgraph::{EdgeType, Undirected};
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

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.lines.iter().map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
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

#[derive(Default, Debug)]
pub struct NamedGraph<N, E, I, Ty: EdgeType = Undirected> {
    graph: StableGraph<N, E, Ty>,
    index: HashMap<I, NodeIndex>,
}

impl<N, E, I: Hash + Ord + Copy, Ty: EdgeType> NamedGraph<N, E, I, Ty> {
    pub fn new() -> Self {
        Self {
            graph: StableGraph::with_capacity(0, 0),
            index: HashMap::new(),
        }
    }

    pub fn nodes_iter(&self) -> impl Iterator<Item = (I, NodeIndex, &N)> {
        self.index
            .iter()
            .map(|(&ident, &index)| (ident, index, self.get_unchecked(index)))
    }

    pub fn insert<Q>(&mut self, ident: I, weight: N)
    where
        I: Borrow<Q>,
        Q: Hash + Eq,
    {
        if let Some(index) = self.index.get(ident.borrow()) {
            *self.graph.node_weight_mut(*index).unwrap() = weight;
        } else {
            let index = self.graph.add_node(weight);
            self.index.insert(ident, index);
        }
    }

    pub fn get_index<Q>(&self, ident: Q) -> Option<NodeIndex>
    where
        I: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.index.get(&ident).copied()
    }

    pub fn insert_edge<Q1, Q2>(&mut self, ident1: Q1, ident2: Q2, weight: E)
    where
        I: Borrow<Q1> + Borrow<Q2>,
        Q1: Hash + Eq,
        Q2: Hash + Eq,
    {
        let n1 = self.get_index(ident1).unwrap();
        let n2 = self.get_index(ident2).unwrap();
        self.graph.add_edge(n1, n2, weight);
    }

    fn get_unchecked(&self, index: NodeIndex) -> &N {
        self.graph.node_weight(index).unwrap()
    }

    fn get_unchecked_mut(&mut self, index: NodeIndex) -> &mut N {
        self.graph.node_weight_mut(index).unwrap()
    }

    pub fn contains<Q>(&self, ident: Q) -> bool
    where
        I: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.index.contains_key(ident.borrow())
    }

    pub fn remove<Q>(&mut self, ident: Q) -> Option<N>
    where
        I: Borrow<Q>,
        Q: Hash + Eq,
    {
        let index = self.get_index(ident)?;
        self.graph.remove_node(index)
    }

    pub fn get<Q>(&self, ident: Q) -> Option<&N>
    where
        I: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.get_index(ident).map(|index| self.get_unchecked(index))
    }

    pub fn get_mut<Q>(&mut self, ident: Q) -> Option<&mut N>
    where
        I: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.get_index(ident)
            .map(|index| self.get_unchecked_mut(index))
    }

    pub fn retain_nodes<F: FnMut(I, NodeIndex, &N) -> bool>(&mut self, mut f: F) {
        let nodes_to_remove: Vec<_> = self
            .nodes_iter()
            .filter(|(n, ix, w)| !f(*n, *ix, w))
            .map(|(n, ix, _)| (n, ix))
            .collect();

        for (node, index) in nodes_to_remove {
            self.index.remove(&node);
            self.graph.remove_node(index);
        }
    }
}
