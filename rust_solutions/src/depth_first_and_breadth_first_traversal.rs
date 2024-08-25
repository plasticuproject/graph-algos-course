use crate::graph::Graph;
use std::collections::VecDeque;

#[allow(dead_code)]
trait Dfs {
    fn depth_first_print_iterative(&self, src: &str) -> Vec<String>;
    fn depth_first_print_recursive(&self, src: &str, r_list: &mut Vec<String>) -> Vec<String>;
}

#[allow(dead_code)]
trait Bfs {
    fn breadth_first_print_iterative(&self, src: &str) -> Vec<String>;
}

impl Dfs for Graph {
    fn depth_first_print_iterative(&self, src: &str) -> Vec<String> {
        let mut stack = VecDeque::new();
        stack.push_back(src.to_string());
        let mut r_list = Vec::new();

        while let Some(current) = stack.pop_back() {
            println!("{current:?}");
            r_list.push(current.clone());
            if let Some(neighbors) = self.nodes.get(&current) {
                for neighbor in neighbors {
                    stack.push_back(neighbor.clone());
                }
            }
        }
        r_list
    }

    fn depth_first_print_recursive(&self, src: &str, r_list: &mut Vec<String>) -> Vec<String> {
        println!("{src:?}");
        r_list.push(src.to_string());
        if let Some(neighbors) = self.nodes.get(src) {
            for neighbor in neighbors {
                self.depth_first_print_recursive(neighbor, r_list);
            }
        }
        r_list.clone()
    }
}

impl Bfs for Graph {
    fn breadth_first_print_iterative(&self, src: &str) -> Vec<String> {
        let mut queue = VecDeque::new();
        queue.push_back(src.to_string());
        let mut r_list = Vec::new();
        while let Some(current) = queue.pop_front() {
            println!("{current:?}");
            r_list.push(current.clone());

            if let Some(neighbors) = self.nodes.get(&current) {
                for neighbor in neighbors {
                    queue.push_back(neighbor.clone());
                }
            }
        }
        r_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // # Our simple graph structure to play with
    //
    // ┌───┐     ┌───┐
    // │   │     │   │
    // │ a ├────►│ b │
    // │   │     │   │
    // └─┬─┘     └─┬─┘
    //   │         │
    //   │         │
    //   │         │
    //   │         │
    //   ▼         ▼
    // ┌───┐     ┌───┐
    // │   │     │   │
    // │ c │     │ d │
    // │   │     │   │
    // └─┬─┘     └─┬─┘
    //   │         │
    //   │         │
    //   │         │
    //   │         │
    //   ▼         ▼
    // ┌───┐     ┌───┐
    // │   │     │   │
    // │ e │     │ f │
    // │   │     │   │
    // └───┘     └───┘
    //

    fn create_graph() -> Graph {
        let mut graph = Graph::new();
        graph.add_node("a", &["c", "b"]);
        graph.add_node("b", &["d"]);
        graph.add_node("c", &["e"]);
        graph.add_node("d", &["f"]);
        graph.add_node("e", &[]);
        graph.add_node("f", &[]);
        graph
    }

    #[test]
    fn depth_first_print_iterative_test() {
        let graph: Graph = create_graph();
        assert!(graph.depth_first_print_iterative("a") == ["a", "b", "d", "f", "c", "e"]);
    }

    #[test]
    fn depth_first_print_recursive_test() {
        let graph: Graph = create_graph();
        let mut r_list: Vec<String> = Vec::new();
        assert!(
            graph.depth_first_print_recursive("a", &mut r_list) == ["a", "c", "e", "b", "d", "f"]
        );
    }

    #[test]
    fn breadth_first_print_iterative_test() {
        let graph: Graph = create_graph();
        assert!(graph.breadth_first_print_iterative("a") == ["a", "c", "b", "e", "d", "f"]);
    }
}
