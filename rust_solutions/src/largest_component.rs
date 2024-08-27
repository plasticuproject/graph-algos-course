use crate::graph::Graph;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
trait Dfs {
    fn depth_first_count_recursive(&self, src: &str, visited: &mut HashSet<String>) -> i32;
    fn depth_first_largest_component(&self) -> i32;
}

#[allow(dead_code)]
trait Bfs {
    fn breadth_first_count(&self, src: &str, visited: &mut HashSet<String>) -> i32;
    fn breadth_first_largest_component(&self) -> i32;
}

impl Dfs for Graph {
    fn depth_first_count_recursive(&self, src: &str, visited: &mut HashSet<String>) -> i32 {
        if visited.contains(src) {
            return 0;
        }
        let mut size = 1;
        visited.insert(src.to_string());
        if let Some(neighbors) = self.nodes.get(src) {
            for neighbor in neighbors {
                size += self.depth_first_count_recursive(neighbor, visited);
            }
        }
        size
    }

    fn depth_first_largest_component(&self) -> i32 {
        //Takes in a graph adjacency list, traverses the graph
        //depth-first and returns the number of nodes in the
        //largest connected component.

        let mut largest = 0;
        let mut visited = HashSet::new();
        for node in self.nodes.keys() {
            if !visited.contains(node) {
                let size = self.depth_first_count_recursive(node, &mut visited);
                if size > largest {
                    largest = size;
                }
            }
        }
        largest
    }
}

impl Bfs for Graph {
    fn breadth_first_count(&self, src: &str, visited: &mut HashSet<String>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(src.to_string());
        let mut size = 0;
        while let Some(current) = queue.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            if let Some(neighbors) = self.nodes.get(&current) {
                for neighbor in neighbors {
                    queue.push_back(neighbor.clone());
                }
                size += 1;
            }
        }
        size
    }

    // Connected components count using BFS
    fn breadth_first_largest_component(&self) -> i32 {
        //Takes in a graph adjacency list, traverses the graph
        //breadth-first and returns the number of nodes in the
        //largest connected component.

        let mut largest = 0;
        let mut visited = HashSet::new();
        for node in self.nodes.keys() {
            let size = self.breadth_first_count(node, &mut visited);
            if size > largest {
                largest = size;
            }
        }
        largest
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
    fn depth_first_largest_component_test() {
        let graph: Graph = create_graph();
        assert!(graph.depth_first_largest_component() == 4);
    }

    #[test]
    fn breadth_first_largest_component_test() {
        let graph: Graph = create_graph();
        assert!(graph.breadth_first_largest_component() == 4);
    }
}
