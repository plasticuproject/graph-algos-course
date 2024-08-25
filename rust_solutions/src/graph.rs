use std::collections::HashMap;

pub struct Graph {
    pub nodes: HashMap<String, Vec<String>>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: &str, connections: &[&str]) {
        self.nodes.insert(
            node.to_string(),
            connections.iter().map(|&s| s.to_string()).collect(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn graph_test() {
        let graph = create_graph();
        assert!(Some(graph).is_some())
    }
}
