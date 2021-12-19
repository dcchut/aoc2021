use crate::{FromProblemInput, ProblemInput, Solution};
use itertools::Itertools;
use std::collections::HashMap;

pub struct Q12;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Node<'a> {
    Lower(&'a str),
    Upper(&'a str),
}

impl<'a> Node<'a> {
    fn is_lower(&self) -> bool {
        matches!(self, Self::Lower(_))
    }

    fn is_start(&self) -> bool {
        self == &Node::Lower("start")
    }

    fn is_end(&self) -> bool {
        self == &Node::Lower("end")
    }
}

#[derive(Clone, Debug)]
pub struct Path<'a> {
    current_node: Node<'a>,
    node_count: HashMap<Node<'a>, usize>,
}

impl<'a> Path<'a> {
    #[must_use]
    pub fn new(node: Node<'a>) -> Self {
        Self {
            current_node: node,
            node_count: HashMap::from([(node, 1)]),
        }
    }

    #[must_use]
    pub fn count(&self, node: Node<'a>) -> usize {
        self.node_count.get(&node).copied().unwrap_or_default()
    }

    #[must_use]
    pub fn append(&self, node: Node<'a>) -> Self {
        let mut path = self.clone();
        path.current_node = node;
        *path.node_count.entry(node).or_default() += 1;
        path
    }
}

impl<'a> From<&'a str> for Node<'a> {
    fn from(n: &'a str) -> Self {
        if n.chars().next().unwrap().is_uppercase() {
            Node::Upper(n)
        } else {
            Node::Lower(n)
        }
    }
}

pub struct Adj<'a>(HashMap<Node<'a>, Vec<Node<'a>>>);

impl<'a> FromProblemInput<'a> for Adj<'a> {
    fn from(lines: &'a ProblemInput) -> Self {
        let mut adj = HashMap::new();

        for line in lines.iter() {
            let (l, r) = line.split('-').map(Node::from).collect_tuple().unwrap();
            adj.entry(l).or_insert_with(Vec::new).push(r);
            adj.entry(r).or_insert_with(Vec::new).push(l);
        }

        Adj(adj)
    }
}

fn count_paths<F: FnMut(&Path, Node) -> bool>(adj: &Adj, mut f: F) -> usize {
    let mut paths = vec![Path::new(Node::from("start"))];
    let mut count = 0;

    while let Some(path) = paths.pop() {
        if path.current_node.is_end() {
            count += 1;
            continue;
        }

        for &n in &adj.0[&path.current_node] {
            if n.is_start() || f(&path, n) {
                continue;
            }
            paths.push(path.append(n));
        }
    }

    count
}

impl Solution for Q12 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let adj: Adj = lines.parse();
        count_paths(&adj, |path, node| node.is_lower() && path.count(node) > 0).to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let adj: Adj = lines.parse();
        count_paths(&adj, |path, node| {
            node.is_lower()
                && path.count(node) >= 1
                && path
                    .node_count
                    .iter()
                    .any(|(&n, &v)| n.is_lower() && v >= 2)
        })
        .to_string()
    }
}
