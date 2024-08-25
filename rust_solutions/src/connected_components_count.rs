use crate::graph::Graph;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
trait Dfs {
    fn depth_first_has_path_recursive(&self, src: &str, visited: &mut HashSet<String>);
    fn connected_components_count(&self) -> usize;
}

#[allow(dead_code)]
trait Bfs {
    fn breadth_first_has_path(&self, src: &str, visited: &mut HashSet<String>) -> bool;
    fn breadth_first_components_count(&self) -> usize;
}

impl Dfs for Graph {
    // Depth first has-path recursive algo with cyclical checks
    fn depth_first_has_path_recursive(&self, src: &str, visited: &mut HashSet<String>) {
        if visited.contains(src) {
            return;
        }
        visited.insert(src.to_string());
        if let Some(neighbors) = self.nodes.get(src) {
            for neighbor in neighbors {
                self.depth_first_has_path_recursive(neighbor, visited);
            }
        }
    }

    fn connected_components_count(&self) -> usize {
        let mut count = 0;
        let mut visited = HashSet::new();

        for node in self.nodes.keys() {
            if !visited.contains(node) {
                self.depth_first_has_path_recursive(node, &mut visited);
                count += 1;
            }
        }

        count
    }
}

impl Bfs for Graph {
    // Breadth first has-path iterative algo with cyclical checks
    fn breadth_first_has_path(&self, src: &str, visited: &mut HashSet<String>) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(src.to_string());

        while let Some(current) = queue.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            if let Some(neighbors) = self.nodes.get(&current) {
                for neighbor in neighbors {
                    queue.push_back(neighbor.clone());
                }
            }
        }
        true
    }

    // Connected components count using BFS
    fn breadth_first_components_count(&self) -> usize {
        let mut count = 0;
        let mut visited = HashSet::new();
        let mut previous_visited = visited.clone();

        for node in self.nodes.keys() {
            self.breadth_first_has_path(node, &mut visited);
            if previous_visited != visited {
                count += 1;
            }
            previous_visited.clone_from(&visited);
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Our 2 component undirected graph structure to play with
    //
    // ┌───┐     ┌───┐     ┌───┐
    // │   │     │   │     │   │
    // │ 1 ├─────┤ 0 ├─────┤ 5 │
    // │   │     │   │     │   │
    // └───┘     └─┬─┘     └─┬─┘
    //             │         │
    //             │         │
    //             │         │
    //             │         │
    //             │         │
    //           ┌─┴─┐       │
    //           │   │       │
    //           │ 8 ├───────┘
    //           │   │
    //           └───┘
    //
    //
    //
    //
    //
    // ┌───┐     ┌───┐
    // │   │     │   │
    // │ 2 ├─────┤ 3 │
    // │   │     │   │
    // └─┬─┘     └─┬─┘
    //   │         │
    //   │         │
    //   │         │
    //   │         │
    //   │         │
    //   │       ┌─┴─┐
    //   │       │   │
    //   └───────┤ 4 │
    //           │   │
    //           └───┘
    //

    fn create_graph() -> Graph {
        let mut graph = Graph::new();
        graph.add_node("0", &["8", "1", "5"]);
        graph.add_node("1", &["0"]);
        graph.add_node("5", &["0", "8"]);
        graph.add_node("8", &["0", "5"]);
        graph.add_node("2", &["3", "4"]);
        graph.add_node("3", &["2", "4"]);
        graph.add_node("4", &["3", "2"]);
        graph
    }

    #[test]
    fn connected_components_count_test() {
        let graph: Graph = create_graph();
        assert!(graph.connected_components_count() == 2);
    }

    #[test]
    fn breadth_first_components_count_test() {
        let graph: Graph = create_graph();
        assert!(graph.breadth_first_components_count() == 2);
    }
}
