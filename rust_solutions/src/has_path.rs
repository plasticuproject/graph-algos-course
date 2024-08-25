use crate::graph::Graph;
use std::collections::VecDeque;

#[allow(dead_code)]
trait Dfs {
    fn depth_first_has_path(&self, src: &str, dst: &str) -> bool;
}

#[allow(dead_code)]
trait Bfs {
    fn breadth_first_has_path(&self, src: &str, dst: &str) -> bool;
}

impl Dfs for Graph {
    fn depth_first_has_path(&self, src: &str, dst: &str) -> bool {
        if src == dst {
            return true;
        }
        if let Some(neighbors) = self.nodes.get(src) {
            for neighbor in neighbors {
                if Some(self.depth_first_has_path(neighbor, dst)) == Some(true) {
                    return true;
                }
            }
        }
        false
    }
}

impl Bfs for Graph {
    fn breadth_first_has_path(&self, src: &str, dst: &str) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(src.to_string());
        while let Some(current) = queue.pop_front() {
            if current == dst {
                return true;
            }

            if let Some(neighbors) = self.nodes.get(&current) {
                for neighbor in neighbors {
                    queue.push_back(neighbor.clone());
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Our simple acyclic graph structure to play with
    //
    // ┌───┐     ┌───┐     ┌───┐
    // │   │     │   │     │   │
    // │ f ├────►│ g ├────►│ h │
    // │   │     │   │     │   │
    // └─┬─┘     └───┘     └───┘
    //   │         ▲
    //   │         │
    //   │     ┌───┘
    //   │     │
    //   ▼     │
    // ┌───┐   │ ┌───┐
    // │   ├───┘ │   │
    // │ i │     │ j │
    // │   │◄────┤   │
    // └─┬─┘     └───┘
    //   │
    //   │
    //   │
    //   │
    //   ▼
    // ┌───┐
    // │   │
    // │ k │
    // │   │
    // └───┘
    //

    fn create_graph() -> Graph {
        let mut graph = Graph::new();
        graph.add_node("f", &["g", "i"]);
        graph.add_node("g", &["h"]);
        graph.add_node("h", &[]);
        graph.add_node("i", &["g", "k"]);
        graph.add_node("j", &["i"]);
        graph.add_node("k", &[]);
        graph
    }

    #[test]
    fn depth_first_has_path_test() {
        let graph: Graph = create_graph();
        assert!(graph.depth_first_has_path("f", "k"));
        assert!(graph.depth_first_has_path("j", "f") == false);
    }

    #[test]
    fn breadth_first_has_path_test() {
        let graph: Graph = create_graph();
        assert!(graph.breadth_first_has_path("f", "k"));
        assert!(graph.breadth_first_has_path("j", "f") == false);
    }
}
