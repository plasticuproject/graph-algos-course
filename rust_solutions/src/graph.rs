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
