use petgraph::Directed;

use crate::{FromProblemInput, NamedGraph, ProblemInput, Solution};

pub struct Q15;

#[derive(Clone, Debug)]
struct GridGraph(
    NamedGraph<i64, i64, (usize, usize), Directed>,
    (usize, usize),
);

fn add_edges(graph: &mut NamedGraph<i64, i64, (usize, usize), Directed>) {
    let mut edges_to_add = Vec::new();

    for ((x, y), _, &weight) in graph.nodes_iter() {
        if x > 0 {
            edges_to_add.push(((x, y), (x - 1, y), *graph.get((x - 1, y)).unwrap()));
            edges_to_add.push(((x - 1, y), (x, y), weight));
        }
        if y > 0 {
            edges_to_add.push(((x, y), (x, y - 1), *graph.get((x, y - 1)).unwrap()));
            edges_to_add.push(((x, y - 1), (x, y), weight));
        }
    }

    for (src, dst, weight) in edges_to_add {
        graph.insert_edge(src, dst, weight);
    }
}

impl FromProblemInput<'_> for GridGraph {
    fn from(lines: &ProblemInput) -> Self {
        let mut graph = NamedGraph::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let digit = c.to_digit(10).unwrap() as i64;
                graph.insert((x, y), digit);
            }
        }

        GridGraph(graph, (lines.lines[0].len() - 1, lines.len() - 1))
    }
}

impl Solution for Q15 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let mut graph: GridGraph = lines.parse();
        add_edges(&mut graph.0);

        graph
            .0
            .shortest_length_path((0, 0), graph.1)
            .unwrap()
            .to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let GridGraph(mut graph, extent) = lines.parse();

        for x_scale in 0..=4 {
            for y_scale in 0..=4 {
                if x_scale == 0 && y_scale == 0 {
                    continue;
                }

                for x in 0..=extent.0 {
                    for y in 0..=extent.1 {
                        let pos = (x_scale * (extent.0 + 1) + x, y_scale * (extent.1 + 1) + y);
                        let base_weight = *graph.get((x, y)).unwrap();
                        let weight = (base_weight + (x_scale + y_scale) as i64 - 1) % 9 + 1;
                        graph.insert(pos, weight);
                    }
                }
            }
        }
        add_edges(&mut graph);

        graph
            .shortest_length_path((0, 0), graph.max_ident().unwrap())
            .unwrap()
            .to_string()
    }
}
