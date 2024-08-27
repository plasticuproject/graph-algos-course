use crate::graph::Graph;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
trait Dfs {
    fn depth_first_has_path_recursive(
        &self,
        src: &str,
        dst: &str,
        visited: &mut HashSet<String>,
    ) -> bool;
    fn undirected_path_depth_first(&self, node_a: &str, node_b: &str) -> bool;
}

#[allow(dead_code)]
trait Bfs {
    fn breadth_first_has_path(&self, src: &str, dst: &str, visited: &mut HashSet<String>) -> bool;
    fn undirected_path_breadth_first(&self, node_a: &str, node_b: &str) -> bool;
}

impl Dfs for Graph {
    fn depth_first_has_path_recursive(
        &self,
        src: &str,
        dst: &str,
        visited: &mut HashSet<String>,
    ) -> bool {
        if src == dst {
            return true;
        }
        if visited.contains(src) {
            return false;
        }
        visited.insert(src.to_string());
        if let Some(neighbors) = self.nodes.get(src) {
            for neighbor in neighbors {
                if Some(self.depth_first_has_path_recursive(neighbor, dst, visited)) == Some(true) {
                    return true;
                }
            }
        }
        false
    }

    fn undirected_path_depth_first(&self, node_a: &str, node_b: &str) -> bool {
        //Undirected depth first has-path recursive algo
        //with cyclical checks from edge list input.

        let mut visited: HashSet<String> = HashSet::new();
        self.depth_first_has_path_recursive(node_a, node_b, &mut visited)
    }
}

impl Bfs for Graph {
    fn breadth_first_has_path(&self, src: &str, dst: &str, visited: &mut HashSet<String>) -> bool {
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(src.to_string());
        while let Some(current) = queue.pop_front() {
            if current == dst {
                return true;
            }
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.to_string());
            if let Some(neighbors) = self.nodes.get(&current) {
                for neighbor in neighbors {
                    queue.push_back(neighbor.clone());
                }
            }
        }
        false
    }

    fn undirected_path_breadth_first(&self, node_a: &str, node_b: &str) -> bool {
        //Undirected breadth first has-path iterative algo
        //with cyclical checks from edge list input.

        let mut visited: HashSet<String> = HashSet::new();
        self.breadth_first_has_path(node_a, node_b, &mut visited)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Our simple undirected graph structure to play with
    //
    // ┌───┐     ┌───┐
    // │   │     │   │
    // │ i ├─────┤ j │
    // │   │     │   │
    // └─┬─┘     └───┘
    //   │
    //   │
    //   │
    //   │
    //   │
    // ┌─┴─┐     ┌───┐
    // │   │     │   │
    // │ k ├─────┤ l │
    // │   │     │   │
    // └─┬─┘     └───┘
    //   │
    //   │
    //   │
    //   │
    //   │
    // ┌─┴─┐
    // │   │
    // │ m │
    // │   │
    // └───┘
    //
    //
    //
    //
    //
    // ┌───┐     ┌───┐
    // │   │     │   │
    // │ o ├─────┤ n │
    // │   │     │   │
    // └───┘     └───┘
    //

    fn create_graph() -> Graph {
        let mut graph = Graph::new();
        graph.add_edge(&["i", "j"]);
        graph.add_edge(&["k", "i"]);
        graph.add_edge(&["m", "k"]);
        graph.add_edge(&["k", "l"]);
        graph.add_edge(&["o", "n"]);
        graph.build_graph_from_edges();
        graph
    }

    #[test]
    fn depth_first_shortest_path_test() {
        let graph: Graph = create_graph();
        assert!(graph.undirected_path_depth_first("j", "m"));
        assert!(!graph.undirected_path_depth_first("j", "n"));
    }

    #[test]
    fn breadth_shortest_path_test() {
        let graph: Graph = create_graph();
        assert!(graph.undirected_path_breadth_first("j", "m"));
        assert!(!graph.undirected_path_breadth_first("j", "n"));
    }
}
