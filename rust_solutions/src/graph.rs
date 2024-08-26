use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Graph {
    pub nodes: HashMap<String, Vec<String>>,
    pub edges: Vec<Vec<String>>,
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
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: &str, connections: &[&str]) {
        self.nodes.insert(
            node.to_string(),
            connections.iter().map(|&s| s.to_string()).collect(),
        );
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn add_edge(&mut self, edge: &[&str]) -> Result<(), String> {
        match edge.len().cmp(&2) {
            Ordering::Greater => Err(format!("Edge length {} exceeds maximum of 2", edge.len(),)),
            Ordering::Less => Err(format!(
                "Edge length {} is less than minimum of 2",
                edge.len(),
            )),
            Ordering::Equal => {
                self.edges
                    .push(edge.iter().map(|&s| s.to_string()).collect());
                Ok(())
            }
        }
    }

    pub fn build_graph_from_edges(&mut self) {
        self.nodes.clear();
        for edge in self.edges.clone() {
            let a = edge[0].to_string();
            let b = edge[1].to_string();

            // Add edge a -> b
            self.nodes
                .entry(a.to_string())
                .or_default()
                .push(b.to_string());

            // Add edge b -> a
            self.nodes
                .entry(b.to_string())
                .or_default()
                .push(a.to_string());
        }
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

    fn create_graph_with_edges() -> Graph {
        let mut graph = Graph::new();
        let _ = graph.add_edge(&["w", "x"]).is_ok();
        let _ = graph.add_edge(&["x", "y"]).is_ok();
        let _ = graph.add_edge(&["z", "y"]).is_ok();
        let _ = graph.add_edge(&["z", "v"]).is_ok();
        let _ = graph.add_edge(&["w", "v"]).is_ok();
        graph.build_graph_from_edges();
        graph
    }

    fn sort_hashmap_values(
        mut hashmap: HashMap<String, Vec<String>>,
    ) -> HashMap<String, Vec<String>> {
        for value in hashmap.values_mut() {
            value.sort();
        }
        hashmap
    }

    #[test]
    fn graph_test() {
        let graph = create_graph();
        assert!(Some(graph).is_some())
    }

    #[test]
    fn graph_with_edges_test() {
        let graph_with_edges = create_graph_with_edges();
        assert!(Some(graph_with_edges.clone()).is_some());

        // Expected HashMap
        let mut expected_nodes = HashMap::new();
        expected_nodes.insert("w".to_string(), vec!["v".to_string(), "x".to_string()]);
        expected_nodes.insert("x".to_string(), vec!["w".to_string(), "y".to_string()]);
        expected_nodes.insert("y".to_string(), vec!["x".to_string(), "z".to_string()]);
        expected_nodes.insert("z".to_string(), vec!["y".to_string(), "v".to_string()]);
        expected_nodes.insert("v".to_string(), vec!["z".to_string(), "w".to_string()]);

        // Sort the vectors in both HashMaps
        let sorted_graph_nodes = sort_hashmap_values(graph_with_edges.nodes);
        let sorted_expected_nodes = sort_hashmap_values(expected_nodes);
        assert_eq!(sorted_graph_nodes, sorted_expected_nodes);
    }

    #[test]
    #[should_panic(expected = "Edge length 3 exceeds maximum of 2")]
    fn edges_test_invalid_row_length() {
        let mut graph = Graph::new();
        graph.add_edge(&["x", "y", "z"]).unwrap();
    }
}
