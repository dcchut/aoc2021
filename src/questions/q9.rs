use crate::{FromProblemInput, NamedGraph, ProblemInput, Solution};
use petgraph::stable_graph::StableGraph;
use petgraph::visit::Dfs;
use petgraph::Undirected;
use std::collections::HashSet;

pub struct Q9;

#[derive(Debug)]
struct Grid(NamedGraph<i64, (), (usize, usize), Undirected>);

impl FromProblemInput<'_> for Grid {
    fn from(lines: &ProblemInput) -> Self {
        let mut graph = NamedGraph::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let value = c.to_digit(10).unwrap() as i64;
                graph.insert((y, x), value);
                if x > 0 && graph.contains(&(y, x - 1)) {
                    graph.insert_edge(&(y, x), &(y, x - 1), ());
                }
                if y > 0 && graph.contains(&(y - 1, x)) {
                    graph.insert_edge(&(y, x), &(y - 1, x), ());
                }
            }
        }

        Grid(graph)
    }
}

#[must_use]
pub fn connected_components<N, E>(g: &StableGraph<N, E, Undirected>) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    for node in g.node_indices() {
        if visited.contains(&node) {
            continue;
        }
        let mut dfs = Dfs::new(g, node);
        let mut component = 0;
        while let Some(v) = dfs.next(g) {
            visited.insert(v);
            component += 1;
        }
        components.push(component);
    }
    components
}

impl Solution for Q9 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let grid: Grid = lines.parse();

        let mut s = 0;
        for (_, index, &value) in grid.0.nodes_iter() {
            if grid
                .0
                .graph
                .neighbors_undirected(index)
                .all(|n| *grid.0.graph.node_weight(n).unwrap() > value)
            {
                s += 1 + value;
            }
        }

        s.to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let mut grid: Grid = lines.parse();
        grid.0.retain_nodes(|_, _, weight| *weight != 9);

        let mut components = connected_components(&grid.0.graph);
        components.sort_by_key(|&x| std::cmp::Reverse(x));

        components[0..3].iter().fold(1, |l, r| l * *r).to_string()
    }
}
