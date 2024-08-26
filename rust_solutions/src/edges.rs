use std::cmp::Ordering;

pub struct Edges {
    pub width: usize,
    pub edges: Vec<Vec<String>>,
}

impl Default for Edges {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Edges {
    #[must_use]
    pub const fn new(width: usize) -> Self {
        Self {
            width,
            edges: Vec::new(),
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn add_row(&mut self, row: &[&str]) -> Result<(), String> {
        match row.len().cmp(&self.width) {
            Ordering::Greater => Err(format!(
                "Row length {} exceeds grid width {}",
                row.len(),
                self.width
            )),
            Ordering::Less => Err(format!(
                "Row length {} is less than grid width {}",
                row.len(),
                self.width
            )),
            Ordering::Equal => {
                self.edges
                    .push(row.iter().map(|&s| s.to_string()).collect());
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_edges() -> Edges {
        let mut edges = Edges::new(2);
        assert!(edges.add_row(&["w", "x"]).is_ok());
        assert!(edges.add_row(&["x", "y"]).is_ok());
        edges
    }

    #[test]
    fn edges_test() {
        let edges = create_edges();
        assert!(Some(edges).is_some())
    }

    #[test]
    #[should_panic(expected = "Row length 3 exceeds grid width 2")]
    fn edges_test_invalid_row_length() {
        let mut edges = Edges::new(2);
        edges.add_row(&["x", "y", "z"]).unwrap();
    }
}
