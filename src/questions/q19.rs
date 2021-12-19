use defaultmap::DefaultHashMap;
use itertools::Itertools;
use nalgebra::{Matrix3, Vector3};
use petgraph::algo::astar;
use petgraph::Directed;
use std::collections::HashSet;

use crate::{FromProblemInput, NamedGraph, ProblemInput, Skip, Solution};

pub struct Q19;

#[derive(Debug, Clone)]
struct Scanner {
    positions: Vec<Vector3<i64>>,
}

impl FromProblemInput<'_> for Scanner {
    fn from(lines: &ProblemInput) -> Self {
        let result: Vec<Vec<i64>> = lines.split(1..).parse();
        let positions = result.into_iter().map(Vector3::from_vec).collect();
        Scanner { positions }
    }
}

fn sign_vectors() -> [Vector3<i64>; 6] {
    [
        Vector3::from([1, 0, 0]),
        Vector3::from([0, 1, 0]),
        Vector3::from([0, 0, 1]),
        Vector3::from([-1, 0, 0]),
        Vector3::from([0, -1, 0]),
        Vector3::from([0, 0, -1]),
    ]
}

fn bases() -> Vec<Matrix3<i64>> {
    sign_vectors()
        .into_iter()
        .permutations(2)
        .filter(|p| p[0].dot(&p[1]) == 0)
        .map(|p| Matrix3::from_columns(&[p[0], p[1], p[0].cross(&p[1])]).transpose())
        .collect()
}

fn resolve(p: &Scanner, q: &Scanner) -> Option<(Matrix3<i64>, Vector3<i64>)> {
    for base in bases() {
        let mut overlaps = DefaultHashMap::new(0);
        for q_position in &q.positions {
            let q_normalized = base * q_position;
            for p_position in &p.positions {
                let delta = p_position - q_normalized;
                overlaps[(delta.x, delta.y, delta.z)] += 1;
            }
        }
        if let Some((&(x, y, z), _)) = overlaps.iter().find(|(_, v)| **v >= 12) {
            return Some((base, Vector3::new(x, y, z)));
        }
    }
    None
}

fn solve_scanners(lines: &ProblemInput) -> (Vec<Vector3<i64>>, Vec<Vector3<i64>>) {
    let scanners: Vec<Scanner> = lines.parse::<Skip<Scanner>>().unwrap();
    let mut graph: NamedGraph<Scanner, (Matrix3<i64>, Vector3<i64>), usize, Directed> =
        NamedGraph::new();

    for (i, scanner) in scanners.into_iter().enumerate() {
        graph.insert(i, scanner);
    }

    let edges: Vec<_> = graph
        .nodes_iter()
        .permutations(2)
        .filter_map(|v| resolve(v[0].2, v[1].2).map(|w| (v[0].0, v[1].0, w)))
        .collect();

    for (i, j, weight) in edges {
        graph.insert_edge(&i, &j, weight);
    }

    let mut positions = HashSet::new();
    let mut scanners = Vec::new();

    for (i, _, scanner) in graph.nodes_iter() {
        let path = astar(
            &graph.graph,
            graph.get_index(&0).unwrap(),
            |n| n == graph.get_index(&i).unwrap(),
            |_| 1,
            |_| 0,
        )
        .unwrap()
        .1;

        let (p, b) = path.into_iter().tuple_windows().fold(
            (Vector3::zeros(), Matrix3::<i64>::identity()),
            |acc, (l, r)| {
                let weight = graph
                    .graph
                    .edge_weight(graph.graph.find_edge(l, r).unwrap())
                    .unwrap();
                (acc.0 + acc.1 * weight.1, acc.1 * weight.0)
            },
        );
        positions.extend(scanner.positions.iter().map(|v| b * v + p));
        scanners.push(p);
    }

    (positions.into_iter().collect(), scanners)
}

impl Solution for Q19 {
    fn part1(&self, lines: &ProblemInput) -> String {
        solve_scanners(lines).0.len().to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        solve_scanners(lines)
            .1
            .into_iter()
            .tuple_combinations()
            .map(|(l, r)| (l - r).abs().sum())
            .max()
            .unwrap()
            .to_string()
    }
}
