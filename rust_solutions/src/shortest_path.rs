use crate::graph::Graph;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
trait Dfs {
    fn depth_first_count(&self, src: &str, dst: &str, visited: &mut HashSet<String>) -> i32;
    fn depth_first_shortest_path(&self, node_a: &str, node_b: &str) -> i32;
}

#[allow(dead_code)]
trait Bfs {
    fn breadth_first_count(&self, src: &str, dst: &str, visited: &mut HashSet<String>) -> i32;
    fn breadth_first_shortest_path(&self, node_a: &str, node_b: &str) -> i32;
}

impl Dfs for Graph {
    fn depth_first_count(&self, src: &str, dst: &str, visited: &mut HashSet<String>) -> i32 {
        let mut stack: VecDeque<(String, i32)> = VecDeque::new();
        stack.push_back((src.to_string(), 0));
        while let Some((current, distance)) = stack.pop_back() {
            if current == dst {
                return distance;
            }
            if let Some(neighbors) = self.nodes.get(&current) {
                for neighbor in neighbors {
                    if visited.contains(neighbor) {
                        continue;
                    }
                    visited.insert(neighbor.clone());
                    stack.push_back((neighbor.clone(), distance + 1));
                }
            }
        }
        -1
    }

    fn depth_first_shortest_path(&self, node_a: &str, node_b: &str) -> i32 {
        // Takes in a graph build from am adjacency list, traverses
        // the graph epth-first and returns the number of edges
        // from the source node to the destination node, or -1 if
        // components are not connected.

        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(node_a.to_string());
        let shortest_path: i32 = self.depth_first_count(node_a, node_b, &mut visited);
        shortest_path
    }
}

impl Bfs for Graph {
    fn breadth_first_count(&self, src: &str, dst: &str, visited: &mut HashSet<String>) -> i32 {
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        queue.push_front((src.to_string(), 0));
        while let Some((current, distance)) = queue.pop_front() {
            if current == dst {
                return distance;
            }
            if let Some(neighbors) = self.nodes.get(&current) {
                for neighbor in neighbors {
                    if visited.contains(neighbor) {
                        continue;
                    }
                    visited.insert(neighbor.clone());
                    queue.push_front((neighbor.clone(), distance + 1));
                }
            }
        }
        -1
    }

    fn breadth_first_shortest_path(&self, node_a: &str, node_b: &str) -> i32 {
        // Takes in a graph build from an adjacency list, traverses
        // the graph breadth-first and returns the number of edges
        // from the source node to the destination node, or -1 if
        // components are not connected.

        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(node_a.to_string());
        let shortest_path: i32 = self.breadth_first_count(node_a, node_b, &mut visited);
        shortest_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Our simple undirected graph structure to play with
    //
    // ┌───┐     ┌───┐     ┌───┐
    // │   │     │   │     │   │
    // │ w ├─────┤ x ├─────┤ y │
    // │   │     │   │     │   │
    // └─┬─┘     └───┘     └─┬─┘
    //   │                   │
    //   │                   │
    //   │                   │
    //   │                   │
    //   │                   │
    //   │       ┌───┐     ┌─┴─┐
    //   │       │   │     │   │
    //   └───────┤ v ├─────┤ z │
    //           │   │     │   │
    //           └───┘     └───┘
    //

    fn create_graph() -> Graph {
        let mut graph = Graph::new();
        let _ = graph.add_edge(&["w", "x"]).is_ok();
        let _ = graph.add_edge(&["x", "y"]).is_ok();
        let _ = graph.add_edge(&["z", "y"]).is_ok();
        let _ = graph.add_edge(&["z", "v"]).is_ok();
        let _ = graph.add_edge(&["w", "v"]).is_ok();
        graph.build_graph_from_edges();
        graph
    }

    #[test]
    fn depth_first_shortest_path_test() {
        let graph: Graph = create_graph();
        println!("SHIT {:?}", graph.depth_first_shortest_path("w", "z"));
        assert!(graph.depth_first_shortest_path("w", "z") == 2);
    }

    #[test]
    fn breadth_shortest_path_test() {
        let graph: Graph = create_graph();
        println!("SHIT {:?}", graph.breadth_first_shortest_path("w", "z"));
        assert!(graph.breadth_first_shortest_path("w", "z") == 2);
    }
}
