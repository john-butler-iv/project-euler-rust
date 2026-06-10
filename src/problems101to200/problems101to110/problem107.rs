// https://projecteuler.net/problem=107

use std::{
    clone,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const GRID_PATH: &str = "./src/problems101to200/problems101to110/problem107_grid.txt";
pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Minimal Network",
        number: 107,
        solve: || core_solve(Graph::from_lines(read_lines(GRID_PATH), ",")),
    }
}

fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let path = Path::new(path);
    let file = match File::open(path) {
        Err(_) => panic!("a valid filepath should have been given"),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("a valid file is given"))
}

fn core_solve(graph: Graph) -> i64 {
    let mut remaining_connections = graph;
    let mut minimum_graph = Graph {
        connection_matrix: vec![
            None;
            remaining_connections.num_nodes * remaining_connections.num_nodes
        ],
        num_nodes: remaining_connections.num_nodes,
        num_edges: 0,
    };

    let total_desired_connections = minimum_graph.num_nodes - 1;

    // TODO sort by weight, and prune list from there
    while minimum_graph.num_edges < total_desired_connections {
        let mut best_connection = None;
        let mut best_index_from = None;
        let mut best_index_to = None;

        for index_from in 0..remaining_connections.num_nodes {
            for index_to in 0..remaining_connections.num_nodes {
                let new_weight = if let Some(new_weight) =
                    remaining_connections.get_weight(index_from, index_to)
                {
                    new_weight
                } else {
                    continue;
                };

                let should_update;
                if best_connection.is_none() {
                    should_update = true;
                } else if let Some(best_weight) = best_connection
                    && best_weight > new_weight
                {
                    should_update = true
                } else {
                    should_update = false;
                }
                if !should_update {
                    continue;
                }

                if minimum_graph.are_connected(index_from, index_to) {
                    continue;
                }

                best_connection = Some(new_weight);
                best_index_from = Some(index_from);
                best_index_to = Some(index_to);
            }
        }

        remaining_connections.remove_connection(
            best_index_from
                .expect("there is always a solution as long as there are enough connections"),
            best_index_to
                .expect("there is always a solution as long as there are enough connections"),
        );
        minimum_graph.set_connection(
            best_index_from
                .expect("there is always a solution as long as there are enough connections"),
            best_index_to
                .expect("there is always a solution as long as there are enough connections"),
            best_connection
                .expect("there is always a solution as long as there are enough connections"),
        );
    }

    (remaining_connections
        .connection_matrix
        .iter()
        .filter_map(|edge| edge.map(|weight| weight as u32))
        .sum::<u32>()
        / 2)
    .into()
}

struct Graph {
    connection_matrix: Vec<Option<u16>>,
    num_nodes: usize,
    num_edges: usize,
}

impl Graph {
    fn from_lines<Iter: Iterator<Item = String>>(lines: Iter, delim: &str) -> Self {
        let mut graph = Graph {
            connection_matrix: Vec::new(),
            num_nodes: 0,
            num_edges: 0,
        };
        let mut iter = lines;

        let first_line = iter.next().expect("The graph is non-empty");
        for weight in first_line.split(delim) {
            if weight == "-" {
                graph.connection_matrix.push(None);
            } else if let Ok(weight) = weight.parse() {
                graph.connection_matrix.push(Some(weight));
                graph.num_edges += 1;
            }
        }

        graph.num_nodes = graph.connection_matrix.len();
        graph
            .connection_matrix
            .reserve_exact(graph.num_nodes * (graph.num_nodes - 1));

        for line in iter {
            for weight in line.split(delim) {
                if weight == "-" {
                    graph.connection_matrix.push(None);
                } else if let Ok(weight) = weight.parse() {
                    graph.connection_matrix.push(Some(weight));
                    graph.num_edges += 1;
                }
            }
        }

        graph.num_edges /= 2;

        graph
    }

    fn get_index(&self, index_from: usize, index_to: usize) -> usize {
        index_from * self.num_nodes + index_to
    }

    fn remove_connection(&mut self, index_from: usize, index_to: usize) {
        let index1 = self.get_index(index_from, index_to);
        let index2 = self.get_index(index_to, index_from);
        self.connection_matrix[index1] = None;
        self.connection_matrix[index2] = None;
        self.num_edges -= 1;
    }

    fn set_connection(&mut self, index_from: usize, index_to: usize, new_weight: u16) {
        let index1 = self.get_index(index_from, index_to);
        let index2 = self.get_index(index_to, index_from);
        if self.connection_matrix[index1].is_none() {
            self.num_edges += 1;
        }
        self.connection_matrix[index1] = Some(new_weight);
        self.connection_matrix[index2] = Some(new_weight);
    }

    fn get_weight(&self, index_from: usize, index_to: usize) -> Option<u16> {
        self.connection_matrix[self.get_index(index_from, index_to)]
    }

    fn are_connected(&self, index_from: usize, index_to: usize) -> bool {
        self.are_connected_inner(index_from, index_to, &mut HashSet::new())
    }

    fn are_connected_inner(
        &self,
        index_from: usize,
        index_to: usize,
        visited_nodes: &mut HashSet<usize>,
    ) -> bool {
        if visited_nodes.contains(&index_from) {
            return false;
        }

        if self.get_weight(index_from, index_to).is_some() {
            return true;
        }

        visited_nodes.insert(index_from);
        for next_index_from in 0..self.num_nodes {
            if self.get_weight(index_from, next_index_from).is_none()
                || visited_nodes.contains(&next_index_from)
                || index_to == next_index_from
            {
                continue;
            }

            if self.are_connected_inner(next_index_from, index_to, visited_nodes) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::problems101to200::problems101to110::problem107::Graph;

    #[test]
    fn toy_example() {
        let lines = [
            "-	16	12	21	-	-	-".into(),
            "16	-	-	17	20	-	-".into(),
            "12	-	-	28	-	31	-".into(),
            "21	17	28	-	18	19	23".into(),
            "-	20	-	18	-	-	11".into(),
            "-	-	31	19	-	-	27".into(),
            "-	-	-	23	11	27	-".into(),
        ]
        .into_iter();
        assert_eq!(super::core_solve(Graph::from_lines(lines, "\t")), 150);
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 0);
    }
}
