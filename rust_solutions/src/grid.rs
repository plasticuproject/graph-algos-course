pub struct Grid {
    pub width: usize,
    pub grid: Vec<Vec<String>>,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Grid {
    #[must_use]
    pub const fn new(width: usize) -> Self {
        Self {
            width,
            grid: Vec::new(),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn add_row(&mut self, row: &[&str]) {
        assert!(
            row.len() == self.width,
            "Row length {} does not match grid width {}",
            row.len(),
            self.width,
        );
        self.grid.push(row.iter().map(|&s| s.to_string()).collect());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_grid() -> Grid {
        let mut grid = Grid::new(2);
        grid.add_row(&["8", "1"]);
        grid.add_row(&["0", "1"]);
        grid
    }

    #[test]
    fn grid_test() {
        let grid = create_grid();
        assert!(Some(grid).is_some())
    }

    #[test]
    #[should_panic(expected = "Row length 3 does not match grid width 2")]
    fn grid_test_invalid_row_length() {
        let mut grid = Grid::new(2);
        grid.add_row(&["1", "2", "3"]);
    }
}
