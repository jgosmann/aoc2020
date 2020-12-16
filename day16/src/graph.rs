use itertools::Itertools;
use std::collections::HashSet;

type Node = usize;
type Path = Vec<Node>;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectedGraph {
    pub adjancency: Vec<HashSet<Node>>,
}

impl DirectedGraph {
    pub fn new(n_vertices: usize) -> Self {
        Self {
            adjancency: vec![HashSet::new(); n_vertices],
        }
    }

    pub fn add_edge(&mut self, from: Node, to: Node) {
        self.adjancency[from].insert(to);
    }

    pub fn remove_edge(&mut self, from: Node, to: Node) {
        self.adjancency[from].remove(&to);
    }

    pub fn dfs(&self, start: Node, target: Node) -> Option<Path> {
        let mut visited = HashSet::with_capacity(self.adjancency.len());
        let mut stack = vec![(start, 0)];

        while let Some((current_vertex, next_neighbour)) = stack.pop() {
            visited.insert(current_vertex);

            if current_vertex == target {
                stack.push((current_vertex, next_neighbour));
                return Some(stack.iter().map(|&(v, _)| v).collect());
            }

            let next_vertex = self.adjancency[current_vertex]
                .iter()
                .enumerate()
                .skip(next_neighbour)
                .filter(|(_, v)| !visited.contains(v))
                .next();
            if let Some((i, vertex)) = next_vertex {
                stack.push((current_vertex, i + 1));
                stack.push((*vertex, 0));
            }
        }
        None
    }

    // Using Ford-Fulkerson algorithm
    pub fn max_flow(&self, start: Node, end: Node) -> Self {
        let mut graph = self.clone();
        let mut flow = Self::new(self.adjancency.len());
        while let Some(path) = graph.dfs(start, end) {
            for edge in path.iter().tuple_windows::<(&Node, &Node)>() {
                flow.add_edge(*edge.0, *edge.1);
                flow.remove_edge(*edge.1, *edge.0);
                graph.add_edge(*edge.1, *edge.0);
                graph.remove_edge(*edge.0, *edge.1);
            }
        }
        for (i, edges) in flow.adjancency.iter_mut().enumerate() {
            *edges = edges
                .iter()
                .filter(|j| self.adjancency[i].contains(j))
                .copied()
                .collect();
        }
        flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let mut graph = DirectedGraph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(3, 4);
        assert_eq!(graph.dfs(0, 4), Some(vec![0, 1, 3, 4]));
        assert_eq!(graph.dfs(0, 5), None);
    }

    #[test]
    fn test_max_flow() {
        let mut graph = DirectedGraph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 5);
        graph.add_edge(4, 5);
        let flow = graph.max_flow(0, 5);
        let mut expected_flow = DirectedGraph::new(6);
        expected_flow.add_edge(0, 1);
        expected_flow.add_edge(0, 2);
        expected_flow.add_edge(1, 4);
        expected_flow.add_edge(2, 3);
        expected_flow.add_edge(3, 5);
        expected_flow.add_edge(4, 5);
        assert_eq!(flow, expected_flow);
    }
}
