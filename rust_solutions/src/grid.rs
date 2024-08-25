use std::cmp::Ordering;

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
                self.grid.push(row.iter().map(|&s| s.to_string()).collect());
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_grid() -> Grid {
        let mut grid = Grid::new(2);
        assert!(grid.add_row(&["8", "1"]).is_ok());
        assert!(grid.add_row(&["0", "1"]).is_ok());
        grid
    }

    #[test]
    fn grid_test() {
        let grid = create_grid();
        assert!(Some(grid).is_some())
    }

    #[test]
    #[should_panic(expected = "Row length 3 exceeds grid width 2")]
    fn grid_test_invalid_row_length() {
        let mut grid = Grid::new(2);
        grid.add_row(&["1", "2", "3"]).unwrap();
    }
}
