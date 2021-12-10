use crate::{FromProblemInput, ProblemInput, Solution};
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::Dfs;
use petgraph::Undirected;
use std::collections::{HashMap, HashSet};

pub struct Q9;

#[derive(Debug)]
struct Grid<T> {
    graph: StableGraph<T, (), Undirected>,
    pos_to_index: HashMap<(usize, usize), NodeIndex>,
}

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut graph = StableGraph::with_capacity(0, 0);
        let mut pos_to_index = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                let pos = (y, x);
                let n = graph.add_node(T::default());
                pos_to_index.insert(pos, n);

                if x > 0 {
                    graph.add_edge(pos_to_index[&(y, x - 1)], n, ());
                }
                if y > 0 {
                    graph.add_edge(pos_to_index[&(y - 1, x)], n, ());
                }
            }
        }

        Grid {
            graph,
            pos_to_index,
        }
    }
}

impl FromProblemInput for Grid<i64> {
    fn from(lines: &ProblemInput) -> Self {
        let height = lines.lines.len();
        let width = lines.lines[0].len();
        let mut grid = Grid::new(width, height);

        for (y, line) in lines.lines.iter().enumerate() {
            for (x, v) in line.chars().enumerate() {
                let v = v.to_digit(10).unwrap() as i64;
                let n = grid.pos_to_index[&(y, x)];
                if v == 9 {
                    grid.graph.remove_node(n);
                } else if let Some(w) = grid.graph.node_weight_mut(n) {
                    *w = v;
                }
            }
        }

        grid
    }
}

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
        let grid: Grid<i64> = lines.parse();

        let mut s = 0;
        for node in grid.graph.node_indices() {
            let node_value = *grid.graph.node_weight(node).unwrap();

            if grid
                .graph
                .neighbors_undirected(node)
                .all(|nbor| *grid.graph.node_weight(nbor).unwrap() > node_value)
            {
                s += 1 + node_value;
            }
        }

        s.to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let grid: Grid<i64> = lines.parse();

        let mut components = connected_components(&grid.graph);
        components.sort_by_key(|x| std::cmp::Reverse(*x));

        components[0..3].iter().fold(1, |l, r| l * *r).to_string()
    }
}
