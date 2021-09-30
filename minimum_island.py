"""minimum_island.py"""

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
GRID = [
    ["W", "L", "W", "W", "W"],
    ["W", "L", "W", "W", "W"],
    ["W", "W", "W", "L", "W"],
    ["W", "W", "L", "L", "W"],
    ["L", "W", "W", "L", "L"],
    ["L", "L", "W", "W", "W"],
]


def _explore(grid: list, row: int, column: int, visited: set) -> int:
    """Depth first has-path recursive algo with cyclical checks.
    Explores adjacent nodes that are marked as "L" and are not
    yet visited, tallying the number of connected nodes."""
    row_in_bounds: bool = 0 <= row < len(grid)
    column_in_bounds: bool = 0 <= column < len(grid[0])
    if not row_in_bounds or not column_in_bounds:
        return 0
    position: str = f"{row},{column}"
    if position in visited:
        return 0
    visited.add(position)
    if grid[row][column] == "W":
        return 0
    node_count: int = 1
    node_count += _explore(grid, row - 1, column, visited)
    node_count += _explore(grid, row + 1, column, visited)
    node_count += _explore(grid, row, column - 1, visited)
    node_count += _explore(grid, row, column + 1, visited)
    return node_count


def minimum_island_count(grid: list) -> int:
    """Takes in a 2D grid array, iterates over the grid, traverses
    the internal components marked with "L" depth-first and returns
    the number of nodes in the smallest connected component within
    the 2D array."""
    count: list = []
    visited: set = set()
    for _r, row in enumerate(grid):
        for _c in range(len(row)):
            connected_nodes = _explore(grid, _r, _c, visited)
            if connected_nodes > 0:
                count.append(connected_nodes)
    return min(count)


assert minimum_island_count(GRID) == 2
