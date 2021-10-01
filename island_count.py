"""island_count.py"""
from typing import List, Set, Tuple

# How to navigate grid
"""
           (r-1,c)
              ^
              |
(r,c-1) <-- (r,c) --> (r,c+1)
              |
              v
           (r+1,c)
"""

# Our graph grid structure
GRID: List[List[str]] = [
    ["W", "L", "W", "W", "W"],
    ["W", "L", "W", "W", "W"],
    ["W", "W", "W", "L", "W"],
    ["W", "W", "L", "L", "W"],
    ["L", "W", "W", "L", "L"],
    ["L", "L", "W", "W", "W"],
]


def _explore(grid: List[List[str]], row: int, column: int,
             visited: Set[Tuple[int]]) -> bool:
    """Depth first has-path recursive algo with cyclical checks.
    Explores adjacent nodes that are marked as "L" and are not
    yet visited."""
    row_in_bounds: bool = 0 <= row < len(grid)
    column_in_bounds: bool = 0 <= column < len(grid[0])
    if not row_in_bounds or not column_in_bounds:
        return False
    position: Tuple(int) = (row, column)
    if position in visited:
        return False
    visited.add(position)
    if grid[row][column] == "W":
        return False
    _explore(grid, row - 1, column, visited)
    _explore(grid, row + 1, column, visited)
    _explore(grid, row, column - 1, visited)
    _explore(grid, row, column + 1, visited)
    return True


def island_count(grid: List[List[str]]) -> int:
    """Takes in a 2D grid array, iterates over the grid, traverses
    the internal components marked with "L" depth-first and returns
    the number of connected components within the 2D array."""
    count: int = 0
    visited: Set[Tuple[int]] = set()
    for _r, row in enumerate(grid):
        for _c in range(len(row)):
            if _explore(grid, _r, _c, visited):
                count += 1
    return count


assert island_count(GRID) == 3
