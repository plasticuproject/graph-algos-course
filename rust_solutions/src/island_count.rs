use crate::grid::Grid;
use std::collections::HashSet;

#[allow(dead_code)]
trait Count {
    fn explore(&self, row: usize, col: usize, visited: &mut HashSet<(usize, usize)>) -> bool;
    fn island_count(&self) -> usize;
}

impl Count for Grid {
    fn explore(&self, row: usize, col: usize, visited: &mut HashSet<(usize, usize)>) -> bool {
        //Depth first has-path recursive algo with cyclical checks.
        //Explores adjacent nodes that are marked as "L" and are not
        //yet visited.

        let row_in_bounds: bool = row < self.grid.len();
        let col_in_bounds: bool = col < self.width;
        if !row_in_bounds || !col_in_bounds {
            return false;
        }
        let position = (row, col);
        if visited.contains(&position) {
            return false;
        }
        visited.insert(position);
        if self.grid[row][col] == "W" {
            return false;
        }
        if row > 0 {
            self.explore(row - 1, col, visited);
        }
        if row < self.grid.len() - 1 {
            self.explore(row + 1, col, visited);
        }
        if col > 0 {
            self.explore(row, col - 1, visited);
        }
        if col < self.width - 1 {
            self.explore(row, col + 1, visited);
        }
        true
    }

    fn island_count(&self) -> usize {
        //Takes in a 2D grid array, iterates over the grid, traverses
        //the internal components marked with "L" depth-first and returns
        //the number of connected components within the 2D array.

        let mut count: usize = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        for (i, row) in self.grid.iter().enumerate() {
            for c in 0..row.len() {
                if self.explore(i, c, &mut visited) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // How to navigate grid
    //
    //            (r-1,c)
    //               ^
    //               |
    // (r,c-1) <-- (r,c) --> (r,c+1)
    //               |
    //               v
    //            (r+1,c)

    fn create_grid() -> Grid {
        let mut grid = Grid::new(5);
        grid.add_row(&["W", "L", "W", "W", "W"]);
        grid.add_row(&["W", "L", "W", "W", "W"]);
        grid.add_row(&["W", "W", "W", "L", "W"]);
        grid.add_row(&["W", "W", "L", "L", "W"]);
        grid.add_row(&["L", "W", "W", "L", "L"]);
        grid.add_row(&["L", "L", "W", "W", "W"]);
        grid
    }

    #[test]
    fn explore_test() {
        let grid = create_grid();
        let mut visited = HashSet::new();

        // Test exploring a land tile (should return true)
        assert_eq!(grid.explore(0, 1, &mut visited), true);
        assert!(visited.contains(&(0, 1)));

        // Test exploring a water tile (should return false)
        assert_eq!(grid.explore(0, 0, &mut visited), false);
        assert!(visited.contains(&(0, 0)));

        // Test exploring out of bounds (should return false)
        assert_eq!(grid.explore(grid.grid.len(), 0, &mut visited), false);
        assert_eq!(grid.explore(0, grid.width, &mut visited), false);
    }

    #[test]
    fn island_count_test() {
        let grid = create_grid();
        assert!(grid.island_count() == 3);
    }
}
