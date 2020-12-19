use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

type Path<T> = Vec<Rc<T>>;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectedGraph<T: Eq + Hash> {
    pub adjancency: HashMap<Rc<T>, HashSet<Rc<T>>>,
}

impl<T: Debug + Eq + Hash> DirectedGraph<T> {
    pub fn new() -> Self {
        Self {
            adjancency: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: &Rc<T>, to: &Rc<T>) {
        let entry = self
            .adjancency
            .entry(Rc::clone(from))
            .or_insert(HashSet::new());
        entry.insert(Rc::clone(to));
    }

    pub fn remove_edge(&mut self, from: &Rc<T>, to: &Rc<T>) {
        if let Some(edges) = self.adjancency.get_mut(from) {
            edges.remove(to);
        }
    }

    pub fn dfs(&self, start: &Rc<T>, target: &Rc<T>) -> Option<Path<T>> {
        let mut visited = HashSet::with_capacity(self.adjancency.len());
        let mut stack = vec![(start, 0)];

        while let Some((current_vertex, next_neighbour)) = stack.pop() {
            visited.insert(current_vertex);

            if current_vertex == target {
                stack.push((current_vertex, next_neighbour));
                return Some(stack.iter().map(|&(v, _)| Rc::clone(v)).collect());
            }

            let next_vertex = self
                .adjancency
                .get(current_vertex)
                .map(|edges| {
                    edges
                        .iter()
                        .enumerate()
                        .skip(next_neighbour)
                        .filter(|(_, v)| !visited.contains(v))
                        .next()
                })
                .flatten();
            if let Some((i, vertex)) = next_vertex {
                stack.push((current_vertex, i + 1));
                stack.push((vertex, 0));
            }
        }
        None
    }
}

impl<T: Clone + Debug + Eq + Hash> DirectedGraph<T> {
    // Using Ford-Fulkerson algorithm
    pub fn max_flow(&self, start: &Rc<T>, end: &Rc<T>) -> Self {
        let mut graph = self.clone();
        let mut flow = Self::new();
        while let Some(path) = graph.dfs(start, end) {
            for edge in path.iter().tuple_windows::<(&Rc<T>, &Rc<T>)>() {
                flow.add_edge(edge.0, edge.1);
                flow.remove_edge(edge.1, edge.0);
                graph.add_edge(edge.1, edge.0);
                graph.remove_edge(edge.0, edge.1);
            }
        }
        for (vertex, edges) in flow.adjancency.iter_mut() {
            *edges = edges
                .iter()
                .filter(|&j| {
                    self.adjancency
                        .get(vertex)
                        .and_then(|e| Some(e.contains(j)))
                        .unwrap_or(false)
                })
                .cloned()
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
        let nodes: Vec<Rc<u32>> = (0..6).map(Rc::new).collect();
        let mut graph = DirectedGraph::new();
        graph.add_edge(&nodes[0], &nodes[1]);
        graph.add_edge(&nodes[1], &nodes[2]);
        graph.add_edge(&nodes[1], &nodes[3]);
        graph.add_edge(&nodes[3], &nodes[4]);
        assert_eq!(
            graph.dfs(&nodes[0], &nodes[4]),
            Some(
                vec![0, 1, 3, 4]
                    .into_iter()
                    .map(|i| Rc::clone(&nodes[i]))
                    .collect()
            )
        );
        assert_eq!(graph.dfs(&nodes[0], &nodes[5]), None);
    }

    #[test]
    fn test_max_flow() {
        let nodes: Vec<Rc<u32>> = (0..6).map(Rc::new).collect();
        let mut graph = DirectedGraph::new();
        graph.add_edge(&nodes[0], &nodes[1]);
        graph.add_edge(&nodes[0], &nodes[2]);
        graph.add_edge(&nodes[1], &nodes[3]);
        graph.add_edge(&nodes[1], &nodes[4]);
        graph.add_edge(&nodes[2], &nodes[3]);
        graph.add_edge(&nodes[3], &nodes[5]);
        graph.add_edge(&nodes[4], &nodes[5]);
        let flow = graph.max_flow(&nodes[0], &nodes[5]);
        let mut expected_flow = DirectedGraph::new();
        expected_flow.add_edge(&nodes[0], &nodes[1]);
        expected_flow.add_edge(&nodes[0], &nodes[2]);
        expected_flow.add_edge(&nodes[1], &nodes[4]);
        expected_flow.add_edge(&nodes[2], &nodes[3]);
        expected_flow.add_edge(&nodes[3], &nodes[5]);
        expected_flow.add_edge(&nodes[4], &nodes[5]);
        assert_eq!(flow, expected_flow);
    }
}
