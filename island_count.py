"""island_count.py"""

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


def _explore(grid: list, row: int, column: int, visited: set) -> bool:
    """Depth first has-path recursive algo with cyclical checks.
    Explores adjacent nodes that are marked as "L" and are not
    yet visited."""
    position: str = f"{row},{column}"
    if position in visited:
        return False
    visited.add(position)
    try:
        if grid[row][column] == "W":
            return False
        _explore(grid, row - 1, column, visited)
        _explore(grid, row + 1, column, visited)
        _explore(grid, row, column - 1, visited)
        _explore(grid, row, column + 1, visited)
    except IndexError:
        return False
    return True


def island_count(grid: list) -> int:
    """Takes in a 2D grid array, iterates over the grid, traverses
    the internal components marked with "L" depth-first and returns
    the number of connected components within the 2D array."""
    count: int = 0
    visited: set = set()
    for _r, row in enumerate(grid):
        for _c in range(len(row)):
            if _explore(grid, _r, _c, visited):
                count += 1
    return count


assert island_count(GRID) == 3
