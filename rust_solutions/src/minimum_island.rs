use crate::grid::Grid;
use std::collections::HashSet;

#[allow(dead_code)]
trait Count {
    fn explore(&self, row: usize, col: usize, visited: &mut HashSet<(usize, usize)>) -> usize;
    fn minimum_island_count(&self) -> usize;
}

impl Count for Grid {
    fn explore(&self, row: usize, col: usize, visited: &mut HashSet<(usize, usize)>) -> usize {
        //Depth first has-path recursive algo with cyclical checks.
        //Explores adjacent nodes that are marked as "L" and are not
        //yet visited, tallying the number of connected nodes.

        let row_in_bounds: bool = row < self.grid.len();
        let col_in_bounds: bool = col < self.width;
        if !row_in_bounds || !col_in_bounds {
            return 0;
        }
        let position = (row, col);
        if visited.contains(&position) {
            return 0;
        }
        visited.insert(position);
        if self.grid[row][col] == "W" {
            return 0;
        }
        let mut node_count = 1;
        if row > 0 {
            node_count += self.explore(row - 1, col, visited);
        }
        if row < self.grid.len() - 1 {
            node_count += self.explore(row + 1, col, visited);
        }
        if col > 0 {
            node_count += self.explore(row, col - 1, visited);
        }
        if col < self.width - 1 {
            node_count += self.explore(row, col + 1, visited);
        }
        node_count
    }

    fn minimum_island_count(&self) -> usize {
        //Takes in a 2D grid array, iterates over the grid, traverses
        //the internal components marked with "L" depth-first and returns
        //the number of nodes in the smallest connected component within
        //the 2D array.

        let mut count = HashSet::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        for (i, row) in self.grid.iter().enumerate() {
            for c in 0..row.len() {
                let connected_nodes = self.explore(i, c, &mut visited);
                if connected_nodes > 0 {
                    count.insert(connected_nodes);
                }
            }
        }
        count.iter().min().map_or(0, |min_value| *min_value)
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

        // Test exploring a land tile (should return 2)
        assert_eq!(grid.explore(0, 1, &mut visited), 2);
        assert!(visited.contains(&(0, 1)));

        // Test exploring a water tile (should return 0)
        assert_eq!(grid.explore(0, 0, &mut visited), 0);
        assert!(visited.contains(&(0, 0)));

        // Test exploring out of bounds (should return 0)
        assert_eq!(grid.explore(grid.grid.len(), 0, &mut visited), 0);
        assert_eq!(grid.explore(0, grid.width, &mut visited), 0);
    }

    #[test]
    fn minimum_island_count_test() {
        let grid = create_grid();
        assert!(grid.minimum_island_count() == 2);
    }
}
